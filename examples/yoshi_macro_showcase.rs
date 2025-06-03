/* examples/yoshi_macro_showcase.rs */
#![warn(missing_docs)]
#![deny(unsafe_code)]
//! **Brief:** Showcase of the `yoshi!` macro and facade crate functionality.
//!
//! This example demonstrates the power and convenience of the `yoshi!` macro
//! provided by the main `yoshi` facade crate, showing all three creation modes
//! and chaining capabilities.
//!
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + yoshi! macro demonstration
//!  - Message-based error creation
//!  - Kind-based error creation
//!  - Foreign error wrapping
//!  - Attribute chaining within macro
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Author:** Lord Xyn

use std::io::{self, ErrorKind};
use yoshi::{yoshi, Arc, HatchExt, Result, Yoshi, YoshiKind};

/// Demonstrates all three modes of the `yoshi!` macro.
fn demonstrate_yoshi_macro_modes() {
    println!("🦕 Yoshi Macro Showcase");
    println!("======================");

    // Mode 1: Message-based creation (creates Internal YoshiKind)
    println!("\n1. Message-based error creation:");
    let err1 = yoshi!(message: "Database connection failed during startup");
    println!("   Error: {}", err1);
    println!("   Kind: {:?}", err1.kind());

    // Mode 2: Kind-based creation (direct YoshiKind specification)
    println!("\n2. Kind-based error creation:");
    let err2 = yoshi!(kind: YoshiKind::NotFound {
        resource_type: "User Profile".into(),
        identifier: "user_12345".into(),
        search_locations: Some(vec!["/db/users".into(), "/cache/profiles".into()]),
    });
    println!("   Error: {}", err2);

    // Mode 3: Foreign error wrapping
    println!("\n3. Foreign error wrapping:");
    let io_error = io::Error::new(ErrorKind::PermissionDenied, "cannot write to log file");
    let err3 = yoshi!(error: io_error);
    println!("   Error: {}", err3);
    println!(
        "   Is Foreign: {:?}",
        matches!(err3.kind(), YoshiKind::Foreign { .. })
    );
}

/// Demonstrates macro attribute chaining capabilities.
fn demonstrate_macro_chaining() {
    println!("\n🔗 Macro Chaining Showcase");
    println!("==========================");

    // Message with chained attributes
    println!("\n1. Message with metadata and suggestion:");
    let err1 = yoshi!(message: "Configuration validation failed",
        with_metadata = ("config_file", "/etc/app.conf"),
        with_suggestion = "Check configuration file syntax and permissions"
    );
    println!("   Error: {}", err1);
    println!("   Suggestion: {:?}", err1.suggestion());

    // Verify metadata
    let metadata = &err1.primary_context().unwrap().metadata;
    println!(
        "   Config file: {:?}",
        metadata.get(&Arc::from("config_file"))
    );

    // Kind with multiple attributes
    println!("\n2. Network error with shell and priority:");
    #[derive(Debug, Clone)]
    struct RequestContext {
        id: String,
        retry_count: u32,
    }

    let err2 = yoshi!(kind: YoshiKind::Network {
        message: "API endpoint unreachable".into(),
        source: None,
        error_code: Some(503),
    },
    with_shell = RequestContext { id: "req_001".to_string(), retry_count: 2 },
    with_priority = 200,
    with_suggestion = "Retry after checking service health"
    );

    println!("   Error: {}", err2);
    println!("   Priority: {}", err2.primary_context().unwrap().priority);
    println!("   Has shell: {}", err2.shell::<RequestContext>().is_some());
    if let Some(ctx) = err2.shell::<RequestContext>() {
        println!("   Request ID: {}", ctx.id);
    }

    // Foreign error with attributes
    println!("\n3. I/O error with enhanced context:");
    let parse_error = "not_a_number".parse::<i32>().unwrap_err();
    let err3 = yoshi!(error: parse_error,
        with_metadata = ("input_value", "not_a_number"),
        with_metadata = ("expected_type", "i32"),
        with_suggestion = "Provide a valid integer value"
    );

    println!("   Error: {}", err3);
    println!("   Suggestion: {:?}", err3.suggestion());
}

/// Demonstrates error propagation with the macro in a realistic workflow.
fn realistic_workflow_example() -> Result<String> {
    println!("\n⚙️  Realistic Workflow Example");
    println!("==============================");

    // Simulate configuration loading
    fn load_config() -> Result<String> {
        Err(yoshi!(message: "Config file corrupted",
            with_metadata = ("file_path", "/etc/app/database.conf"),
            with_suggestion = "Restore from backup or recreate configuration"
        ))
    }

    // Simulate database connection
    fn connect_database(config: &str) -> Result<String> {
        let timeout_error = io::Error::new(ErrorKind::TimedOut, "connection timeout");
        Err(yoshi!(error: timeout_error,
            with_metadata = ("database_host", "db.example.com"),
            with_metadata = ("timeout_duration", "30s")
        ))
        .context("Failed to establish database connection")
        .help("Check database server status and network connectivity")
    }

    // Simulate user authentication
    fn authenticate_user(db_connection: &str) -> Result<String> {
        Err(yoshi!(kind: YoshiKind::Validation {
            field: "user_credentials".into(),
            message: "Invalid authentication token".into(),
            expected: Some("Valid JWT token".into()),
            actual: Some("expired_token_abc123".into()),
        },
        with_metadata = ("auth_service", "oauth2"),
        with_priority = 180
        ))
        .context("User authentication failed during login")
    }

    // Main workflow with error propagation
    let config = load_config().context("Application initialization failed")?;

    let db = connect_database(&config).context("Database layer initialization failed")?;

    let user = authenticate_user(&db).context("Security layer validation failed")?;

    Ok(format!(
        "Workflow completed successfully for user: {}",
        user
    ))
}

fn main() {
    // Run demonstrations
    demonstrate_yoshi_macro_modes();
    demonstrate_macro_chaining();

    // Run realistic workflow and handle error
    match realistic_workflow_example() {
        Ok(result) => println!("\n✅ {}", result),
        Err(error) => {
            println!("\n❌ Workflow failed with comprehensive error trace:");
            println!("{}", error);

            // Additional error analysis
            println!("\n📊 Error Analysis:");
            println!("   Instance ID: {}", error.instance_id());
            println!("   Severity: {}", error.severity());
            println!("   Is Transient: {}", error.is_transient());

            let analysis = error.analyze_contexts();
            println!("   Contexts: {}", analysis.total_contexts);
            println!("   Has Suggestions: {}", analysis.has_suggestions);
            println!("   Metadata Entries: {}", analysis.metadata_entries);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_based_creation() {
        let err = yoshi!(message: "test error");
        assert!(matches!(err.kind(), YoshiKind::Internal { .. }));
        assert!(err.to_string().contains("test error"));
    }

    #[test]
    fn test_kind_based_creation() {
        let err = yoshi!(kind: YoshiKind::NotFound {
            resource_type: "Test".into(),
            identifier: "test_id".into(),
            search_locations: None,
        });
        assert!(matches!(err.kind(), YoshiKind::NotFound { .. }));
    }

    #[test]
    fn test_foreign_error_creation() {
        let io_err = io::Error::new(ErrorKind::Other, "test io error");
        let err = yoshi!(error: io_err);
        assert!(matches!(err.kind(), YoshiKind::Foreign { .. }));
    }

    #[test]
    fn test_macro_with_metadata() {
        let err = yoshi!(message: "test",
            with_metadata = ("key", "value"),
            with_suggestion = "try again"
        );

        assert!(err.suggestion().is_some());
        assert_eq!(err.suggestion().unwrap(), "try again");

        let metadata = &err.primary_context().unwrap().metadata;
        assert_eq!(
            metadata.get(&Arc::from("key")).map(|s| s.as_ref()),
            Some("value")
        );
    }

    #[test]
    fn test_macro_with_shell_and_priority() {
        #[derive(Debug, PartialEq)]
        struct TestShell(u32);

        let err = yoshi!(message: "test",
            with_shell = TestShell(42),
            with_priority = 150
        );

        assert_eq!(err.primary_context().unwrap().priority, 150);
        assert_eq!(err.shell::<TestShell>().unwrap().0, 42);
    }

    #[test]
    fn test_realistic_workflow_fails() {
        let result = realistic_workflow_example();
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert!(error
            .to_string()
            .contains("Application initialization failed"));
    }
}
