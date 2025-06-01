//! # Yoshi Benchmarks Package
//!
//! This package contains comprehensive performance benchmarks for the entire
//! Yoshi error handling framework ecosystem.

// Re-export the comprehensive_comparison module from tests
#[path = "../tests/comprehensive_comparison.rs"]
pub mod comprehensive_comparison;

pub fn main() {
    println!("Yoshi Benchmarks Package - Use `cargo bench` to run benchmarks");
}
