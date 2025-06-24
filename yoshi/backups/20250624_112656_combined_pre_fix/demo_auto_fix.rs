/* examples/demo_auto_fix.rs */
//! Demo example for `YoshiAutoFix` autonomous code fixing
//! #![yoshi(auto-fix)]

use yoshi::*;

/// Demo function with issues that `YoshiAutoFix` can fix
pub fn demo_function_with_issues() {
    // Unused variable that should be removed

    // println! that should become tracing::info!
    tracing::info!("🎉 This is a success message");

    // println! that should become tracing::error!
    tracing::error!("🚨 This is an error message");

    // println! that should become tracing::warn!
    tracing::warn!("⚠️ This is a warning message");

    // println! that should become tracing::debug!
    tracing::debug!("🔍 This is a debug message");

    // Another unused variable
}

/// Function with unnecessary Ok(()) wrapping
pub fn function_with_unnecessary_wrap() -> Result<(), AnyError> {
    // This should be simplified to just ()
    Ok(())
}

/// Another function with unnecessary wrapping
pub fn another_unnecessary_wrap() -> Result<(), AnyError> {
    // This should also be simplified
    Ok(())
}

/// Function that demonstrates needless borrow (simplified detection)
pub fn needless_borrow_demo() {
    let data = vec![1, 2, 3];
    let _cloned = &data.clone(); // This pattern might be detected
}

fn main() -> Result<(), AnyError> {
    tracing::info!("🤖 YoshiAutoFix Demo - Testing Autonomous Code Fixing");

    // Test the demo functions
    demo_function_with_issues();
    function_with_unnecessary_wrap()?;
    another_unnecessary_wrap()?;
    needless_borrow_demo();

    tracing::info!("✅ Demo completed! Add #![yoshi(auto-fix)] to enable autonomous fixing.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_function() {
        // More unused variables for testing

        // More println! statements to convert
        tracing::info!("✅ Test passed successfully");
        tracing::error!("❌ Test failed with error");

        demo_function_with_issues();
    }
}
