/* examples/test_derive_functionality.rs */
//! **Brief:** Example demonstrating YoshiError derive macro functionality.
//!
//! **Module Classification:** Standard
//! **Complexity Level:** Low
//! **API Stability:** Experimental
//!
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + YoshiError derive macro demonstration
//!  - Basic enum derivation
//!  - Field usage verification
//!  - Custom conversion testing
//!  - Documentation integration
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//!
//! ## Mathematical Properties
//!
//! **Algorithmic Complexity:**
//! - Time Complexity: O(1) for error operations
//! - Space Complexity: O(1) for error storage
//! - Concurrency Safety: Thread-safe error handling
//!
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **Author:** Lord Xyn
// **License:** MIT OR Apache-2.0
// **License File:** /LICENSE
// **Effective Date:** 2025-05-30 | **Open Source Release**
// **Contact:** LordXyn@proton.me

use std::error::Error;
use yoshi_derive::YoshiError;

/// Simple test error to verify derive macro functionality.
#[derive(Debug, YoshiError)]
pub enum SimpleError {
    /// Basic I/O error
    #[yoshi(kind = "Io")]
    Io(std::io::Error),

    /// Network error with struct fields
    #[yoshi(kind = "Network")]
    Network {
        #[yoshi(doc = "Network endpoint")]
        endpoint: String,
    },
}

fn main() {
    let io_err = SimpleError::Io(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Test file not found",
    ));

    println!("Simple I/O error: {}", io_err);

    let net_err = SimpleError::Network {
        endpoint: "https://example.com".to_string(),
    };

    println!("Simple network error: {}", net_err);
    println!("Example completed successfully!");
}
