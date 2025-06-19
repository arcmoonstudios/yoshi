/* yoshi-analyzer/src/ml/features.rs */
#![warn(missing_docs)]
//! **Brief:** Feature Extraction and Preprocessing Pipeline for Yoshi ML Analysis.
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + [Feature Engineering]
//!  - [AST-based structural features with tree-sitter integration]
//!  - [Lexical and syntactic features from source code]
//!  - [Semantic embeddings from transformer models]
//! + [Preprocessing Pipeline]
//!  - [Code normalization and tokenization]
//!  - [Feature scaling and dimensionality reduction]
//!  - [Multi-modal feature fusion for enhanced analysis]
//! + [Performance Optimization]
//!  - [Incremental feature computation for large codebases]
//!  - [Feature caching with intelligent invalidation]
//!  - [Parallel processing for batch feature extraction]
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **Author:** Lord Xyn
// **License:** MIT OR Apache-2.0

use super::{CachedFeatures, MLResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use yoshi_core::Yoshi;

/// Feature extraction engine for code analysis
pub struct FeatureExtractor {
    /// Configuration for feature extraction
    config: FeatureConfig,
    /// Cache for computed features
    feature_cache: HashMap<String, CachedFeatures>,
}

/// Configuration for feature extraction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureConfig {
    /// Enable AST-based features
    pub enable_ast_features: bool,
    /// Enable lexical features
    pub enable_lexical_features: bool,
    /// Enable semantic features
    pub enable_semantic_features: bool,
    /// Maximum code length to process
    pub max_code_length: usize,
    /// Feature vector dimension
    pub feature_dimension: usize,
}

impl Default for FeatureConfig {
    fn default() -> Self {
        Self {
            enable_ast_features: true,
            enable_lexical_features: true,
            enable_semantic_features: true,
            max_code_length: 10000,
            feature_dimension: 768,
        }
    }
}

/// Extracted feature set from code analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureSet {
    /// AST-based structural features
    pub ast_features: ASTFeatures,
    /// Lexical and syntactic features
    pub lexical_features: LexicalFeatures,
    /// Semantic embedding features
    pub semantic_features: SemanticFeatures,
    /// Combined feature vector
    pub combined_vector: Vec<f32>,
}

/// AST-based structural features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ASTFeatures {
    /// Total number of AST nodes
    pub node_count: usize,
    /// Maximum depth of AST
    pub max_depth: usize,
    /// Number of function definitions
    pub function_count: usize,
    /// Number of struct definitions
    pub struct_count: usize,
    /// Number of enum definitions
    pub enum_count: usize,
    /// Number of impl blocks
    pub impl_count: usize,
    /// Number of macro invocations
    pub macro_count: usize,
    /// Cyclomatic complexity estimate
    pub complexity_score: f64,
}

impl Default for ASTFeatures {
    fn default() -> Self {
        Self {
            node_count: 0,
            max_depth: 0,
            function_count: 0,
            struct_count: 0,
            enum_count: 0,
            impl_count: 0,
            macro_count: 0,
            complexity_score: 0.0,
        }
    }
}

/// Lexical and syntactic features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LexicalFeatures {
    /// Total lines of code
    pub lines_of_code: usize,
    /// Number of tokens
    pub token_count: usize,
    /// Number of unique identifiers
    pub unique_identifiers: usize,
    /// Number of comments
    pub comment_count: usize,
    /// Average line length
    pub avg_line_length: f64,
    /// Keyword frequency distribution
    pub keyword_frequencies: HashMap<String, usize>,
    /// Identifier length statistics
    pub identifier_stats: IdentifierStats,
}

impl Default for LexicalFeatures {
    fn default() -> Self {
        Self {
            lines_of_code: 0,
            token_count: 0,
            unique_identifiers: 0,
            comment_count: 0,
            avg_line_length: 0.0,
            keyword_frequencies: HashMap::new(),
            identifier_stats: IdentifierStats::default(),
        }
    }
}

/// Identifier length and naming statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentifierStats {
    /// Average identifier length
    pub avg_length: f64,
    /// Maximum identifier length
    pub max_length: usize,
    /// Number of `snake_case` identifiers
    pub snake_case_count: usize,
    /// Number of CamelCase identifiers
    pub camel_case_count: usize,
    /// Number of `SCREAMING_SNAKE_CASE` identifiers
    pub screaming_snake_count: usize,
}

impl Default for IdentifierStats {
    fn default() -> Self {
        Self {
            avg_length: 0.0,
            max_length: 0,
            snake_case_count: 0,
            camel_case_count: 0,
            screaming_snake_count: 0,
        }
    }
}

/// Semantic embedding features
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct SemanticFeatures {
    /// Dense embedding vector
    pub embedding: Vec<f32>,
    /// Attention weights (if available)
    pub attention_weights: Vec<f32>,
    /// Token-level embeddings
    pub token_embeddings: Vec<Vec<f32>>,
    /// Semantic similarity scores
    pub similarity_scores: HashMap<String, f64>,
}


impl FeatureExtractor {
    /// Create a new feature extractor with default configuration
    #[must_use] pub fn new() -> Self {
        Self::with_config(FeatureConfig::default())
    }

    /// Create a new feature extractor with custom configuration
    #[must_use] pub fn with_config(config: FeatureConfig) -> Self {
        Self {
            config,
            feature_cache: HashMap::new(),
        }
    }

    /// Extract comprehensive features from source code
    pub fn extract_features(&mut self, code: &str) -> MLResult<FeatureSet> {
        // Check if code is too long
        if code.len() > self.config.max_code_length {
            return Err(Yoshi::from(format!(
                "Code length {} exceeds maximum {}",
                code.len(),
                self.config.max_code_length
            )));
        }

        let mut feature_set = FeatureSet {
            ast_features: ASTFeatures::default(),
            lexical_features: LexicalFeatures::default(),
            semantic_features: SemanticFeatures::default(),
            combined_vector: Vec::new(),
        };

        // Extract AST features if enabled
        if self.config.enable_ast_features {
            feature_set.ast_features = self.extract_ast_features(code)?;
        }

        // Extract lexical features if enabled
        if self.config.enable_lexical_features {
            feature_set.lexical_features = self.extract_lexical_features(code)?;
        }

        // Extract semantic features if enabled
        if self.config.enable_semantic_features {
            feature_set.semantic_features = self.extract_semantic_features(code)?;
        }

        // Combine all features into a single vector
        feature_set.combined_vector = self.combine_features(&feature_set)?;

        Ok(feature_set)
    }

    /// Extract AST-based structural features
    fn extract_ast_features(&self, code: &str) -> MLResult<ASTFeatures> {
        // Placeholder implementation - would use tree-sitter or syn for real AST parsing
        let lines: Vec<&str> = code.lines().collect();

        let function_count = lines.iter().filter(|line| line.contains("fn ")).count();
        let struct_count = lines.iter().filter(|line| line.contains("struct ")).count();
        let enum_count = lines.iter().filter(|line| line.contains("enum ")).count();
        let impl_count = lines.iter().filter(|line| line.contains("impl ")).count();
        let macro_count = lines.iter().filter(|line| line.contains('!')).count();

        // Simple complexity estimation based on control flow keywords
        let complexity_keywords = ["if", "else", "match", "for", "while", "loop"];
        let complexity_score = lines
            .iter()
            .map(|line| {
                complexity_keywords
                    .iter()
                    .map(|keyword| line.matches(keyword).count())
                    .sum::<usize>() as f64
            })
            .sum::<f64>()
            + 1.0; // Base complexity of 1

        Ok(ASTFeatures {
            node_count: lines.len() * 5, // Rough estimate
            max_depth: 10,               // Placeholder
            function_count,
            struct_count,
            enum_count,
            impl_count,
            macro_count,
            complexity_score,
        })
    }

    /// Extract lexical and syntactic features
    fn extract_lexical_features(&self, code: &str) -> MLResult<LexicalFeatures> {
        let lines: Vec<&str> = code.lines().collect();
        let tokens: Vec<&str> = code.split_whitespace().collect();

        // Count comments
        let comment_count = lines
            .iter()
            .filter(|line| {
                line.trim_start().starts_with("//") || line.trim_start().starts_with("/*")
            })
            .count();

        // Calculate average line length
        let total_chars: usize = lines.iter().map(|line| line.len()).sum();
        let avg_line_length = if lines.is_empty() {
            0.0
        } else {
            total_chars as f64 / lines.len() as f64
        };

        // Extract identifiers (simplified) - handle punctuation by cleaning tokens
        let identifiers: Vec<String> = tokens
            .iter()
            .filter_map(|token| {
                // Remove punctuation from the start and end of tokens
                let cleaned = token.trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
                if !cleaned.is_empty()
                    && cleaned.chars().all(|c| c.is_alphanumeric() || c == '_')
                    && cleaned.chars().any(char::is_alphabetic)
                // Must contain at least one letter
                {
                    Some(cleaned.to_string())
                } else {
                    None
                }
            })
            .collect();

        let unique_identifiers = identifiers
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len();

        // Analyze identifier naming patterns
        let snake_case_count = identifiers
            .iter()
            .filter(|id| {
                id.contains('_')
                    && id
                        .chars()
                        .all(|c| c.is_lowercase() || c.is_numeric() || c == '_')
            })
            .count();

        let camel_case_count = identifiers
            .iter()
            .filter(|id| id.chars().any(char::is_uppercase) && !id.contains('_'))
            .count();

        let screaming_snake_count = identifiers
            .iter()
            .filter(|id| {
                id.contains('_')
                    && id
                        .chars()
                        .all(|c| c.is_uppercase() || c.is_numeric() || c == '_')
            })
            .count();

        let avg_identifier_length = if identifiers.is_empty() {
            0.0
        } else {
            identifiers.iter().map(yoshi_core::String::len).sum::<usize>() as f64 / identifiers.len() as f64
        };

        let max_identifier_length = identifiers.iter().map(yoshi_core::String::len).max().unwrap_or(0);

        // Basic keyword frequency (simplified)
        let rust_keywords = [
            "fn", "struct", "enum", "impl", "let", "mut", "if", "else", "match", "for", "while",
        ];
        let mut keyword_frequencies = HashMap::new();
        for keyword in rust_keywords {
            let count = code.matches(keyword).count();
            if count > 0 {
                keyword_frequencies.insert(keyword.to_string(), count);
            }
        }

        Ok(LexicalFeatures {
            lines_of_code: lines.len(),
            token_count: tokens.len(),
            unique_identifiers,
            comment_count,
            avg_line_length,
            keyword_frequencies,
            identifier_stats: IdentifierStats {
                avg_length: avg_identifier_length,
                max_length: max_identifier_length,
                snake_case_count,
                camel_case_count,
                screaming_snake_count,
            },
        })
    }

    /// Extract semantic embedding features
    fn extract_semantic_features(&self, _code: &str) -> MLResult<SemanticFeatures> {
        // Placeholder implementation - would use transformer model for real embeddings
        let embedding = vec![0.0; self.config.feature_dimension];

        Ok(SemanticFeatures {
            embedding,
            attention_weights: Vec::new(),
            token_embeddings: Vec::new(),
            similarity_scores: HashMap::new(),
        })
    }

    /// Combine all feature types into a single vector
    fn combine_features(&self, feature_set: &FeatureSet) -> MLResult<Vec<f32>> {
        let mut combined = Vec::new();

        // Add AST features
        if self.config.enable_ast_features {
            combined.extend_from_slice(&[
                feature_set.ast_features.node_count as f32,
                feature_set.ast_features.max_depth as f32,
                feature_set.ast_features.function_count as f32,
                feature_set.ast_features.struct_count as f32,
                feature_set.ast_features.enum_count as f32,
                feature_set.ast_features.impl_count as f32,
                feature_set.ast_features.macro_count as f32,
                feature_set.ast_features.complexity_score as f32,
            ]);
        }

        // Add lexical features
        if self.config.enable_lexical_features {
            combined.extend_from_slice(&[
                feature_set.lexical_features.lines_of_code as f32,
                feature_set.lexical_features.token_count as f32,
                feature_set.lexical_features.unique_identifiers as f32,
                feature_set.lexical_features.comment_count as f32,
                feature_set.lexical_features.avg_line_length as f32,
                feature_set.lexical_features.identifier_stats.avg_length as f32,
                feature_set.lexical_features.identifier_stats.max_length as f32,
                feature_set
                    .lexical_features
                    .identifier_stats
                    .snake_case_count as f32,
                feature_set
                    .lexical_features
                    .identifier_stats
                    .camel_case_count as f32,
                feature_set
                    .lexical_features
                    .identifier_stats
                    .screaming_snake_count as f32,
            ]);
        }

        // Add semantic features
        if self.config.enable_semantic_features {
            combined.extend_from_slice(&feature_set.semantic_features.embedding);
        }

        // Pad or truncate to target dimension
        combined.resize(self.config.feature_dimension, 0.0);

        Ok(combined)
    }

    /// Clear the feature cache
    pub fn clear_cache(&mut self) {
        self.feature_cache.clear();
    }

    /// Get cache statistics
    #[must_use] pub fn get_cache_stats(&self) -> FeatureCacheStats {
        FeatureCacheStats {
            total_entries: self.feature_cache.len(),
            estimated_memory_bytes: self.feature_cache.len() * 1000, // Rough estimate
        }
    }
}

/// Feature cache statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureCacheStats {
    /// Total number of cached entries
    pub total_entries: usize,
    /// Estimated memory usage in bytes
    pub estimated_memory_bytes: usize,
}

impl Default for FeatureExtractor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_extractor_creation() {
        let extractor = FeatureExtractor::new();
        assert!(extractor.config.enable_ast_features);
        assert!(extractor.config.enable_lexical_features);
        assert!(extractor.config.enable_semantic_features);
    }

    #[test]
    fn test_feature_config_default() {
        let config = FeatureConfig::default();
        assert_eq!(config.feature_dimension, 768);
        assert_eq!(config.max_code_length, 10000);
    }

    #[test]
    fn test_lexical_feature_extraction() {
        let extractor = FeatureExtractor::new();
        let code = r#"
            fn hello_world() {
                println!("Hello, world!");
            }

            struct MyStruct {
                field: i32,
            }
        "#;

        let features = extractor.extract_lexical_features(code).unwrap();
        // Test lexical features (not AST features)
        assert!(features.token_count > 0);
        assert!(features.lines_of_code > 0);
        assert!(features.unique_identifiers > 0);
        assert!(features.keyword_frequencies.contains_key("fn"));
        assert!(features.keyword_frequencies.contains_key("struct"));
        assert!(features.identifier_stats.avg_length > 0.0);
    }

    #[test]
    fn test_ast_feature_extraction() {
        let extractor = FeatureExtractor::new();
        let code = r#"
            fn test() {
                if true {
                    for i in 0..10 {
                        println!("{}", i);
                    }
                }
            }
        "#;

        let features = extractor.extract_ast_features(code).unwrap();
        assert!(features.function_count > 0);
        assert!(features.complexity_score > 1.0);
        assert!(features.node_count > 0);
        assert!(features.macro_count > 0); // println! is a macro
    }

    #[test]
    fn test_comprehensive_feature_extraction() {
        let mut extractor = FeatureExtractor::new();
        let code = r"
            // This is a comment
            fn calculate_fibonacci(n: usize) -> usize {
                if n <= 1 {
                    return n;
                }
                calculate_fibonacci(n - 1) + calculate_fibonacci(n - 2)
            }

            struct FibonacciCalculator {
                cache: Vec<usize>,
            }

            impl FibonacciCalculator {
                fn new() -> Self {
                    Self { cache: Vec::new() }
                }
            }
        ";

        let feature_set = extractor.extract_features(code).unwrap();

        // Test AST features
        assert!(feature_set.ast_features.function_count >= 2);
        assert!(feature_set.ast_features.struct_count >= 1);
        assert!(feature_set.ast_features.impl_count >= 1);
        assert!(feature_set.ast_features.complexity_score > 1.0);

        // Test lexical features
        assert!(feature_set.lexical_features.lines_of_code > 10);
        assert!(feature_set.lexical_features.token_count > 20);
        assert!(feature_set.lexical_features.comment_count >= 1);
        assert!(feature_set.lexical_features.unique_identifiers > 5);

        // Test combined vector
        assert!(!feature_set.combined_vector.is_empty());
        assert_eq!(
            feature_set.combined_vector.len(),
            extractor.config.feature_dimension
        );
    }

    #[test]
    fn test_identifier_analysis() {
        let extractor = FeatureExtractor::new();
        let code = r#"
            const MAX_SIZE: usize = 100;
            const ANOTHER_CONSTANT: i32 = 42;
            let snake_case_var = 42;
            let CamelCaseVar = "test";
            fn process_data() {}
        "#;

        let features = extractor.extract_lexical_features(code).unwrap();

        // Test that we have different identifier patterns
        assert!(features.identifier_stats.snake_case_count >= 2); // snake_case_var, process_data
        assert!(features.identifier_stats.screaming_snake_count >= 2); // MAX_SIZE, ANOTHER_CONSTANT
        assert!(features.identifier_stats.camel_case_count >= 1); // CamelCaseVar
        assert!(features.identifier_stats.avg_length > 0.0);
        assert!(features.identifier_stats.max_length >= "ANOTHER_CONSTANT".len());

        // Verify we detected some identifiers
        assert!(features.unique_identifiers > 0);
    }

    #[test]
    fn test_feature_cache() {
        let mut extractor = FeatureExtractor::new();
        let initial_stats = extractor.get_cache_stats();
        assert_eq!(initial_stats.total_entries, 0);

        extractor.clear_cache();
        let cleared_stats = extractor.get_cache_stats();
        assert_eq!(cleared_stats.total_entries, 0);
    }

    #[test]
    fn test_default_implementations() {
        let ast_features = ASTFeatures::default();
        assert_eq!(ast_features.function_count, 0);
        assert_eq!(ast_features.complexity_score, 0.0);

        let lexical_features = LexicalFeatures::default();
        assert_eq!(lexical_features.lines_of_code, 0);
        assert_eq!(lexical_features.token_count, 0);

        let semantic_features = SemanticFeatures::default();
        assert!(semantic_features.embedding.is_empty());

        let identifier_stats = IdentifierStats::default();
        assert_eq!(identifier_stats.avg_length, 0.0);
        assert_eq!(identifier_stats.snake_case_count, 0);
    }
}
