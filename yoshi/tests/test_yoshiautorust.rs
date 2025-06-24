//! **`YoshiAutoRust` Test Example**
//!
//! This example demonstrates the `YoshiAF` autonomous fixing system.

use yoshi::{auto_fix::test_yoshi_af, Hatch};

fn main() -> Hatch<()> {
    tracing::info!("🤖 Testing YoshiAutoRust with #![yoshi(auto-correct)] Detection...");

    // Test the YoshiAutoRust system
    match test_yoshi_af() {
        Ok(()) => {
            tracing::info!("✅ YoshiAutoRust test completed successfully!");
        }
        Err(e) => {
            tracing::error!("❌ YoshiAutoRust test failed: {}", e);
        }
    }

    Ok(())
}
