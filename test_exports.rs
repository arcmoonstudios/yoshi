// Test file to verify all required exports are available

use yoshi::{yoshi, yoshi_af, YoshiError, Oops};

fn main() {
    // Test that all exports are accessible
    println!("Testing exports...");

    // Test yoshi! macro
    let err1 = yoshi!(message: "Test error");
    println!("yoshi! macro works: {}", err1);

    // Test Oops enum
    let oops = Oops::ConfigMissing {
        file_path: "test.conf".to_string(),
    };
    println!("Oops enum works: {:?}", oops);

    // Test YoshiError derive (should be available for custom derives)
    #[derive(Debug, YoshiError)]
    enum TestError {
        #[yoshi(display = "Test error")]
        Test,
    }

    let test_err = TestError::Test;
    println!("YoshiError derive works: {:?}", test_err);

    // Test yoshi_af! macro
    // Note: This should be defined as a procedural macro, let's see if we can use it
    println!("All exports verified successfully!");
}
