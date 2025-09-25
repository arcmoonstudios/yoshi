/* yoshi-deluxe/src/docs.rs */
//! **Brief:** Documentation scraping engine with intelligent fallback strategies for yoshi-deluxe.
//!
//! This module provides comprehensive documentation scraping capabilities with robust
//! error handling, multiple source fallbacks, and intelligent caching. It integrates
//! with the yoshi error framework to provide detailed error context and recovery options.

use crate::{
    constants::{DOCS_CACHE, DOCS_SCRAPING_RETRY_COUNT, HTTP_CLIENT, REGEX_PATTERNS},
    errors::{factory, Result, YoshiDeluxeExt},
    types::{
        CachedDocsData, CodeExample, CrateInfo, DataSource, MethodSignature, MethodSuggestion,
        Parameter, StabilityInfo, StabilityLevel, TraitImplementation,
    },
};
use scraper::{Html, Selector};
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::{Duration, SystemTime},
};
use tokio::time::timeout;
use yoshi_std::{HatchExt, LayText};

//--------------------------------------------------------------------------------------------------
// Documentation Scraping Engine with Structured API Support
//--------------------------------------------------------------------------------------------------

/// Production-grade documentation scraping engine with structured API support
pub struct DocsScrapingEngine {
    /// HTTP client with connection pooling
    client: &'static reqwest::Client,
    /// Documentation cache
    cache: &'static Arc<tokio::sync::RwLock<HashMap<String, CachedDocsData>>>,
    /// Scraping metrics
    metrics: ScrapingMetrics,
}

/// Scraping performance metrics
#[derive(Debug, Default)]
pub struct ScrapingMetrics {
    /// Successful scrapes
    pub successful_scrapes: AtomicU64,
    /// Failed scrapes
    pub failed_scrapes: AtomicU64,
    /// Cache hits
    pub cache_hits: AtomicU64,
    /// URLs attempted
    pub urls_attempted: AtomicU64,
    /// Total methods scraped
    pub methods_scraped: AtomicU64,
    /// Retry operations
    pub retry_operations: AtomicU64,
}

impl ScrapingMetrics {
    /// Record successful scrape
    pub fn record_success(&self, methods_count: usize) {
        self.successful_scrapes.fetch_add(1, Ordering::Relaxed);
        self.methods_scraped
            .fetch_add(methods_count as u64, Ordering::Relaxed);
    }

    /// Record failed scrape
    pub fn record_failure(&self) {
        self.failed_scrapes.fetch_add(1, Ordering::Relaxed);
    }

    /// Record cache hit
    pub fn record_cache_hit(&self) {
        self.cache_hits.fetch_add(1, Ordering::Relaxed);
    }

    /// Record URL attempt
    pub fn record_url_attempt(&self) {
        self.urls_attempted.fetch_add(1, Ordering::Relaxed);
    }

    /// Record retry operation
    pub fn record_retry(&self) {
        self.retry_operations.fetch_add(1, Ordering::Relaxed);
    }

    /// Get success rate
    #[must_use]
    pub fn success_rate(&self) -> f64 {
        let success = self.successful_scrapes.load(Ordering::Relaxed) as f64;
        let total = success + self.failed_scrapes.load(Ordering::Relaxed) as f64;
        if total > 0.0 {
            success / total
        } else {
            0.0
        }
    }

    /// Get cache hit rate
    #[must_use]
    pub fn cache_hit_rate(&self) -> f64 {
        let hits = self.cache_hits.load(Ordering::Relaxed) as f64;
        let total = hits + self.successful_scrapes.load(Ordering::Relaxed) as f64;
        if total > 0.0 {
            hits / total
        } else {
            0.0
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Documentation Scraping Implementation
//--------------------------------------------------------------------------------------------------

impl DocsScrapingEngine {
    /// Creates a new documentation scraping engine
    #[must_use]
    pub fn new() -> Self {
        Self {
            client: &HTTP_CLIENT,
            cache: &DOCS_CACHE,
            metrics: ScrapingMetrics::default(),
        }
    }

    /// Scrapes documentation for a specific type/crate with fallback strategies
    ///
    /// # Errors
    ///
    /// Returns a yoshi error if all documentation sources fail
    pub async fn scrape_type_documentation(
        &self,
        crate_name: &str,
        type_name: &str,
    ) -> Result<CachedDocsData> {
        let cache_key = format!("{crate_name}::{type_name}");

        // Check cache first
        if let Some(cached) = self.get_cached_docs(&cache_key).await {
            self.metrics.record_cache_hit();
            return Ok(cached);
        }

        // Try HTML scraping with fallback strategies
        let docs_data = self
            .try_html_scraping_with_retry(crate_name, type_name)
            .await
            .lay("Attempting documentation scraping with retries")?;

        // Cache the result
        self.cache_docs(cache_key, docs_data.clone()).await;
        self.metrics.record_success(docs_data.methods.len());

        Ok(docs_data)
    }

    /// HTML scraping with robust error handling and multiple URL attempts
    async fn try_html_scraping_with_retry(
        &self,
        crate_name: &str,
        type_name: &str,
    ) -> Result<CachedDocsData> {
        let mut last_error = None;

        for attempt in 0..=DOCS_SCRAPING_RETRY_COUNT {
            match self.try_html_scraping(crate_name, type_name).await {
                Ok(data) => return Ok(data),
                Err(error) => {
                    last_error = Some(error);
                    if attempt < DOCS_SCRAPING_RETRY_COUNT {
                        self.metrics.record_retry();
                        // Exponential backoff
                        let delay = Duration::from_millis(100 * 2_u64.pow(attempt as u32));
                        tokio::time::sleep(delay).await;
                    }
                }
            }
        }

        self.metrics.record_failure();
        Err(last_error.unwrap_or_else(|| {
            factory::docs_scraping_error(
                crate_name,
                type_name,
                "max_retries_exceeded",
                "Maximum retry attempts exceeded".to_string(),
            )
        }))
    }

    /// HTML scraping with robust error handling and multiple URL attempts
    async fn try_html_scraping(&self, crate_name: &str, type_name: &str) -> Result<CachedDocsData> {
        let urls = self.generate_documentation_urls(crate_name, type_name);
        let mut last_error = None;

        for url in urls {
            self.metrics.record_url_attempt();
            match self.scrape_url(&url).await {
                Ok(html) => {
                    return self
                        .parse_documentation(&html, &url, crate_name, type_name)
                        .await
                        .lay("Parsing scraped documentation content");
                }
                Err(error) => {
                    last_error = Some(error);
                    continue;
                }
            }
        }

        Err(last_error.unwrap_or_else(|| {
            factory::docs_scraping_error(
                crate_name,
                type_name,
                "no_valid_urls",
                "No valid URLs found".to_string(),
            )
        }))
    }

    /// Generate comprehensive list of documentation URLs to try
    fn generate_documentation_urls(&self, crate_name: &str, type_name: &str) -> Vec<String> {
        let crate_slug = crate_name.replace('-', "_");
        vec![
            // Primary docs.rs URLs
            format!("https://docs.rs/{crate_name}/latest/{crate_slug}/struct.{type_name}.html"),
            format!("https://docs.rs/{crate_name}/latest/{crate_slug}/enum.{type_name}.html"),
            format!("https://docs.rs/{crate_name}/latest/{crate_slug}/trait.{type_name}.html"),
            format!("https://docs.rs/{crate_name}/latest/{crate_slug}/type.{type_name}.html"),
            // Alternative version patterns
            format!("https://docs.rs/{crate_name}/*/{crate_slug}/struct.{type_name}.html"),
            format!("https://docs.rs/{crate_name}/*/{crate_slug}/enum.{type_name}.html"),
            // Module-specific patterns
            format!("https://docs.rs/{crate_name}/latest/{crate_slug}/{type_name}/index.html"),
            // Alternative crate name patterns
            format!("https://docs.rs/{crate_slug}/latest/{crate_slug}/struct.{type_name}.html"),
        ]
    }

    /// Scrape a specific URL with timeout and error handling
    async fn scrape_url(&self, url: &str) -> Result<String> {
        let request_future = self.client.get(url).send();

        let response = timeout(crate::constants::HTTP_TIMEOUT, request_future)
            .await
            .map_err(|_| {
                factory::docs_scraping_error(
                    "unknown",
                    "unknown",
                    "request_timeout",
                    "Request timed out".to_string(),
                )
            })
            .lay("Awaiting HTTP response")?
            .map_err(|e| factory::docs_scraping_error("unknown", "unknown", "network_error", e.to_string()))
            .lay("Sending HTTP request")?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            return Err(factory::docs_scraping_error(
                "unknown",
                "unknown",
                &format!("http_error_{status}"),
                format!("HTTP {status}"),
            ))
            .lay("Checking HTTP response status");
        }

        response
            .text()
            .await
            .with_operation_context("response_body_reading")
            .lay("Reading response body")
    }

    /// Parse HTML documentation with robust selector handling
    async fn parse_documentation(
        &self,
        html: &str,
        url: &str,
        crate_name: &str,
        type_name: &str,
    ) -> Result<CachedDocsData> {
        let document = Html::parse_document(html);

        // Use fallback selectors for robustness
        let methods = self
            .extract_methods_robust(&document)
            .lay("Extracting method signatures")?;
        let implementations = self
            .extract_implementations_robust(&document)
            .lay("Extracting trait implementations")?;
        let examples = self
            .extract_examples_robust(&document)
            .lay("Extracting code examples")?;

        let crate_info = CrateInfo {
            name: crate_name.to_string(),
            version: self
                .extract_version_from_url(url)
                .unwrap_or_else(|| "latest".to_string()),
            docs_url: url.to_string(),
            repository: self.extract_repository_link(&document),
            description: self.extract_crate_description(&document),
            license: self.extract_license_info(&document),
        };

        let docs_data = CachedDocsData::new(
            crate_info,
            methods,
            implementations,
            examples,
            DataSource::DocsRs {
                url: url.to_string(),
            },
        );

        Ok(docs_data)
    }

    /// Extract methods with multiple selector fallbacks
    fn extract_methods_robust(&self, document: &Html) -> Result<Vec<MethodSignature>> {
        let selectors = [
            ".method",
            ".impl-items .method",
            "[data-method]",
            ".item-decl",
            ".method-signature",
        ];

        let mut methods = Vec::new();

        for selector_str in &selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                for element in document.select(&selector) {
                    if let Some(method) = self.parse_method_element(&element) {
                        methods.push(method);
                    }
                }
            }

            if !methods.is_empty() {
                break; // Use first successful selector
            }
        }

        // If no methods found with standard selectors, try generic extraction
        if methods.is_empty() {
            methods = self.extract_methods_generic(&document)?;
        }

        Ok(methods)
    }

    /// Parse individual method element with error recovery
    fn parse_method_element(&self, element: &scraper::ElementRef<'_>) -> Option<MethodSignature> {
        let name_selector = Selector::parse(".method-name, .item-name, code").ok()?;
        let name = element
            .select(&name_selector)
            .next()?
            .text()
            .collect::<String>()
            .trim()
            .to_string();

        // Skip if name is empty or looks invalid
        if name.is_empty() || name.len() > 100 {
            return None;
        }

        let signature_selector = Selector::parse(".signature, pre, .item-decl").ok()?;
        let signature = element
            .select(&signature_selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string());

        let docblock_selector = Selector::parse(".docblock, .item-docs").ok()?;
        let documentation =
            element
                .select(&docblock_selector)
                .next()
                .map_or_else(String::new, |el| {
                    el.text()
                        .collect::<String>()
                        .trim()
                        .chars()
                        .take(1000) // Limit documentation length
                        .collect()
                });

        let mut method = MethodSignature::new(name);

        if let Some(sig) = signature.as_ref() {
            method.parameters = self.parse_parameters_from_signature(sig);
            method.return_type = self.extract_return_type_from_signature(sig);
        }

        method.documentation = documentation;
        method.visibility = "pub".to_string(); // Default assumption for docs.rs
        method.stability = self.extract_stability_info(element);

        Some(method)
    }

    /// Generic method extraction when specific selectors fail
    fn extract_methods_generic(&self, document: &Html) -> Result<Vec<MethodSignature>> {
        let mut methods = Vec::new();

        // Look for function signatures in any code blocks
        if let Ok(code_selector) = Selector::parse("code, pre") {
            for element in document.select(&code_selector) {
                let text = element.text().collect::<String>();
                if let Some(regex) = REGEX_PATTERNS.get("method_signature") {
                    for capture in regex.captures_iter(&text) {
                        if let Some(method_name) = capture.get(1) {
                            let mut method = MethodSignature::new(method_name.as_str());

                            if let Some(params) = capture.get(2) {
                                method.parameters =
                                    self.parse_parameters_from_signature(params.as_str());
                            }

                            if let Some(return_type) = capture.get(3) {
                                method.return_type = Some(return_type.as_str().trim().to_string());
                            }

                            methods.push(method);
                        }
                    }
                }
            }
        }

        Ok(methods)
    }

    /// Parse parameters from method signature with enhanced parsing
    fn parse_parameters_from_signature(&self, signature: &str) -> Vec<Parameter> {
        let Some(params_start) = signature.find('(') else {
            return Vec::new();
        };
        let Some(params_end) = signature[params_start..].find(')') else {
            return Vec::new();
        };

        let params_str = &signature[params_start + 1..params_start + params_end];

        params_str
            .split(',')
            .filter_map(|param| {
                let param = param.trim();
                if param.is_empty() || param == "self" || param.starts_with("&self") {
                    return None;
                }

                let parts: Vec<&str> = param.splitn(2, ':').collect();
                if parts.len() == 2 {
                    let name = parts[0].trim();
                    let param_type = parts[1].trim();

                    // Clean up parameter name
                    let clean_name = name
                        .trim_start_matches("mut ")
                        .trim_start_matches("ref ")
                        .trim();

                    let mut parameter = Parameter::new(clean_name, param_type);

                    if name.contains("mut ") {
                        parameter.mark_mutable();
                    }

                    Some(parameter)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Extract return type from signature
    fn extract_return_type_from_signature(&self, signature: &str) -> Option<String> {
        if let Some(arrow_pos) = signature.find("->") {
            let return_part = signature[arrow_pos + 2..].trim();

            // Find the end of the return type (before where clause or opening brace)
            let end_pos = return_part
                .find(" where")
                .or_else(|| return_part.find(" {"))
                .or_else(|| return_part.find(';'))
                .unwrap_or(return_part.len());

            Some(return_part[..end_pos].trim().to_string())
        } else {
            None
        }
    }

    /// Extract stability information from element
    fn extract_stability_info(&self, element: &scraper::ElementRef<'_>) -> StabilityInfo {
        let mut stability = StabilityInfo::default();

        // Look for stability attributes in the element
        let text = element.text().collect::<String>().to_lowercase();

        if text.contains("unstable") || text.contains("experimental") {
            stability.level = StabilityLevel::Unstable;
        } else if text.contains("internal") {
            stability.level = StabilityLevel::Internal;
        }

        // Look for feature gates
        if let Some(start) = text.find("feature = \"") {
            if let Some(end) = text[start + 11..].find('"') {
                stability.feature = Some(text[start + 11..start + 11 + end].to_string());
            }
        }

        stability
    }

    /// Extract trait implementations with fallback selectors
    fn extract_implementations_robust(&self, document: &Html) -> Result<Vec<TraitImplementation>> {
        let selectors = [
            ".impl-items",
            ".trait-implementations",
            "[data-impl]",
            ".impl",
            "#implementations",
        ];

        let mut implementations = Vec::new();

        for selector_str in &selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                for element in document.select(&selector) {
                    if let Some(impl_info) = self.parse_impl_element(&element) {
                        implementations.push(impl_info);
                    }
                }
            }
        }

        Ok(implementations)
    }

    /// Parse implementation element
    fn parse_impl_element(&self, element: &scraper::ElementRef<'_>) -> Option<TraitImplementation> {
        let impl_text = element.text().collect::<String>();

        // Try structured parsing first
        if let Some(regex) = REGEX_PATTERNS.get("api_trait_impl") {
            if let Some(captures) = regex.captures(&impl_text) {
                let trait_name = captures.get(1)?.as_str().to_string();
                let implementing_type = captures.get(2)?.as_str().to_string();

                let method_selector = Selector::parse(".method, .method-name").ok()?;
                let methods = element
                    .select(&method_selector)
                    .map(|el| el.text().collect::<String>().trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();

                return Some(TraitImplementation::new(trait_name, implementing_type));
            }
        }

        // Fallback to text-based parsing
        let lines: Vec<&str> = impl_text.lines().collect();
        for line in lines {
            if line.trim().starts_with("impl") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 && parts[2] == "for" {
                    let trait_name = parts[1].to_string();
                    let implementing_type = parts[3].to_string();

                    let mut implementation =
                        TraitImplementation::new(trait_name, implementing_type);

                    // Extract methods from the impl block
                    let method_selector = Selector::parse(".method").ok()?;
                    let methods: Vec<String> = element
                        .select(&method_selector)
                        .map(|el| el.text().collect::<String>().trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();

                    for method in methods {
                        implementation.add_method(method);
                    }

                    return Some(implementation);
                }
            }
        }

        None
    }

    /// Extract code examples with multiple selector strategies
    fn extract_examples_robust(&self, document: &Html) -> Result<Vec<CodeExample>> {
        let selectors = [
            ".example-wrap pre",
            ".docblock pre",
            "pre.playground",
            "code.rust",
            ".rustdoc-example",
        ];

        let mut examples = Vec::new();

        for selector_str in &selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                for element in document.select(&selector) {
                    let code = element.text().collect::<String>().trim().to_string();

                    if !code.is_empty() && code.len() > 10 && code.len() < 10000 {
                        let mut example =
                            CodeExample::new(code, "Documentation example".to_string());

                        // Analyze code complexity
                        example.set_complexity(self.analyze_code_complexity(&example.code));

                        // Check if it looks like it compiles
                        example.set_compiles(self.estimate_compilation_status(&example.code));

                        examples.push(example);
                    }
                }
            }
        }

        // Remove duplicate examples
        examples.sort_by(|a, b| a.code.len().cmp(&b.code.len()));
        examples.dedup_by(|a, b| self.calculate_code_similarity(&a.code, &b.code) > 0.8);

        Ok(examples)
    }

    /// Analyze code complexity
    fn analyze_code_complexity(&self, code: &str) -> u8 {
        let mut complexity = 1;

        // Count various complexity indicators
        complexity += code.matches("fn ").count().min(3) as u8;
        complexity += code.matches("if ").count().min(2) as u8;
        complexity += code.matches("match ").count().min(2) as u8;
        complexity += code.matches("loop ").count().min(2) as u8;
        complexity += code.matches("async ").count().min(2) as u8;
        complexity += code.matches("unsafe ").count().min(3) as u8;

        complexity.min(5)
    }

    /// Estimate if code compiles
    fn estimate_compilation_status(&self, code: &str) -> bool {
        // Simple heuristics for compilation status
        !code.contains("// This won't compile")
            && !code.contains("compile_fail")
            && !code.contains("TODO")
            && !code.contains("unimplemented!")
            && code.matches('{').count() == code.matches('}').count()
            && code.matches('(').count() == code.matches(')').count()
    }

    /// Calculate similarity between code snippets
    fn calculate_code_similarity(&self, code1: &str, code2: &str) -> f64 {
        if code1 == code2 {
            return 1.0;
        }

        let len1 = code1.len();
        let len2 = code2.len();
        let max_len = len1.max(len2);

        if max_len == 0 {
            return 1.0;
        }

        // Simple similarity based on common characters
        let common_chars = code1
            .chars()
            .zip(code2.chars())
            .take_while(|(c1, c2)| c1 == c2)
            .count();

        common_chars as f64 / max_len as f64
    }

    /// Extract version from URL
    fn extract_version_from_url(&self, url: &str) -> Option<String> {
        if let Some(start) = url.find("/docs.rs/") {
            let remaining = &url[start + 9..];
            if let Some(slash_pos) = remaining.find('/') {
                let _crate_part = &remaining[..slash_pos];
                if let Some(version_start) = remaining[slash_pos + 1..].find('/') {
                    let version = &remaining[slash_pos + 1..slash_pos + 1 + version_start];
                    if version != "latest" {
                        return Some(version.to_string());
                    }
                }
            }
        }
        None
    }

    /// Extract repository link from document
    fn extract_repository_link(&self, document: &Html) -> Option<String> {
        if let Ok(selector) = Selector::parse("a[href*='github.com'], a[href*='gitlab.com']") {
            for element in document.select(&selector) {
                if let Some(href) = element.value().attr("href") {
                    return Some(href.to_string());
                }
            }
        }
        None
    }

    /// Extract crate description
    fn extract_crate_description(&self, document: &Html) -> Option<String> {
        if let Ok(selector) = Selector::parse(".crate-description, .docblock p") {
            if let Some(element) = document.select(&selector).next() {
                let description = element.text().collect::<String>().trim().to_string();
                if !description.is_empty() && description.len() < 500 {
                    return Some(description);
                }
            }
        }
        None
    }

    /// Extract license information
    fn extract_license_info(&self, document: &Html) -> Option<String> {
        if let Ok(selector) = Selector::parse("a[href*='license'], .license") {
            if let Some(element) = document.select(&selector).next() {
                return Some(element.text().collect::<String>().trim().to_string());
            }
        }
        None
    }

    /// Searches for similar method names with fuzzy matching
    ///
    /// # Errors
    ///
    /// Returns a yoshi error if documentation scraping fails
    pub async fn search_similar_methods(
        &self,
        crate_name: &str,
        type_name: &str,
        target_method: &str,
    ) -> Result<Vec<MethodSuggestion>> {
        let docs_data = self
            .scrape_type_documentation(crate_name, type_name)
            .await
            .lay("Scraping documentation for method search")?;

        let mut suggestions: Vec<_> = docs_data
            .methods
            .iter()
            .filter_map(|method| {
                let similarity = self.calculate_similarity(&method.name, target_method);
                if similarity > crate::constants::DEFAULT_SIMILARITY_THRESHOLD {
                    Some(MethodSuggestion::new(
                        method.name.clone(),
                        similarity,
                        method.canonical_signature(),
                        method.documentation.clone(),
                    ))
                } else {
                    None
                }
            })
            .collect();

        suggestions.sort_by(|a, b| {
            b.similarity_score
                .partial_cmp(&a.similarity_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        suggestions.truncate(10);
        Ok(suggestions)
    }

    /// Enhanced string similarity calculation using multiple algorithms
    fn calculate_similarity(&self, a: &str, b: &str) -> f64 {
        let levenshtein = self.levenshtein_similarity(a, b);
        let jaro_winkler = self.jaro_winkler_similarity(a, b);
        let common_prefix = self.common_prefix_similarity(a, b);
        0.5 * levenshtein + 0.3 * jaro_winkler + 0.2 * common_prefix
    }

    /// Levenshtein distance similarity
    fn levenshtein_similarity(&self, a: &str, b: &str) -> f64 {
        let (a_len, b_len) = (a.chars().count(), b.chars().count());
        if a_len == 0 {
            return if b_len == 0 { 1.0 } else { 0.0 };
        }
        if b_len == 0 {
            return 0.0;
        }

        let mut column: Vec<usize> = (0..=a_len).collect();
        for (_j, b_char) in b.chars().enumerate() {
            let mut last_diag = column[0];
            column[0] += 1;
            for (i, a_char) in a.chars().enumerate() {
                let old_diag = column[i + 1];
                let cost = if a_char == b_char { 0 } else { 1 };
                column[i + 1] = (column[i + 1] + 1).min(column[i] + 1).min(last_diag + cost);
                last_diag = old_diag;
            }
        }

        let distance = column[a_len];
        1.0 - (distance as f64 / a_len.max(b_len) as f64)
    }

    /// Simplified Jaro-Winkler similarity
    fn jaro_winkler_similarity(&self, a: &str, b: &str) -> f64 {
        if a == b {
            return 1.0;
        }
        let (a_len, b_len) = (a.len(), b.len());
        if a_len == 0 || b_len == 0 {
            return 0.0;
        }

        let common_prefix = a
            .chars()
            .zip(b.chars())
            .take(4)
            .take_while(|(c1, c2)| c1 == c2)
            .count();
        let common_chars = a.chars().filter(|&c| b.contains(c)).count();
        let jaro = common_chars as f64 / a_len.max(b_len) as f64;

        jaro + (0.1 * common_prefix as f64 * (1.0 - jaro))
    }

    /// Common prefix similarity
    fn common_prefix_similarity(&self, a: &str, b: &str) -> f64 {
        let common_prefix = a
            .chars()
            .zip(b.chars())
            .take_while(|(c1, c2)| c1 == c2)
            .count();
        let max_len = a.len().max(b.len());
        if max_len == 0 {
            1.0
        } else {
            common_prefix as f64 / max_len as f64
        }
    }

    /// Retrieves cached documentation data with validation
    async fn get_cached_docs(&self, key: &str) -> Option<CachedDocsData> {
        let mut cache = self.cache.write().await;
        if let Some(cached) = cache.get_mut(key) {
            if cached.is_valid() {
                cached.touch();
                return Some(cached.clone());
            } else {
                // Remove expired cache entry
                cache.remove(key);
            }
        }
        None
    }

    /// Caches documentation data with LRU eviction
    async fn cache_docs(&self, key: String, data: CachedDocsData) {
        let mut cache = self.cache.write().await;

        if cache.len() >= crate::constants::MAX_CACHE_ENTRIES {
            let mut entries: Vec<_> = cache.iter().map(|(k, v)| (k.clone(), v.access_count())).collect();
            entries.sort_by_key(|(_, count)| *count);
            let keys_to_remove: Vec<_> = entries.iter().take(100).map(|(k, _)| k.clone()).collect();
            for key in keys_to_remove {
                cache.remove(&key);
            }
        }
        cache.insert(key, data);
    }

    /// Get scraping metrics
    #[must_use]
    pub fn metrics(&self) -> &ScrapingMetrics {
        &self.metrics
    }

    /// Clear documentation cache
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }

    /// Get cache statistics
    pub async fn cache_stats(&self) -> DocsCacheStats {
        let cache = self.cache.read().await;
        DocsCacheStats {
            cache_size: cache.len(),
            successful_scrapes: self.metrics.successful_scrapes.load(Ordering::Relaxed),
            failed_scrapes: self.metrics.failed_scrapes.load(Ordering::Relaxed),
            cache_hit_rate: self.metrics.cache_hit_rate(),
            success_rate: self.metrics.success_rate(),
            methods_scraped: self.metrics.methods_scraped.load(Ordering::Relaxed),
        }
    }
}

impl Default for DocsScrapingEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Documentation cache statistics
#[derive(Debug, Clone)]
pub struct DocsCacheStats {
    /// Current cache size
    pub cache_size: usize,
    /// Number of successful scrapes
    pub successful_scrapes: u64,
    /// Number of failed scrapes
    pub failed_scrapes: u64,
    /// Cache hit rate
    pub cache_hit_rate: f64,
    /// Scraping success rate
    pub success_rate: f64,
    /// Total methods scraped
    pub methods_scraped: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_docs_engine_creation() {
        let engine = DocsScrapingEngine::new();
        assert_eq!(engine.metrics().success_rate(), 0.0);
    }

    #[test]
    fn test_url_generation() {
        let engine = DocsScrapingEngine::new();
        let urls = engine.generate_documentation_urls("tokio", "Runtime");

        assert!(!urls.is_empty());
        assert!(urls.iter().any(|url| url.contains("struct.Runtime.html")));
        assert!(urls.iter().any(|url| url.contains("tokio")));
    }

    #[test]
    fn test_parameter_parsing() {
        let engine = DocsScrapingEngine::new();
        let signature = "fn test(x: i32, mut y: String, z: &str) -> bool";
        let params = engine.parse_parameters_from_signature(signature);

        assert_eq!(params.len(), 3);
        assert_eq!(params[0].name, "x");
        assert_eq!(params[0].param_type, "i32");
        assert_eq!(params[1].name, "y");
        assert!(params[1].is_mutable);
        assert_eq!(params[2].name, "z");
        assert_eq!(params[2].param_type, "&str");
    }

    #[test]
    fn test_return_type_extraction() {
        let engine = DocsScrapingEngine::new();

        let signature1 = "fn test() -> bool";
        assert_eq!(
            engine.extract_return_type_from_signature(signature1),
            Some("bool".to_string())
        );

        let signature2 = "fn test() -> Result<String, Error> where";
        assert_eq!(
            engine.extract_return_type_from_signature(signature2),
            Some("Result<String, Error>".to_string())
        );

        let signature3 = "fn test()";
        assert_eq!(engine.extract_return_type_from_signature(signature3), None);
    }

    #[test]
    fn test_version_extraction() {
        let engine = DocsScrapingEngine::new();

        let url1 = "https://docs.rs/tokio/1.0.0/tokio/struct.Runtime.html";
        assert_eq!(
            engine.extract_version_from_url(url1),
            Some("1.0.0".to_string())
        );

        let url2 = "https://docs.rs/tokio/latest/tokio/struct.Runtime.html";
        assert_eq!(engine.extract_version_from_url(url2), None);
    }

    #[test]
    fn test_similarity_calculations() {
        let engine = DocsScrapingEngine::new();

        // Test identical strings
        assert_eq!(engine.calculate_similarity("test", "test"), 1.0);

        // Test similar strings
        let sim1 = engine.calculate_similarity("method_name", "method_nam");
        assert!(sim1 > 0.8);

        // Test different strings
        let sim2 = engine.calculate_similarity("completely", "different");
        assert!(sim2 < 0.5);
    }

    #[test]
    fn test_code_complexity_analysis() {
        let engine = DocsScrapingEngine::new();

        let simple_code = "let x = 5;";
        assert_eq!(engine.analyze_code_complexity(simple_code), 1);

        let complex_code = r#"
            fn complex_function() {
                if condition {
                    match value {
                        Some(x) => loop {
                            if x > 0 {
                                break;
                            }
                        },
                        None => {}
                    }
                }
            }
        "#;
        assert!(engine.analyze_code_complexity(complex_code) > 3);
    }

    #[test]
    fn test_compilation_status_estimation() {
        let engine = DocsScrapingEngine::new();

        let good_code = "fn main() { println!(\"Hello\"); }";
        assert!(engine.estimate_compilation_status(good_code));

        let bad_code = "fn main() { // This won't compile }";
        assert!(!engine.estimate_compilation_status(bad_code));

        let unbalanced_code = "fn main() { {{{ }";
        assert!(!engine.estimate_compilation_status(unbalanced_code));
    }

    #[test]
    fn test_metrics_operations() {
        let metrics = ScrapingMetrics::default();

        metrics.record_success(5);
        metrics.record_failure();
        metrics.record_cache_hit();

        assert_eq!(metrics.successful_scrapes.load(Ordering::Relaxed), 1);
        assert_eq!(metrics.failed_scrapes.load(Ordering::Relaxed), 1);
        assert_eq!(metrics.cache_hits.load(Ordering::Relaxed), 1);
        assert_eq!(metrics.methods_scraped.load(Ordering::Relaxed), 5);
        assert_eq!(metrics.success_rate(), 0.5);
    }

    #[tokio::test]
    async fn test_cache_operations() {
        let engine = DocsScrapingEngine::new();

        // Create test data
        let crate_info = CrateInfo::new("test", "1.0.0", "https://example.com");
        let docs_data = CachedDocsData::new(
            crate_info,
            vec![],
            vec![],
            vec![],
            DataSource::LocalAnalysis,
        );

        // Test caching
        engine.cache_docs("test_key".to_string(), docs_data).await;

        // Test retrieval
        let cached = engine.get_cached_docs("test_key").await;
        assert!(cached.is_some());

        let stats = engine.cache_stats().await;
        assert_eq!(stats.cache_size, 1);
    }
}
