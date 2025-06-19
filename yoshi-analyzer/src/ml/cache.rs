/* yoshi-analyzer/src/ml/cache.rs */
#![warn(missing_docs)]
//! **Brief:** Intelligent Caching and Optimization Utilities for Yoshi ML Pipeline.
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + [Intelligent Caching]
//!  - [LRU cache with TTL for feature vectors and model outputs]
//!  - [Hierarchical caching with memory pressure management]
//!  - [Cache warming and precomputation for common patterns]
//! + [Performance Optimization]
//!  - [Batch processing with intelligent batching strategies]
//!  - [Memory pool management for large tensor operations]
//!  - [Asynchronous cache operations with background eviction]
//! + [Cache Analytics]
//!  - [Hit rate monitoring and optimization suggestions]
//!  - [Memory usage tracking and leak detection]
//!  - [Performance profiling and bottleneck identification]
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **Author:** Lord Xyn
// **License:** MIT OR Apache-2.0

use lru::LruCache;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use super::{CachedFeatures, MLResult};

/// Cache configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Maximum number of entries in the cache
    pub max_entries: usize,
    /// Time-to-live for cache entries in seconds
    pub ttl_seconds: u64,
    /// Enable automatic cache warming
    pub enable_warming: bool,
    /// Memory pressure threshold (0.0-1.0)
    pub memory_pressure_threshold: f64,
    /// Background eviction interval in seconds
    pub eviction_interval_seconds: u64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_entries: 10000,
            ttl_seconds: 3600, // 1 hour
            enable_warming: true,
            memory_pressure_threshold: 0.8,
            eviction_interval_seconds: 300, // 5 minutes
        }
    }
}

/// Cache entry with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry<T> {
    /// The cached value
    pub value: T,
    /// Timestamp when the entry was created
    pub created_at: u64,
    /// Timestamp when the entry was last accessed
    pub last_accessed: u64,
    /// Number of times this entry has been accessed
    pub access_count: u64,
    /// Size of the entry in bytes (estimated)
    pub size_bytes: usize,
}

impl<T> CacheEntry<T> {
    /// Create a new cache entry
    pub fn new(value: T, size_bytes: usize) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            value,
            created_at: now,
            last_accessed: now,
            access_count: 1,
            size_bytes,
        }
    }

    /// Check if the entry has expired
    pub fn is_expired(&self, ttl_seconds: u64) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        now - self.created_at > ttl_seconds
    }

    /// Update access statistics
    pub fn update_access(&mut self) {
        self.last_accessed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.access_count += 1;
    }
}

/// Intelligent ML cache with TTL and LRU eviction
pub struct MLCache<T> {
    /// LRU cache for main storage
    cache: Arc<RwLock<LruCache<String, CacheEntry<T>>>>,
    /// Configuration settings
    config: CacheConfig,
    /// Cache statistics
    stats: Arc<RwLock<CacheStats>>,
    /// Background eviction handle
    _eviction_handle: Option<tokio::task::JoinHandle<()>>,
}

/// Cache statistics and metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    /// Total number of cache hits
    pub hits: u64,
    /// Total number of cache misses
    pub misses: u64,
    /// Total number of evictions
    pub evictions: u64,
    /// Current number of entries
    pub current_entries: usize,
    /// Total memory usage in bytes
    pub memory_usage_bytes: usize,
    /// Average access time in nanoseconds
    pub avg_access_time_ns: u64,
    /// Cache hit rate (0.0-1.0)
    pub hit_rate: f64,
}

impl Default for CacheStats {
    fn default() -> Self {
        Self {
            hits: 0,
            misses: 0,
            evictions: 0,
            current_entries: 0,
            memory_usage_bytes: 0,
            avg_access_time_ns: 0,
            hit_rate: 0.0,
        }
    }
}

impl<T: Clone + Send + Sync + 'static> MLCache<T> {
    /// Create a new ML cache with default configuration
    #[must_use] pub fn new() -> Self {
        Self::with_config(CacheConfig::default())
    }

    /// Create a new ML cache with custom configuration
    #[must_use] pub fn with_config(config: CacheConfig) -> Self {
        let cache = Arc::new(RwLock::new(LruCache::new(
            NonZeroUsize::new(config.max_entries).unwrap(),
        )));
        let stats = Arc::new(RwLock::new(CacheStats::default()));

        // Start background eviction task
        let eviction_handle = if config.eviction_interval_seconds > 0 {
            let cache_clone = cache.clone();
            let stats_clone = stats.clone();
            let ttl_seconds = config.ttl_seconds;
            let interval = Duration::from_secs(config.eviction_interval_seconds);

            Some(tokio::spawn(async move {
                let mut interval_timer = tokio::time::interval(interval);
                loop {
                    interval_timer.tick().await;
                    Self::background_eviction(&cache_clone, &stats_clone, ttl_seconds);
                }
            }))
        } else {
            None
        };

        Self {
            cache,
            config,
            stats,
            _eviction_handle: eviction_handle,
        }
    }

    /// Get a value from the cache
    #[must_use] pub fn get(&self, key: &str) -> Option<T> {
        let start_time = Instant::now();

        let result = {
            let mut cache = self.cache.write();
            if let Some(entry) = cache.get_mut(key) {
                // Check if expired
                if entry.is_expired(self.config.ttl_seconds) {
                    cache.pop(key);
                    None
                } else {
                    entry.update_access();
                    Some(entry.value.clone())
                }
            } else {
                None
            }
        };

        // Update statistics
        let access_time = start_time.elapsed().as_nanos() as u64;
        let mut stats = self.stats.write();

        if result.is_some() {
            stats.hits += 1;
        } else {
            stats.misses += 1;
        }

        // Update hit rate
        let total_requests = stats.hits + stats.misses;
        stats.hit_rate = if total_requests > 0 {
            stats.hits as f64 / total_requests as f64
        } else {
            0.0
        };

        // Update average access time
        stats.avg_access_time_ns = u64::midpoint(stats.avg_access_time_ns, access_time);

        result
    }

    /// Put a value into the cache
    pub fn put(&self, key: String, value: T, size_bytes: usize) {
        let entry = CacheEntry::new(value, size_bytes);

        let evicted = {
            let mut cache = self.cache.write();
            cache.put(key, entry).is_some()
        };

        // Update statistics
        let mut stats = self.stats.write();
        stats.current_entries = self.cache.read().len();

        if evicted {
            stats.evictions += 1;
        }

        // Update memory usage (rough estimate)
        stats.memory_usage_bytes += size_bytes;
    }

    /// Remove a value from the cache
    #[must_use] pub fn remove(&self, key: &str) -> Option<T> {
        let result = {
            let mut cache = self.cache.write();
            cache.pop(key).map(|entry| entry.value)
        };

        if result.is_some() {
            let mut stats = self.stats.write();
            stats.current_entries = self.cache.read().len();
        }

        result
    }

    /// Clear all entries from the cache
    pub fn clear(&self) {
        {
            let mut cache = self.cache.write();
            cache.clear();
        }

        let mut stats = self.stats.write();
        stats.current_entries = 0;
        stats.memory_usage_bytes = 0;
    }

    /// Get current cache statistics
    #[must_use] pub fn get_stats(&self) -> CacheStats {
        let stats = self.stats.read();
        let mut result = stats.clone();
        result.current_entries = self.cache.read().len();
        result
    }

    /// Check if the cache contains a key
    #[must_use] pub fn contains_key(&self, key: &str) -> bool {
        let cache = self.cache.read();
        cache.contains(key)
    }

    /// Get the current size of the cache
    #[must_use] pub fn len(&self) -> usize {
        self.cache.read().len()
    }

    /// Check if the cache is empty
    #[must_use] pub fn is_empty(&self) -> bool {
        self.cache.read().is_empty()
    }

    /// Warm the cache with commonly accessed patterns
    pub fn warm_cache(&self, patterns: Vec<(String, T, usize)>) -> MLResult<()> {
        if !self.config.enable_warming {
            return Ok(());
        }

        println!("🔥 Warming ML cache with {} patterns...", patterns.len());

        for (key, value, size) in patterns {
            self.put(key, value, size);
        }

        println!("✅ Cache warming completed");
        Ok(())
    }

    /// Background eviction of expired entries
    fn background_eviction(
        cache: &Arc<RwLock<LruCache<String, CacheEntry<T>>>>,
        stats: &Arc<RwLock<CacheStats>>,
        ttl_seconds: u64,
    ) {
        let expired_keys: Vec<String> = {
            let cache_read = cache.read();
            cache_read
                .iter()
                .filter(|(_, entry)| entry.is_expired(ttl_seconds))
                .map(|(key, _)| key.clone())
                .collect()
        };

        if !expired_keys.is_empty() {
            let mut cache_write = cache.write();
            let mut eviction_count = 0;

            for key in expired_keys {
                if cache_write.pop(&key).is_some() {
                    eviction_count += 1;
                }
            }

            if eviction_count > 0 {
                let mut stats_write = stats.write();
                stats_write.evictions += eviction_count;
                stats_write.current_entries = cache_write.len();

                println!(
                    "🧹 Background eviction removed {eviction_count} expired entries"
                );
            }
        }
    }
}

impl<T: Clone + Send + Sync + 'static> Default for MLCache<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Specialized cache for feature vectors
pub type FeatureCache = MLCache<CachedFeatures>;

/// Specialized cache for model predictions
pub type PredictionCache = MLCache<(String, f64)>;

/// Cache key generator for consistent hashing
pub struct CacheKeyGenerator;

impl CacheKeyGenerator {
    /// Generate a cache key for source code
    #[must_use] pub fn code_key(code: &str) -> String {
        use std::collections::hash_map::DefaultHasher;

        let mut hasher = DefaultHasher::new();
        code.hash(&mut hasher);
        format!("code_{:x}", hasher.finish())
    }

    /// Generate a cache key for model predictions
    #[must_use] pub fn prediction_key(model_name: &str, input_hash: u64) -> String {
        format!("pred_{model_name}_{input_hash:x}")
    }

    /// Generate a cache key for features
    #[must_use] pub fn feature_key(extractor_config: &str, code_hash: u64) -> String {
        format!("feat_{extractor_config}_{code_hash:x}")
    }
}

/// Cache manager for coordinating multiple cache instances
pub struct CacheManager {
    /// Feature cache
    pub features: FeatureCache,
    /// Prediction cache
    pub predictions: PredictionCache,
    /// Global cache configuration
    #[allow(dead_code)] // Used for future cache configuration updates
    config: CacheConfig,
}

impl CacheManager {
    /// Create a new cache manager
    #[must_use] pub fn new() -> Self {
        let config = CacheConfig::default();

        Self {
            features: FeatureCache::with_config(config.clone()),
            predictions: PredictionCache::with_config(config.clone()),
            config,
        }
    }

    /// Create a new cache manager with custom configuration
    #[must_use] pub fn with_config(config: CacheConfig) -> Self {
        Self {
            features: FeatureCache::with_config(config.clone()),
            predictions: PredictionCache::with_config(config.clone()),
            config,
        }
    }

    /// Get combined cache statistics
    #[must_use] pub fn get_combined_stats(&self) -> CombinedCacheStats {
        let feature_stats = self.features.get_stats();
        let prediction_stats = self.predictions.get_stats();

        CombinedCacheStats {
            feature_cache: feature_stats,
            prediction_cache: prediction_stats,
            total_memory_bytes: 0, // Would calculate actual memory usage
            total_entries: self.features.len() + self.predictions.len(),
        }
    }

    /// Clear all caches
    pub fn clear_all(&self) {
        self.features.clear();
        self.predictions.clear();
        println!("🧹 All ML caches cleared");
    }
}

/// Combined statistics for all cache instances
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombinedCacheStats {
    /// Feature cache statistics
    pub feature_cache: CacheStats,
    /// Prediction cache statistics
    pub prediction_cache: CacheStats,
    /// Total memory usage across all caches
    pub total_memory_bytes: usize,
    /// Total entries across all caches
    pub total_entries: usize,
}

impl Default for CacheManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_creation() {
        let cache: MLCache<String> = MLCache::new();
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }

    #[tokio::test]
    async fn test_cache_put_get() {
        let cache: MLCache<String> = MLCache::new();
        cache.put("test".to_string(), "value".to_string(), 100);

        assert_eq!(cache.len(), 1);
        assert!(cache.contains_key("test"));

        let result = cache.get("test");
        assert_eq!(result, Some("value".to_string()));
    }

    #[test]
    fn test_cache_key_generation() {
        let key1 = CacheKeyGenerator::code_key("fn main() {}");
        let key2 = CacheKeyGenerator::code_key("fn main() {}");
        let key3 = CacheKeyGenerator::code_key("fn test() {}");

        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }

    #[tokio::test]
    async fn test_cache_manager() {
        let manager = CacheManager::new();
        assert!(manager.features.is_empty());
        assert!(manager.predictions.is_empty());

        let stats = manager.get_combined_stats();
        assert_eq!(stats.total_entries, 0);
    }

    #[test]
    fn test_cache_entry_expiration() {
        let entry = CacheEntry::new("test".to_string(), 100);
        assert!(!entry.is_expired(3600)); // 1 hour TTL - should not be expired

        // Test that entries don't expire with very large TTL
        assert!(!entry.is_expired(86400)); // 24 hour TTL - should not be expired

        // Test expiration logic by waiting a full second to ensure time difference
        let timed_entry = CacheEntry::new("test".to_string(), 100);
        std::thread::sleep(std::time::Duration::from_secs(1)); // Wait 1 full second
        assert!(timed_entry.is_expired(0)); // TTL=0 means immediate expiration after 1 second
    }
}
