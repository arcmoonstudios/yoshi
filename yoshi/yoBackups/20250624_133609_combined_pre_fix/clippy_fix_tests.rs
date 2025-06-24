//! **Comprehensive tests for `ClippyFixEngine`**
//!
//! Tests the integration of official Clippy documentation with our
//! yoshi-derive capabilities for automated code fixing.

#[cfg(feature = "auto-fix")]
use yoshi::auto_fix::unclipped::{test_clippy_fix_engine, ClippyFixEngine};

#[cfg(feature = "auto-fix")]
#[tokio::test]
async fn test_clippy_fix_engine_comprehensive() {
    // Initialize tracing for test output

    // Run the comprehensive test
    let result = test_clippy_fix_engine();
    assert!(
        result.is_ok(),
        "ClippyFixEngine test should pass: {result:?}"
    );
}

#[cfg(feature = "auto-fix")]
#[tokio::test]
async fn test_uninlined_format_args_fix() {
    let mut engine = ClippyFixEngine::new().expect("Failed to create ClippyFixEngine");

    // Test the specific pattern from yoFixME.txt
    let test_code = r#"println!("Hello {}", name);"#;
    let fixed_code = engine
        .apply_clippy_fixes(test_code)
        .expect("Failed to apply fixes");

    // Should convert to inline format
    assert!(
        fixed_code.contains("{name}"),
        "Should inline the format argument: {fixed_code}"
    );
}

#[cfg(feature = "auto-fix")]
#[tokio::test]
async fn test_assigning_clones_fix() {
    let mut engine = ClippyFixEngine::new().expect("Failed to create ClippyFixEngine");

    // Test the specific pattern from yoFixME.txt
    let test_code = r"target = source.clone();";
    let fixed_code = engine
        .apply_clippy_fixes(test_code)
        .expect("Failed to apply fixes");

    // Should convert to clone_from
    assert!(
        fixed_code.contains("clone_from"),
        "Should use clone_from instead of clone: {fixed_code}"
    );
}

#[cfg(feature = "auto-fix")]
#[tokio::test]
async fn test_redundant_closure_fix() {
    let mut engine = ClippyFixEngine::new().expect("Failed to create ClippyFixEngine");

    // Test the specific pattern from yoFixME.txt
    let test_code = r".map(|s| s.to_string())";
    let fixed_code = engine
        .apply_clippy_fixes(test_code)
        .expect("Failed to apply fixes");

    // Should convert to method reference
    assert!(
        fixed_code.contains("ToString::to_string"),
        "Should use method reference: {fixed_code}"
    );
}

#[cfg(feature = "auto-fix")]
#[tokio::test]
async fn test_indexing_slicing_safety() {
    let mut engine = ClippyFixEngine::new().expect("Failed to create ClippyFixEngine");

    // Test the safety-critical pattern from yoFixME.txt
    let test_code = r"lines[issue.line_number - 1] = value;";
    let fixed_code = engine
        .apply_clippy_fixes(test_code)
        .expect("Failed to apply fixes");

    // Should use safe indexing
    assert!(
        fixed_code.contains("get_mut") || fixed_code.contains("if let"),
        "Should use safe indexing: {fixed_code}"
    );
}

#[cfg(feature = "auto-fix")]
#[tokio::test]
async fn test_multiple_patterns_integration() {
    let mut engine = ClippyFixEngine::new().expect("Failed to create ClippyFixEngine");

    // Test multiple patterns together
    let test_code = r#"
        fn example() {
            println!("Debug: {}", value);
            target = source.clone();
            let result = items.iter().map(|s| s.to_string()).collect();
        }
    "#;

    let fixed_code = engine
        .apply_clippy_fixes(test_code)
        .expect("Failed to apply fixes");
    let stats = engine.get_stats();

    // Verify multiple fixes were applied
    assert!(stats.total_fixes_applied > 0, "Should apply multiple fixes");
    assert!(
        fixed_code.contains("{value}")
            || fixed_code.contains("clone_from")
            || fixed_code.contains("ToString::to_string"),
        "Should apply at least one fix: {fixed_code}"
    );
}

#[cfg(feature = "auto-fix")]
#[tokio::test]
async fn test_new_safety_patterns() {
    let mut engine = ClippyFixEngine::new().expect("Failed to create ClippyFixEngine");

    // Test bool_comparison pattern
    let bool_test = r#"if flag == true { println!("true"); }"#;
    let bool_fixed = engine
        .apply_clippy_fixes(bool_test)
        .expect("Failed to apply bool fixes");
    assert!(
        bool_fixed.contains("if flag {") || !bool_fixed.contains("== true"),
        "Should fix bool comparison: {bool_fixed}"
    );

    // Test len_zero pattern
    let len_test = r#"if vec.len() == 0 { println!("empty"); }"#;
    let len_fixed = engine
        .apply_clippy_fixes(len_test)
        .expect("Failed to apply len fixes");
    assert!(
        len_fixed.contains("is_empty()") || !len_fixed.contains("len() == 0"),
        "Should fix len zero: {len_fixed}"
    );

    // Test clone_on_copy pattern
    let clone_test = r"let index: usize = 42; let copied = index.clone();";
    let clone_fixed = engine
        .apply_clippy_fixes(clone_test)
        .expect("Failed to apply clone fixes");
    assert!(
        clone_fixed.contains("let copied = index;") || !clone_fixed.contains("index.clone()"),
        "Should fix clone on copy: {clone_fixed}"
    );

    // Test float_cmp pattern
    let float_test = r#"let a: f64 = 1.0; let b: f64 = 2.0; if a == b { println!("equal"); }"#;
    let float_fixed = engine
        .apply_clippy_fixes(float_test)
        .expect("Failed to apply float fixes");
    assert!(
        float_fixed.contains("EPSILON") || !float_fixed.contains("a == b"),
        "Should fix float comparison: {float_fixed}"
    );
}

#[cfg(not(feature = "auto-fix"))]
#[tokio::test]
async fn test_auto_fix_feature_disabled() {
    // When auto-fix feature is disabled, this test should still pass
    // but the functionality won't be available
    println!("auto-fix feature is disabled, skipping ClippyFixEngine tests");
}