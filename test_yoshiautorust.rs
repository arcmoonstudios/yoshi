use yoshi_std::test_yoshiautorust_generator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 Testing YoshiAutoRust with #![yoshi(auto-correct)] Detection...");
    
    // Test the YoshiAutoRust system
    match test_yoshiautorust_generator() {
        Ok(()) => {
            println!("✅ YoshiAutoRust test completed successfully!");
        }
        Err(e) => {
            println!("❌ YoshiAutoRust test failed: {}", e);
        }
    }
    
    Ok(())
}
