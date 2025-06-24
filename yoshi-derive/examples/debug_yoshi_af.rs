/* yoshi/yoshi-derive/examples/debug_yoshi_af.rs */

//! Debug example to test `yoshi_af`! macro syntax

use yoshi_derive::yoshi_af;

fn main() {
    tracing::info!("Testing yoshi_af! macro...");

    // Test 1: Simple function
    yoshi_af! {
        fn simple_test() -> Result<String, Box<dyn std::error::Error>> {
            Ok("test".to_string())
        }
    }

    // Test 2: Let's debug this step by step
    // First, try without pub
    yoshi_af! {
        fn private_test() -> Result<String, Box<dyn std::error::Error>> {
            Ok("private test".to_string())
        }
    }

    // Now try with pub - ULTIMATE DEBUGGING!
    yoshi_af! {
        pub fn public_test() -> Result<String, Box<dyn std::error::Error>> {
            Ok("public test".to_string())
        }
    }

    // Call the example functions to demonstrate their usage
    if let Ok(result) = simple_test() {
        tracing::info!("Simple test result: {result}");
    }

    if let Ok(result) = private_test() {
        tracing::info!("Private test result: {result}");
    }

    if let Ok(result) = public_test() {
        tracing::info!("Public test result: {result}");
    }

    tracing::info!("All yoshi_af! macro tests passed!");
}
