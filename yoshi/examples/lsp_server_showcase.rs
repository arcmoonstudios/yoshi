/* yoshi/examples/lsp_server_showcase.rs */
//! **Brief:** Showcase of Yoshi LSP server integration for real-time optimization suggestions.
//!
//! This example demonstrates how to start and use the Yoshi LSP server through the main
//! yoshi facade. The LSP server provides real-time optimization suggestions, code actions,
//! and hover information for VS Code and other LSP-compatible editors.
//!
//! ## Usage
//!
//! ```bash
//! # Start the LSP server (for VS Code integration)
//! cargo run --example lsp_server_showcase --features lsp-integration
//!
//! # Test the LSP server functionality
//! cargo test --example lsp_server_showcase --features lsp-integration
//! ```
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn

// Import everything through the yoshi facade - the only import needed!
use yoshi::Hatch;

#[cfg(feature = "lsp-integration")]
#[tokio::main]
async fn main() -> Hatch<()> {
    println!("🚀 Yoshi LSP Server Showcase 🚀\n");

    // Show LSP server capabilities
    demonstrate_lsp_capabilities().await?;

    // Start the LSP server (this would normally be called by VS Code)
    println!("🔧 Starting Yoshi LSP Server...");
    println!("📝 Note: In production, this would be started by VS Code automatically");
    println!("💡 To integrate with VS Code, add the Yoshi extension to your editor");

    // For demonstration, we'll show how to start the server
    // In real usage, VS Code would call this automatically
    println!("\n🎯 LSP Server Configuration:");
    let config = YoshiLspConfig::default();
    println!(
        "   • Optimization detection: {}",
        config.enable_optimization_detection
    );
    println!("   • Code actions: {}", config.enable_code_actions);
    println!("   • Hover info: {}", config.enable_hover_info);
    println!(
        "   • Min confidence: {:.1}%",
        config.min_confidence_threshold * 100.0
    );

    println!("\n✅ Yoshi LSP Server showcase completed!");
    println!("🔗 To use with VS Code, install the Yoshi extension and restart your editor");

    Ok(())
}

#[cfg(not(feature = "lsp-integration"))]
fn main() {
    println!("❌ LSP integration feature not enabled!");
    println!("💡 Run with: cargo run --example lsp_server_showcase --features lsp-integration");
}

#[cfg(feature = "lsp-integration")]
async fn demonstrate_lsp_capabilities() -> Hatch<()> {
    println!("🎯 Yoshi LSP Server Capabilities");
    println!("================================\n");

    println!("📋 **Real-time Features:**");
    println!("   🔍 Optimization detection as you type");
    println!("   ⚡ Instant code actions for improvements");
    println!("   💡 Hover tooltips with optimization details");
    println!("   📊 Performance impact estimates");
    println!("   🛡️ Safety validation for all suggestions\n");

    println!("🚀 **Optimization Types:**");
    println!("   • Vec::new() → Vec::with_capacity() optimization");
    println!("   • .unwrap() → .expect() or ? operator optimization");
    println!("   • String cloning optimization (coming soon)");
    println!("   • Iterator collect() optimization (coming soon)");
    println!("   • Borrowing optimization (coming soon)\n");

    println!("🎨 **VS Code Integration:**");
    println!("   • Real-time squiggly underlines for optimization opportunities");
    println!("   • Quick fix suggestions with 🚀 Yoshi branding");
    println!("   • Hover information with performance impact");
    println!("   • Status bar showing optimization statistics");
    println!("   • Configuration panel for customizing behavior\n");

    // Demonstrate optimization detection
    demonstrate_optimization_detection().await?;

    Ok(())
}

#[cfg(feature = "lsp-integration")]
async fn demonstrate_optimization_detection() -> Hatch<()> {
    println!("🔬 **Optimization Detection Demo:**");
    println!("==================================\n");

    // Example code that would trigger optimizations
    let sample_code = r#"
fn example_function() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut items = Vec::new();
    items.push("first".to_string());
    items.push("second".to_string());
    items.push("third".to_string());

    let maybe_value = Some("test".to_string());
    let value = maybe_value.unwrap();

    Ok(items)
}
"#;

    println!("📝 **Sample Code:**");
    println!("```rust{}", sample_code);
    println!("```\n");

    // Use the optimization engine to detect opportunities
    let engine = yoshi_deluxe::OptimizationEngine::new();
    let opportunities = engine.detect_optimization_opportunities(sample_code);

    println!("🎯 **Detected Optimizations:**");
    for (i, opp) in opportunities.iter().enumerate() {
        let impact_emoji = match opp.performance_impact {
            yoshi_deluxe::optimization::PerformanceImpact::High => "🚀",
            yoshi_deluxe::optimization::PerformanceImpact::Medium => "⚡",
            yoshi_deluxe::optimization::PerformanceImpact::Low => "💡",
        };

        println!("   {}. {} {}", i + 1, impact_emoji, opp.description);
        println!(
            "      📍 Location: line {}, column {}",
            opp.location.line, opp.location.column
        );
        println!("      🎯 Confidence: {:.1}%", opp.confidence * 100.0);
        println!("      📈 Impact: {:?}", opp.performance_impact);
        println!("      💡 Suggestion: {}", opp.description);
        println!();
    }

    if opportunities.is_empty() {
        println!("   ✅ No optimization opportunities detected (code is already optimal!)");
    } else {
        println!(
            "📊 **Summary:** {} optimization opportunities detected",
            opportunities.len()
        );
        println!("🔧 **In VS Code:** These would appear as:");
        println!("   • Squiggly underlines in the editor");
        println!("   • Quick fix suggestions in the context menu");
        println!("   • Hover tooltips with detailed information");
    }

    println!();
    Ok(())
}

// Example functions that would benefit from Yoshi optimizations
#[allow(dead_code)]
fn example_with_optimizations() -> Hatch<Vec<String>> {
    // This Vec::new() would be optimized to Vec::with_capacity(3)
    let mut suggestions = Vec::new();
    suggestions.push("Use Vec::with_capacity()".to_string());
    suggestions.push("Replace .unwrap() with .expect()".to_string());
    suggestions.push("Consider using ? operator".to_string());

    // This .unwrap() would be optimized to .expect() or ?
    let config = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "default_config.toml".to_string());
    suggestions.push(format!("Config loaded from: {config}"));

    Ok(suggestions)
}

#[allow(dead_code)]
fn already_optimized_function() -> Hatch<Vec<String>> {
    // This is already optimized - no suggestions needed
    let mut items = Vec::with_capacity(2);
    let result = std::env::var("PATH").expect("PATH environment variable should be set");
    items.push(result);
    items.push("Already optimized!".to_string());
    Ok(items)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "lsp-integration")]
    #[tokio::test]
    async fn test_lsp_capabilities_demo() {
        let result = demonstrate_lsp_capabilities().await;
        assert!(result.is_ok(), "LSP capabilities demo should succeed");
    }

    #[cfg(feature = "lsp-integration")]
    #[tokio::test]
    async fn test_optimization_detection_demo() {
        let result = demonstrate_optimization_detection().await;
        assert!(result.is_ok(), "Optimization detection demo should succeed");
    }

    #[test]
    fn test_example_functions_compile() {
        // Test that our example functions compile correctly
        let result1 = example_with_optimizations();
        assert!(result1.is_ok());

        let result2 = already_optimized_function();
        assert!(result2.is_ok());
    }

    #[cfg(feature = "lsp-integration")]
    #[test]
    fn test_lsp_config_creation() {
        let config = YoshiLspConfig::default();
        assert!(config.enable_optimization_detection);
        assert!(config.enable_code_actions);
        assert!(config.enable_hover_info);
        assert!(config.min_confidence_threshold > 0.0);
    }
}
