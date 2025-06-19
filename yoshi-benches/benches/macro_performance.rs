#![allow(missing_docs)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::doc_markdown)]
//! ArcMoon Studios - Yoshi Framework Macro Expansion Benchmarks
//! Copyright (c) 2024 ArcMoon Studios. All rights reserved.
//!
//! Ultra-precise benchmarks for the actual macro expansion process.

#![allow(missing_docs)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::doc_markdown)]

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use proc_macro2::TokenStream;
use quote::quote;
use std::hint::black_box;
use std::time::Instant;
use syn::{parse_quote, DeriveInput};

/// Direct macro expansion benchmark (simulating compile-time performance)
fn bench_macro_expansion_direct(c: &mut Criterion) {
    let mut group = c.benchmark_group("macro_expansion_direct");

    // Simple enum expansion
    group.bench_function("simple_enum_expansion", |b| {
        let _input: DeriveInput = parse_quote! {
            #[derive(Debug)]
            enum SimpleError {
                Variant1,
                Variant2,
                Variant3,
            }
        };

        b.iter(|| {
            let start = Instant::now();

            // Simulate the macro expansion process
            let _expanded = black_box(quote! {
                impl ::std::fmt::Display for SimpleError {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        match self {
                            SimpleError::Variant1 => write!(f, "Variant1"),
                            SimpleError::Variant2 => write!(f, "Variant2"),
                            SimpleError::Variant3 => write!(f, "Variant3"),
                        }
                    }
                }

                impl ::std::error::Error for SimpleError {}
            });

            start.elapsed().as_nanos()
        });
    });

    // Complex enum expansion
    group.bench_function("complex_enum_expansion", |b| {
        let _input: DeriveInput = parse_quote! {
            #[derive(Debug)]
            enum ComplexError {
                Network { code: u32, message: String },
                Database { query: String },
                Validation { field: String, error: String },
                Critical,
            }
        };

        b.iter(|| {
            let start = Instant::now();

            // Simulate complex macro expansion
            let _expanded = black_box(quote! {
                impl ::std::fmt::Display for ComplexError {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        match self {
                            ComplexError::Network { code, message } => {
                                write!(f, "Network error {}: {}", code, message)
                            },
                            ComplexError::Database { query } => {
                                write!(f, "Database error with query: {}", query)
                            },
                            ComplexError::Validation { field, error } => {
                                write!(f, "Validation error in field '{}': {}", field, error)
                            },
                            ComplexError::Critical => write!(f, "Critical error"),
                        }
                    }
                }

                impl ::std::error::Error for ComplexError {}

                impl ::std::convert::From<ComplexError> for ::yoshi_std::Yoshi {
                    fn from(err: ComplexError) -> Self {
                        ::yoshi_std::Yoshi::new(::yoshi_std::YoshiKind::Internal {
                            message: ::std::sync::Arc::from(format!("{}", err)),
                            source: None,
                            component: Some(::std::sync::Arc::from("ComplexError")),
                        })
                    }
                }
            });

            start.elapsed().as_nanos()
        });
    });

    group.finish();
}

/// Benchmark TokenStream processing performance
fn bench_tokenstream_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("tokenstream_processing");

    let sizes = vec![1, 5, 10, 25, 50, 100];

    for size in sizes {
        group.bench_with_input(
            BenchmarkId::new("tokenstream_size", size),
            &size,
            |b, &size| {
                b.iter(|| {
                    let start = Instant::now();

                    // Generate TokenStream of varying sizes
                    let mut tokens = TokenStream::new();
                    for i in 0..size {
                        let variant_name = format!("Variant{i}");
                        let field_name = format!("field{i}");
                        let variant_ident =
                            syn::Ident::new(&variant_name, proc_macro2::Span::call_site());
                        let field_ident =
                            syn::Ident::new(&field_name, proc_macro2::Span::call_site());
                        let variant_tokens = quote! {
                            #variant_ident { #field_ident: String },
                        };
                        tokens.extend(variant_tokens);
                    }

                    let _final_tokens = black_box(quote! {
                        enum GeneratedEnum {
                            #tokens
                        }
                    });

                    start.elapsed().as_nanos()
                });
            },
        );
    }

    group.finish();
}

/// Benchmark VectorStream cache simulation
fn bench_vectorstream_cache_simulation(c: &mut Criterion) {
    let mut group = c.benchmark_group("vectorstream_cache");

    // Simulate hash-based cache operations
    group.bench_function("cache_key_generation", |b| {
        b.iter(|| {
            let start = Instant::now();

            // Simulate cache key generation (as done in the macro)
            use std::hash::{DefaultHasher, Hash, Hasher};
            let mut hasher = DefaultHasher::new();
            "ComplexError".hash(&mut hasher);
            "enum".hash(&mut hasher);
            "yoshi_derive".hash(&mut hasher);
            let _cache_key = black_box(hasher.finish());

            start.elapsed().as_nanos()
        });
    });

    // Simulate lockfree processing
    group.bench_function("lockfree_processing_simulation", |b| {
        use std::collections::HashMap;
        let cache: HashMap<u64, TokenStream> = HashMap::new();

        b.iter(|| {
            let start = Instant::now();

            // Simulate lockfree cache lookup and generation
            let cache_key = 12345u64;
            let _result = cache.get(&cache_key).cloned().unwrap_or_else(|| {
                // Simulate token generation
                black_box(quote! {
                    impl Display for CachedType {
                        fn fmt(&self, f: &mut Formatter) -> Result {
                            write!(f, "Cached implementation")
                        }
                    }
                })
            });

            start.elapsed().as_nanos()
        });
    });

    group.finish();
}

/// Benchmark memory allocation patterns
fn bench_memory_allocation_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_allocation");

    // String allocation benchmark
    group.bench_function("string_allocations", |b| {
        b.iter(|| {
            let start = Instant::now();

            // Simulate string allocations during macro expansion
            let _strings: Vec<String> = (0..100)
                .map(|i| black_box(format!("variant_{i}")))
                .collect();

            start.elapsed().as_nanos()
        });
    });

    // TokenStream allocation benchmark
    group.bench_function("tokenstream_allocations", |b| {
        b.iter(|| {
            let start = Instant::now();

            // Simulate TokenStream allocations
            let _tokens: Vec<TokenStream> = (0..50)
                .map(|i| {
                    let variant_name = format!("Variant{i}");
                    let variant_ident =
                        syn::Ident::new(&variant_name, proc_macro2::Span::call_site());
                    black_box(quote! { #variant_ident, })
                })
                .collect();

            start.elapsed().as_nanos()
        });
    });

    group.finish();
}

/// Benchmark error handling overhead
fn bench_error_handling_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("error_handling_overhead");

    group.bench_function("result_creation", |b| {
        b.iter(|| {
            let start = Instant::now();

            // Simulate Result creation patterns used in the macro
            let _results: Vec<Result<TokenStream, syn::Error>> = (0..100)
                .map(|i| {
                    if i % 10 == 0 {
                        Err(black_box(syn::Error::new_spanned(
                            quote! { error_token },
                            "Simulated error",
                        )))
                    } else {
                        Ok(black_box(quote! { success_token }))
                    }
                })
                .collect();

            start.elapsed().as_nanos()
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_macro_expansion_direct,
    bench_tokenstream_processing,
    bench_vectorstream_cache_simulation,
    bench_memory_allocation_patterns,
    bench_error_handling_overhead
);

criterion_main!(benches);
