//! # Model Management for Yoshi MCP
//!
//! This module handles GGUF model loading, verification, and management
//! with complete Yoshi framework error handling integration.

use crate::{error::YoshiMcpError, Hatch, YoshiMcpConfig};
use candle_core::quantized::gguf_file;
use candle_core::{DType, Device, Tensor};
use candle_transformers::models::llama::{Llama, LlamaConfig};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use yoshi_std::{Yoshi, YoshiKind};

/// Model information and metadata
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ModelInfo {
    /// Model identifier (e.g., "qwen2.5-7b-instruct")
    pub id: String,
    /// Display name for the model
    pub name: String,
    /// Model file path
    pub path: PathBuf,
    /// Model size in bytes
    pub size: u64,
    /// SHA256 hash for verification
    pub hash: String,
    /// Model architecture (llama, qwen, etc.)
    pub architecture: String,
    /// Parameter count
    pub parameters: String,
    /// Quantization level (q4_0, q8_0, etc.)
    pub quantization: String,
}

/// Model manager for handling GGUF models with Yoshi error handling
#[derive(Debug)]
pub struct ModelManager {
    /// Available models registry
    models: HashMap<String, ModelInfo>,
    /// Models directory path
    models_dir: PathBuf,
    /// Verification hashes for trusted models
    trusted_hashes: HashMap<String, String>,
}

impl ModelManager {
    /// Create a new model manager
    pub fn new(config: &YoshiMcpConfig) -> Hatch<Self> {
        let models_dir = Self::get_models_directory()?;

        // Ensure models directory exists
        std::fs::create_dir_all(&models_dir).map_err(|e| {
            YoshiMcpError::model_load_failed(
                models_dir.to_string_lossy().as_ref(),
                &format!("Failed to create models directory: {}", e),
            )
        })?;

        let mut manager = Self {
            models: HashMap::new(),
            models_dir,
            trusted_hashes: HashMap::new(),
        };

        // Load trusted model hashes
        manager.load_trusted_hashes()?;

        // Discover existing models
        manager.discover_models()?;

        Ok(manager)
    }

    /// Get the models directory path
    fn get_models_directory() -> Hatch<PathBuf> {
        let home_dir = dirs::home_dir().ok_or_else(|| {
            YoshiMcpError::config_invalid("home_directory", "valid home directory", "none")
        })?;

        Ok(home_dir.join(".yoshi-mcp").join("models"))
    }

    /// Load trusted model hashes from configuration
    fn load_trusted_hashes(&mut self) -> Hatch<()> {
        let hashes_file = self.models_dir.join("trusted_hashes.json");

        if hashes_file.exists() {
            let content = std::fs::read_to_string(&hashes_file).map_err(|e| {
                YoshiMcpError::model_load_failed(
                    hashes_file.to_string_lossy().as_ref(),
                    &format!("Failed to read trusted hashes: {}", e),
                )
            })?;

            self.trusted_hashes = serde_json::from_str(&content).map_err(|e| {
                YoshiMcpError::config_invalid(
                    "trusted_hashes",
                    "valid JSON format",
                    &format!("parse error: {}", e),
                )
            })?;
        } else {
            // Create default trusted hashes for popular models
            self.create_default_trusted_hashes()?;
        }

        Ok(())
    }

    /// Create default trusted hashes for popular models
    fn create_default_trusted_hashes(&mut self) -> Hatch<()> {
        // Add known good hashes for popular models
        // These would be updated from a trusted source in production
        self.trusted_hashes.insert(
            "qwen2.5-7b-instruct-q4_0.gguf".to_string(),
            "placeholder_hash_would_be_real_sha256".to_string(),
        );

        // Save to file
        let hashes_file = self.models_dir.join("trusted_hashes.json");
        let content = serde_json::to_string_pretty(&self.trusted_hashes).map_err(|e| {
            YoshiMcpError::config_invalid(
                "trusted_hashes",
                "serializable JSON",
                &format!("serialization error: {}", e),
            )
        })?;

        std::fs::write(&hashes_file, content).map_err(|e| {
            YoshiMcpError::model_load_failed(
                hashes_file.to_string_lossy().as_ref(),
                &format!("Failed to write trusted hashes: {}", e),
            )
        })?;

        Ok(())
    }

    /// Discover existing models in the models directory
    fn discover_models(&mut self) -> Hatch<()> {
        let entries = std::fs::read_dir(&self.models_dir).map_err(|e| {
            YoshiMcpError::model_load_failed(
                self.models_dir.to_string_lossy().as_ref(),
                &format!("Failed to read models directory: {}", e),
            )
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| {
                YoshiMcpError::model_load_failed(
                    "directory_entry",
                    &format!("Failed to read directory entry: {}", e),
                )
            })?;

            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("gguf") {
                if let Ok(model_info) = self.analyze_model_file(&path) {
                    self.models.insert(model_info.id.clone(), model_info);
                }
            }
        }

        Ok(())
    }

    /// Analyze a GGUF model file and extract metadata
    fn analyze_model_file(&self, path: &Path) -> Hatch<ModelInfo> {
        let metadata = std::fs::metadata(path).map_err(|e| {
            YoshiMcpError::model_load_failed(
                path.to_string_lossy().as_ref(),
                &format!("Failed to read file metadata: {}", e),
            )
        })?;

        let size = metadata.len();
        let hash = self.calculate_file_hash(path)?;

        let filename = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");

        // Extract model information from filename
        // This is a simplified parser - in production, you'd parse the GGUF header
        let (name, architecture, quantization) = self.parse_model_filename(filename);

        Ok(ModelInfo {
            id: filename.to_string(),
            name,
            path: path.to_path_buf(),
            size,
            hash,
            architecture,
            parameters: "unknown".to_string(), // Would be extracted from GGUF header
            quantization,
        })
    }

    /// Parse model information from filename
    fn parse_model_filename(&self, filename: &str) -> (String, String, String) {
        // Simple filename parsing - would be more sophisticated in production
        let name = filename.replace(".gguf", "");

        let architecture = if name.contains("qwen") {
            "qwen".to_string()
        } else if name.contains("llama") {
            "llama".to_string()
        } else {
            "unknown".to_string()
        };

        let quantization = if name.contains("q4_0") {
            "q4_0".to_string()
        } else if name.contains("q8_0") {
            "q8_0".to_string()
        } else {
            "unknown".to_string()
        };

        (name, architecture, quantization)
    }

    /// Calculate SHA256 hash of a file
    fn calculate_file_hash(&self, path: &Path) -> Hatch<String> {
        let mut file = std::fs::File::open(path).map_err(|e| {
            YoshiMcpError::model_load_failed(
                path.to_string_lossy().as_ref(),
                &format!("Failed to open file for hashing: {}", e),
            )
        })?;

        let mut hasher = Sha256::new();
        std::io::copy(&mut file, &mut hasher).map_err(|e| {
            YoshiMcpError::model_load_failed(
                path.to_string_lossy().as_ref(),
                &format!("Failed to calculate hash: {}", e),
            )
        })?;

        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Verify model integrity against trusted hashes
    pub fn verify_model(&self, model_id: &str) -> Hatch<bool> {
        let model = self.models.get(model_id).ok_or_else(|| {
            YoshiMcpError::config_invalid("model_id", "existing model ID", model_id)
        })?;

        if let Some(trusted_hash) = self.trusted_hashes.get(model_id) {
            Ok(&model.hash == trusted_hash)
        } else {
            // Model not in trusted list - could be user-provided
            Ok(false)
        }
    }

    /// Get list of available models
    pub fn list_models(&self) -> Vec<&ModelInfo> {
        self.models.values().collect()
    }

    /// Get model information by ID
    pub fn get_model(&self, model_id: &str) -> Option<&ModelInfo> {
        self.models.get(model_id)
    }

    /// Download a model from Hugging Face Hub
    pub async fn download_model(&mut self, model_id: &str, repo_id: &str) -> Hatch<ModelInfo> {
        let filename = format!("{}.gguf", model_id);
        let model_path = self.models_dir.join(&filename);

        // Download using reqwest
        let url = format!(
            "https://huggingface.co/{}/resolve/main/{}",
            repo_id, filename
        );
        let response = reqwest::get(&url).await.map_err(|e| {
            YoshiMcpError::inference_failed(&url, &format!("Failed to download model: {}", e))
        })?;

        if !response.status().is_success() {
            return Err(YoshiMcpError::inference_failed(
                &url,
                &format!("HTTP error: {}", response.status()),
            ));
        }

        let bytes = response.bytes().await.map_err(|e| {
            YoshiMcpError::inference_failed(&url, &format!("Failed to read response: {}", e))
        })?;

        std::fs::write(&model_path, bytes).map_err(|e| {
            YoshiMcpError::model_load_failed(
                model_path.to_string_lossy().as_ref(),
                &format!("Failed to write model file: {}", e),
            )
        })?;

        // Analyze the downloaded model
        let model_info = self.analyze_model_file(&model_path)?;
        self.models
            .insert(model_info.id.clone(), model_info.clone());

        Ok(model_info)
    }
}
