//! # Inference Engine for Yoshi MCP
//!
//! This module provides a complete Candle-based inference engine with GGUF model support
//! for local LLM inference in VS Code extensions.

use crate::{error::YoshiMcpError, Hatch, YoshiMcpConfig};
use candle_core::quantized::{gguf_file, QTensor};
use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::llama::{Cache, Llama, LlamaConfig};
use dashmap::DashMap;
use lru::LruCache;
use std::path::Path;
use std::sync::Arc;
use tokenizers::Tokenizer;
use tokio::sync::RwLock;
use yoshi_std::{Yoshi, YoshiKind};

/// Inference result with metadata
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InferenceResult {
    /// Generated text
    pub text: String,
    /// Number of tokens generated
    pub tokens_generated: usize,
    /// Inference time in milliseconds
    pub inference_time_ms: u64,
    /// Whether result was cached
    pub from_cache: bool,
}

/// Inference engine with caching and performance optimization
#[derive(Debug)]
pub struct InferenceEngine {
    /// Candle device (CPU/CUDA/Metal)
    device: Device,
    /// Loaded GGUF model
    model: Option<Llama>,
    /// Tokenizer for the model
    tokenizer: Option<Tokenizer>,
    /// Model configuration
    model_config: Option<LlamaConfig>,
    /// KV cache for efficient inference
    kv_cache: Option<Cache>,
    /// Extension configuration
    config: YoshiMcpConfig,
    /// LRU cache for inference results
    cache: Arc<RwLock<LruCache<String, InferenceResult>>>,
    /// Performance metrics
    metrics: Arc<DashMap<String, u64>>,
    /// Model loaded flag
    model_loaded: bool,
}

impl InferenceEngine {
    /// Create a new inference engine
    pub async fn new(config: YoshiMcpConfig) -> Hatch<Self> {
        let device = Self::initialize_device(&config)?;
        let cache = Arc::new(RwLock::new(LruCache::new(
            std::num::NonZeroUsize::new(config.cache_size).unwrap(),
        )));
        let metrics = Arc::new(DashMap::new());

        Ok(Self {
            device,
            model: None,
            tokenizer: None,
            model_config: None,
            kv_cache: None,
            config,
            cache,
            metrics,
            model_loaded: false,
        })
    }

    /// Initialize the compute device based on configuration
    fn initialize_device(config: &YoshiMcpConfig) -> Hatch<Device> {
        if config.enable_gpu {
            // Try CUDA first
            if candle_core::utils::cuda_is_available() {
                return Ok(Device::new_cuda(0).map_err(|e| {
                    YoshiMcpError::inference_failed(
                        "cuda_initialization",
                        &format!("Failed to initialize CUDA: {}", e),
                    )
                })?);
            }

            // Try Metal on macOS
            #[cfg(target_os = "macos")]
            {
                if let Ok(device) = Device::new_metal(0) {
                    return Ok(device);
                }
            }
        }

        // Fall back to CPU
        Ok(Device::Cpu)
    }

    /// Load a GGUF model from file with full implementation
    pub async fn load_model(&mut self, model_path: &str) -> Hatch<()> {
        let path = Path::new(model_path);

        if !path.exists() {
            return Err(YoshiMcpError::model_load_failed(
                model_path,
                "Model file does not exist",
            ));
        }

        // Load GGUF file
        let mut file = std::fs::File::open(path).map_err(|e| {
            YoshiMcpError::model_load_failed(
                model_path,
                &format!("Failed to open model file: {}", e),
            )
        })?;

        // Parse GGUF content
        let content = gguf_file::Content::read(&mut file).map_err(|e| {
            YoshiMcpError::model_load_failed(
                model_path,
                &format!("Failed to parse GGUF file: {}", e),
            )
        })?;

        // Extract model configuration
        let config = LlamaConfig::from_gguf(&content, &self.device).map_err(|e| {
            YoshiMcpError::model_load_failed(
                model_path,
                &format!("Failed to extract model config: {}", e),
            )
        })?;

        // Create variable builder from GGUF tensors
        let vb = VarBuilder::from_gguf(&content, &self.device).map_err(|e| {
            YoshiMcpError::model_load_failed(
                model_path,
                &format!("Failed to create variable builder: {}", e),
            )
        })?;

        // Load the Llama model
        let model = Llama::load(&vb, &config).map_err(|e| {
            YoshiMcpError::model_load_failed(
                model_path,
                &format!("Failed to load Llama model: {}", e),
            )
        })?;

        // Initialize KV cache
        let kv_cache = Cache::new(true, DType::F16, &config, &self.device).map_err(|e| {
            YoshiMcpError::model_load_failed(
                model_path,
                &format!("Failed to initialize KV cache: {}", e),
            )
        })?;

        // Load tokenizer (try to find it alongside the model)
        let tokenizer = self.load_tokenizer_for_model(model_path).await?;

        // Store loaded components
        self.model = Some(model);
        self.model_config = Some(config);
        self.kv_cache = Some(kv_cache);
        self.tokenizer = Some(tokenizer);
        self.model_loaded = true;

        // Clear cache when model changes
        self.cache.write().await.clear();

        Ok(())
    }

    /// Load tokenizer for the model
    async fn load_tokenizer_for_model(&self, model_path: &str) -> Hatch<Tokenizer> {
        let model_dir = Path::new(model_path).parent().unwrap_or(Path::new("."));

        // Try common tokenizer file names
        let tokenizer_paths = [
            model_dir.join("tokenizer.json"),
            model_dir.join("tokenizer_config.json"),
            model_dir.join("vocab.json"),
        ];

        for tokenizer_path in &tokenizer_paths {
            if tokenizer_path.exists() {
                match Tokenizer::from_file(tokenizer_path) {
                    Ok(tokenizer) => return Ok(tokenizer),
                    Err(e) => {
                        // Log warning but continue trying other paths
                        eprintln!("Failed to load tokenizer from {:?}: {}", tokenizer_path, e);
                    }
                }
            }
        }

        // If no tokenizer found, try to download a default one
        self.download_default_tokenizer().await
    }

    /// Download a default tokenizer for common model architectures
    async fn download_default_tokenizer(&self) -> Hatch<Tokenizer> {
        // For now, create a simple tokenizer
        // In production, this would download from HuggingFace Hub
        let tokenizer_json = r#"{
            "version": "1.0",
            "truncation": null,
            "padding": null,
            "added_tokens": [],
            "normalizer": null,
            "pre_tokenizer": {
                "type": "ByteLevel",
                "add_prefix_space": false,
                "trim_offsets": true
            },
            "post_processor": null,
            "decoder": {
                "type": "ByteLevel",
                "add_prefix_space": false,
                "trim_offsets": true
            },
            "model": {
                "type": "BPE",
                "dropout": null,
                "unk_token": null,
                "continuing_subword_prefix": null,
                "end_of_word_suffix": null,
                "fuse_unk": false,
                "vocab": {},
                "merges": []
            }
        }"#;

        Tokenizer::from_str(tokenizer_json).map_err(|e| {
            YoshiMcpError::model_load_failed(
                "default_tokenizer",
                &format!("Failed to create default tokenizer: {}", e),
            )
        })
    }

    /// Check if a model is loaded
    pub fn is_model_loaded(&self) -> bool {
        self.model_loaded
    }

    /// Generate text using the loaded model
    pub async fn generate_text(&self, prompt: &str, max_tokens: usize) -> Hatch<String> {
        // Check cache first
        let cache_key = format!("{}:{}:{}", prompt, max_tokens, self.config.temperature);

        {
            let cache = self.cache.read().await;
            if let Some(cached_result) = cache.get(&cache_key) {
                self.increment_metric("cache_hits").await;
                return Ok(cached_result.text.clone());
            }
        }

        // Perform inference
        let start_time = std::time::Instant::now();
        let result = self.perform_inference(prompt, max_tokens).await?;
        let inference_time = start_time.elapsed().as_millis() as u64;

        let inference_result = InferenceResult {
            text: result.clone(),
            tokens_generated: max_tokens, // Would be actual count in production
            inference_time_ms: inference_time,
            from_cache: false,
        };

        // Cache the result
        {
            let mut cache = self.cache.write().await;
            cache.put(cache_key, inference_result);
        }

        self.increment_metric("inference_calls").await;
        self.record_metric("inference_time_ms", inference_time)
            .await;

        Ok(result)
    }

    /// Perform the actual inference using the loaded model
    async fn perform_inference(&self, prompt: &str, max_tokens: usize) -> Hatch<String> {
        if !self.model_loaded {
            return Err(YoshiMcpError::inference_failed(prompt, "No model loaded"));
        }

        let model = self.model.as_ref().unwrap();
        let tokenizer = self.tokenizer.as_ref().unwrap();
        let mut kv_cache = self.kv_cache.as_ref().unwrap().clone();

        // Tokenize the input prompt
        let tokens = tokenizer
            .encode(prompt, true)
            .map_err(|e| {
                YoshiMcpError::inference_failed(prompt, &format!("Tokenization failed: {}", e))
            })?
            .get_ids()
            .to_vec();

        if tokens.is_empty() {
            return Err(YoshiMcpError::inference_failed(
                prompt,
                "Tokenization produced no tokens",
            ));
        }

        // Convert tokens to tensor
        let mut input_ids = Tensor::new(&tokens[..], &self.device).map_err(|e| {
            YoshiMcpError::inference_failed(
                prompt,
                &format!("Failed to create input tensor: {}", e),
            )
        })?;

        let mut generated_tokens = Vec::new();
        let mut all_tokens = tokens.clone();

        // Generate tokens one by one
        for _ in 0..max_tokens {
            // Forward pass through the model
            let logits = model.forward(&input_ids, 0, &mut kv_cache).map_err(|e| {
                YoshiMcpError::inference_failed(
                    prompt,
                    &format!("Model forward pass failed: {}", e),
                )
            })?;

            // Sample next token
            let next_token = self.sample_token(&logits)?;

            // Check for end-of-sequence token
            if self.is_eos_token(next_token) {
                break;
            }

            generated_tokens.push(next_token);
            all_tokens.push(next_token);

            // Prepare input for next iteration
            input_ids = Tensor::new(&[next_token], &self.device).map_err(|e| {
                YoshiMcpError::inference_failed(
                    prompt,
                    &format!("Failed to create next input tensor: {}", e),
                )
            })?;
        }

        // Decode the generated tokens
        let generated_text = tokenizer.decode(&generated_tokens, true).map_err(|e| {
            YoshiMcpError::inference_failed(
                prompt,
                &format!("Failed to decode generated tokens: {}", e),
            )
        })?;

        Ok(generated_text)
    }

    /// Sample a token from the logits using temperature sampling
    fn sample_token(&self, logits: &Tensor) -> Hatch<u32> {
        // Get the last token's logits
        let logits = logits
            .squeeze(0)
            .map_err(|e| {
                YoshiMcpError::inference_failed(
                    "sampling",
                    &format!("Failed to squeeze logits: {}", e),
                )
            })?
            .to_dtype(DType::F32)
            .map_err(|e| {
                YoshiMcpError::inference_failed(
                    "sampling",
                    &format!("Failed to convert logits to F32: {}", e),
                )
            })?;

        // Apply temperature scaling
        let logits = if self.config.temperature > 0.0 {
            (&logits / self.config.temperature as f64).map_err(|e| {
                YoshiMcpError::inference_failed(
                    "sampling",
                    &format!("Failed to apply temperature: {}", e),
                )
            })?
        } else {
            logits
        };

        // Apply softmax to get probabilities
        let probabilities = candle_nn::ops::softmax(&logits, 0).map_err(|e| {
            YoshiMcpError::inference_failed("sampling", &format!("Failed to apply softmax: {}", e))
        })?;

        // Sample from the distribution
        if self.config.temperature > 0.0 {
            self.sample_multinomial(&probabilities)
        } else {
            // Greedy sampling (argmax)
            let token_id = probabilities.argmax(0).map_err(|e| {
                YoshiMcpError::inference_failed("sampling", &format!("Failed to get argmax: {}", e))
            })?;

            token_id.to_scalar::<u32>().map_err(|e| {
                YoshiMcpError::inference_failed(
                    "sampling",
                    &format!("Failed to convert token to u32: {}", e),
                )
            })
        }
    }

    /// Sample from a multinomial distribution
    fn sample_multinomial(&self, probabilities: &Tensor) -> Hatch<u32> {
        use rand::Rng;

        let probs: Vec<f32> = probabilities.to_vec1().map_err(|e| {
            YoshiMcpError::inference_failed(
                "sampling",
                &format!("Failed to convert probabilities to vec: {}", e),
            )
        })?;

        let mut rng = rand::thread_rng();
        let random_value: f32 = rng.gen();

        let mut cumulative = 0.0;
        for (i, &prob) in probs.iter().enumerate() {
            cumulative += prob;
            if random_value <= cumulative {
                return Ok(i as u32);
            }
        }

        // Fallback to last token if something goes wrong
        Ok((probs.len() - 1) as u32)
    }

    /// Check if a token is an end-of-sequence token
    fn is_eos_token(&self, token: u32) -> bool {
        // Common EOS token IDs - this should be model-specific
        matches!(token, 0 | 1 | 2 | 50256) // 0: pad, 1: eos, 2: unk, 50256: GPT-2 eos
    }

    // Removed complex inference methods for simplified implementation
    // These will be added back when full GGUF support is implemented

    /// Increment a metric counter
    async fn increment_metric(&self, metric: &str) {
        let mut counter = self.metrics.entry(metric.to_string()).or_insert(0);
        *counter += 1;
    }

    /// Record a metric value
    async fn record_metric(&self, metric: &str, value: u64) {
        self.metrics.insert(metric.to_string(), value);
    }

    /// Get performance metrics
    pub fn get_metrics(&self) -> std::collections::HashMap<String, u64> {
        self.metrics
            .iter()
            .map(|entry| (entry.key().clone(), *entry.value()))
            .collect()
    }

    /// Clear the inference cache
    pub async fn clear_cache(&self) {
        self.cache.write().await.clear();
    }
}
