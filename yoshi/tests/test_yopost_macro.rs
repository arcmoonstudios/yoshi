/* examples/test_yopost_macro.rs */
//! Test example for the yopost! macro

use yoshi::*;

fn main() -> Hatch<()> {
    println!("🧪 Testing yopost! macro...");
    
    // Test message-based error creation
    let err = yopost!(message: "Something went wrong".into());
    println!("Message error: {err}");
    
    // Test formatted message
    let err = yopost!(message: "Failed to load {}", "config.toml");
    println!("Formatted error: {err}");
    
    // Test error wrapping
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let err = yopost!(error: io_err);
    println!("Wrapped error: {err}");
    
    println!("✅ All yopost! macro tests passed!");
    Ok(())
}
