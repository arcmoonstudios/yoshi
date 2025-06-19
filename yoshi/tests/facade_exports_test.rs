/* yoshi/tests/facade_exports_test.rs */
//! Test that all necessary types are properly exported through the yoshi facade

use yoshi::*;

#[tokio::test]
async fn test_facade_exports_complete() {
    // Test that all auto-correction types are available through 'use yoshi::*'

    // Core error types should be available
    let _result: Hatch<String> = Ok("test".to_string());

    // Auto-correction system should be available
    let _system = YoshiACSystem::new();

    // Configuration types should be available
    let _config = SystemConfig::default();

    // Diagnostic types should be available
    let _diagnostic = CompilerDiagnostic::new("test_id", "Test message", DiagnosticLevel::Warning);

    // Span types should be available
    let _span = DiagnosticSpan::new(
        std::path::PathBuf::from("test.rs"),
        0,
        10,
        1,
        1,
        1,
        10,
        "test code".to_string(),
    );

    // Correction types should be available
    let _correction = ProjectCorrection::new(std::path::PathBuf::from("test.rs"), _diagnostic);

    // Safety levels should be available
    let _safety = SafetyLevel::Safe;

    // Correction strategies should be available
    let _strategy = CorrectionStrategy::Generic {
        description: "Test strategy".to_string(),
    };

    // Proposal types should be available
    let _proposal = CorrectionProposal::new(
        _strategy,
        0.9,
        SafetyLevel::Safe,
        Some("Test proposal".to_string()),
    );

    println!("✅ All types properly exported through yoshi facade");
}

#[tokio::test]
async fn test_yoshi_af_macro_available() {
    // Test that yoshi_af! macro is available
    yoshi_af! {
        pub fn test_function(input: Option<String>) -> Hatch<String> {
            Ok(input.unwrap_or_default())
        }
    }

    let result = test_function(Some("test".to_string())).expect("test_function should succeed");
    assert_eq!(result, "test");

    println!("✅ yoshi_af! macro properly exported");
}

#[test]
fn test_derive_macro_available() {
    // Test that YoshiError derive macro is available
    #[derive(Debug, YoshiError)]
    enum TestError {
        #[yoshi(display = "Test error occurred")]
        TestVariant,
    }

    let _error = TestError::TestVariant;
    println!("✅ YoshiError derive macro properly exported");
}

#[test]
fn test_convenience_types_available() {
    // Test that convenience types are available when feature is enabled
    #[cfg(feature = "convenience")]
    {
        let _map: DashMap<String, i32> = DashMap::new();
        let _vec: SmallVec<[i32; 4]> = SmallVec::new();
        let _regex = Regex::new(r"test").unwrap();
    }

    println!("✅ Convenience types properly exported");
}

#[test]
fn test_error_handling_traits() {
    // Test that error handling types are available
    let _result: Hatch<String> = Ok("test".to_string());

    // Test that the yoshi! macro is available
    let _macro_error = yoshi!(message: "Test macro error");

    println!("✅ Error handling types properly exported");
}
