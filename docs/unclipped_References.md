# Comprehensive Rust Attributes Reference

## Legend

**Severity Levels:**

- `✓` = Can use `#[allow()]`, `#[warn()]`, `#[deny()]`, `#[forbid()]`
- `◆` = Attribute only (no severity levels)
- `🔧` = Clippy lint with auto-fix available
- `⚠️` = Clippy lint that may have false positives

**Syntax Patterns:**

- `<attribute>` = Base attribute name
- `<attribute>(...)` = Attribute with parameters
- `<category>::<lint>` = Clippy lint in specific category

---

## Core Compiler Attributes

### Code Generation & Optimization

```rust
◆ #[inline]                    // Inline function calls
◆ #[inline(always)]            // Force inlining
◆ #[inline(never)]             // Prevent inlining
◆ #[cold]                      // Mark function as rarely called
◆ #[hot]                       // Mark function as frequently called
◆ #[no_mangle]                 // Prevent name mangling
◆ #[export_name = "name"]      // Set exported symbol name
◆ #[link_name = "name"]        // Set linked symbol name
◆ #[target_feature(enable = "feature")] // Enable target-specific features
◆ #[repr(C)]                   // C-compatible memory layout
◆ #[repr(transparent)]         // Transparent wrapper type
◆ #[repr(packed)]              // Pack struct fields without padding
◆ #[repr(align(n))]            // Align to n bytes
Memory Management & Safety
rust◆ #[may_dangle]                // Unsafe dropck attribute
◆ #[fundamental]               // Mark trait as fundamental
◆ #[lang = "item"]             // Language item marker
◆ #[marker]                    // Marker trait
◆ #[pin_project]               // Pin projection (external crate)
◆ #[box(value)]                // Box syntax (unstable)
Conditional Compilation
rust◆ #[cfg(condition)]            // Conditional compilation
◆ #[cfg_attr(condition, attr)] // Conditional attributes
◆ #[cfg(target_os = "linux")]  // OS-specific compilation
◆ #[cfg(target_arch = "x86_64")] // Architecture-specific
◆ #[cfg(feature = "name")]     // Feature-gated compilation
◆ #[cfg(debug_assertions)]     // Debug build only
◆ #[cfg(test)]                 // Test build only
Testing & Documentation
rust◆ #[test]                      // Mark function as test
◆ #[bench]                     // Mark function as benchmark
◆ #[should_panic]              // Test should panic
◆ #[ignore]                    // Ignore test
◆ #[doc(hidden)]               // Hide from documentation
◆ #[doc(inline)]               // Force inline documentation
◆ #[doc(no_inline)]            // Prevent inline documentation
◆ #[doc(alias = "name")]       // Add search alias
Derive & Traits
rust◆ #[derive(Debug, Clone, ...)] // Auto-derive traits
◆ #[automatically_derived]     // Compiler-generated code
◆ #[default]                   // Default variant/implementation
FFI (Foreign Function Interface)
rust◆ #[extern]                    // External linkage
◆ #[no_mangle]                 // Prevent symbol mangling
◆ #[link(name = "lib")]        // Link external library
◆ #[link(kind = "static")]     // Static linking
◆ #[link(kind = "dylib")]      // Dynamic linking
◆ #[link_section = "section"]  // Place in specific section
Stability & Features
rust◆ #[stable(feature = "name", since = "1.0.0")]     // Stable API
◆ #[unstable(feature = "name", issue = "12345")]   // Unstable API
◆ #[deprecated]                                     // Deprecated item
◆ #[deprecated(since = "1.0.0", note = "reason")] // Detailed deprecation

Compiler Lint Attributes
General Lints
rust✓ dead_code                    // Unused code detection
✓ unused_imports               // Unused import statements
✓ unused_variables             // Unused variable bindings
✓ unused_mut                   // Unnecessary mut keywords
✓ unreachable_code             // Code that cannot be reached
✓ unreachable_patterns         // Unreachable match patterns
✓ unused_assignments           // Assignments to unused variables
✓ unused_attributes            // Unused attributes
✓ unused_must_use              // Unused must-use values
✓ path_statements              // Path statements with no effect
✓ while_true                   // while true loops
Style & Conventions
rust✓ non_camel_case_types         // Type naming convention
✓ non_snake_case               // Function/variable naming
✓ non_upper_case_globals       // Global constant naming
✓ non_shorthand_field_patterns // Struct pattern shorthand
✓ unused_parens                // Unnecessary parentheses
✓ unused_braces                // Unnecessary braces
✓ redundant_semicolons         // Extra semicolons
Safety & Correctness
rust✓ unsafe_code                  // Usage of unsafe code
✓ missing_docs                 // Missing documentation
✓ missing_debug_implementations // Missing Debug trait
✓ missing_copy_implementations  // Missing Copy trait
✓ trivial_casts                // Unnecessary type casts
✓ trivial_numeric_casts        // Unnecessary numeric casts
✓ improper_ctypes              // Improper C types in FFI
✓ variant_size_differences     // Large enum variant differences

Clippy Lint Categories
Correctness (clippy::correctness)
rust✓🔧 clippy::absurd_extreme_comparisons    // x < 0 for unsigned types
✓   clippy::almost_swapped                // Likely swapped variables
✓🔧 clippy::approx_constant               // Approximate mathematical constants
✓   clippy::assertions_on_constants       // Assertions on constants
✓🔧 clippy::bad_bit_mask                  // Bad bit mask operations
✓   clippy::cast_lossless                 // Lossless casts
✓   clippy::clone_double_ref              // Cloning double references
✓🔧 clippy::cmp_nan                       // Comparisons with NaN
✓   clippy::deprecated_semver             // Deprecated semver requirements
✓🔧 clippy::derive_hash_xor_eq            // Hash without Eq implementation
✓   clippy::drop_copy                     // Dropping Copy types
✓   clippy::duplicate_mod                 // Duplicate module declarations
✓🔧 clippy::eq_op                         // Equal operands in comparisons
✓   clippy::erasing_op                    // Operations that erase operands
✓   clippy::eval_order_dependence         // Evaluation order dependence
✓🔧 clippy::float_cmp                     // Float equality comparisons
✓   clippy::for_loop_over_option          // for loops over Options
✓   clippy::for_loop_over_result          // for loops over Results
✓   clippy::impossible_cast               // Impossible casts
✓🔧 clippy::ineffective_bit_mask          // Ineffective bit masks
✓   clippy::infinite_iter                 // Infinite iterators
✓🔧 clippy::inline_fn_without_body        // Inline functions without body
✓   clippy::iterator_step_by_zero         // Iterator step_by(0)
✓🔧 clippy::logic_bug                     // Logic bugs in boolean expressions
✓   clippy::mem_discriminant_non_enum     // mem::discriminant on non-enums
✓   clippy::mem_replace_with_default      // mem::replace with default
✓🔧 clippy::min_max                       // min/max with same arguments
✓   clippy::mismatched_target_os          // Mismatched target OS
✓🔧 clippy::modulo_one                    // Modulo with 1
✓   clippy::never_loop                    // Loops that never loop
✓🔧 clippy::no_effect                     // Statements with no effect
✓   clippy::nonsensical_open_options      // Nonsensical OpenOptions
✓🔧 clippy::option_map_unwrap_or          // option.map().unwrap_or()
✓🔧 clippy::option_map_unwrap_or_else     // option.map().unwrap_or_else()
✓   clippy::out_of_bounds_indexing        // Out of bounds array indexing
✓🔧 clippy::panic_params                  // Wrong panic! parameters
✓   clippy::possible_missing_comma        // Missing comma in array
✓🔧 clippy::precedence                    // Precedence confusion
✓🔧 clippy::print_with_newline            // print! with newline
✓🔧 clippy::println_empty_string          // println!("")
✓   clippy::range_step_by_zero            // Range step_by(0)
✓🔧 clippy::redundant_closure_call        // Immediately called closures
✓🔧 clippy::result_map_unwrap_or_else     // result.map().unwrap_or_else()
✓🔧 clippy::reverse_range_loop            // Reversed range in for loop
✓🔧 clippy::search_is_some                // .find().is_some()
✓🔧 clippy::should_implement_trait        // Should implement standard traits
✓🔧 clippy::single_match                  // Single match expressions
✓🔧 clippy::str_to_string                 // &str.to_string()
✓🔧 clippy::string_to_string              // String.to_string()
✓   clippy::temporary_assignment          // Assignment to temporaries
✓🔧 clippy::toplevel_ref_arg              // Top-level ref arguments
✓🔧 clippy::transmute_ptr_to_ref          // Transmute pointer to reference
✓🔧 clippy::unit_cmp                      // Comparisons with unit values
✓🔧 clippy::unnecessary_operation         // Unnecessary operations
✓🔧 clippy::unreachable                   // Unreachable code
✓🔧 clippy::unused_collect                // Unused collect() calls
✓🔧 clippy::unused_io_amount              // Unused IO operation results
✓🔧 clippy::useless_format                // Useless format! calls
✓🔧 clippy::vec_resize_to_zero            // vec.resize(0, x)
✓🔧 clippy::while_let_on_iterator         // while let on iterators
✓🔧 clippy::wrong_self_convention         // Wrong self convention
✓🔧 clippy::zst_offset                    // Offset on zero-sized types
Style (clippy::style)
rust✓🔧 clippy::assign_op_pattern             // a = a + b patterns
✓🔧 clippy::blacklisted_name              // Blacklisted variable names
✓🔧 clippy::bool_comparison               // Comparisons with booleans
✓🔧 clippy::chars_next_cmp                // .chars().next() == Some(x)
✓🔧 clippy::clone_on_copy                 // Cloning Copy types
✓🔧 clippy::collapsible_if                // Collapsible if statements
✓🔧 clippy::comparison_chain              // Comparison chains
✓🔧 clippy::double_neg                    // Double negation
✓🔧 clippy::duplicate_underscore_argument // Duplicate _ arguments
✓🔧 clippy::excessive_precision           // Excessive float precision
✓🔧 clippy::explicit_counter_loop         // Manual counter in for loops
✓🔧 clippy::explicit_into_iter_loop       // Explicit into_iter() calls
✓🔧 clippy::explicit_iter_loop            // Explicit iter() calls
✓🔧 clippy::filter_next                   // .filter().next()
✓🔧 clippy::for_kv_map                    // for (k, v) in map
✓🔧 clippy::get_unwrap                    // .get().unwrap()
✓🔧 clippy::identity_conversion           // Identity conversions
✓🔧 clippy::implicit_hasher               // HashMap/HashSet with default hasher
✓🔧 clippy::implicit_return               // Implicit return statements
✓🔧 clippy::inconsistent_digit_grouping   // Inconsistent digit grouping
✓🔧 clippy::infallible_destructuring_match // Infallible destructuring
✓🔧 clippy::into_iter_on_array            // into_iter() on arrays
✓🔧 clippy::into_iter_on_ref              // into_iter() on references
✓🔧 clippy::items_after_statements        // Items after statements
✓🔧 clippy::iter_next_loop                // Manual iteration
✓🔧 clippy::iter_nth_zero                 // .iter().nth(0)
✓🔧 clippy::iter_skip_next                // .iter().skip().next()
✓🔧 clippy::just_underscores_and_digits   // Variable names with only _ and digits
✓🔧 clippy::len_without_is_empty          // len() without is_empty()
✓🔧 clippy::len_zero                      // len() == 0
✓🔧 clippy::let_and_return                // Let binding followed by return
✓🔧 clippy::let_unit_value                // Let binding unit values
✓🔧 clippy::main_recursion                // Recursive main function
✓🔧 clippy::manual_saturating_arithmetic  // Manual saturating arithmetic
✓🔧 clippy::many_single_char_names        // Many single character names
✓🔧 clippy::map_collect                   // .map().collect()
✓🔧 clippy::match_bool                    // Matching on booleans
✓🔧 clippy::match_overlapping_arm         // Overlapping match arms
✓🔧 clippy::match_ref_pats                // ref patterns in match
✓🔧 clippy::match_single_binding          // Single binding matches
✓🔧 clippy::mem_replace_option_with_none  // mem::replace with None
✓🔧 clippy::new_ret_no_self               // new() not returning Self
✓🔧 clippy::new_without_default           // new() without Default
✓🔧 clippy::nonminimal_bool               // Non-minimal boolean expressions
✓🔧 clippy::ok_expect                     // .ok().expect()
✓🔧 clippy::option_map_or_none            // .map_or(None, f)
✓🔧 clippy::or_fun_call                   // .or(function_call())
✓🔧 clippy::ptr_arg                       // Pointer arguments
✓🔧 clippy::question_mark                 // Manual error propagation
✓🔧 clippy::range_plus_one                // Range with + 1
✓🔧 clippy::range_minus_one               // Range with - 1
✓🔧 clippy::redundant_closure             // Redundant closures
✓🔧 clippy::redundant_field_names         // Redundant field names
✓🔧 clippy::redundant_pattern             // Redundant patterns
✓🔧 clippy::redundant_pattern_matching    // Redundant pattern matching
✓🔧 clippy::redundant_static_lifetimes    // Redundant 'static lifetimes
✓🔧 clippy::result_map_or_into_option     // .map_or(None, Some)
✓🔧 clippy::same_functions_in_if_condition // Same function in if condition
✓🔧 clippy::short_circuit_statement       // Short circuit in statements
✓🔧 clippy::single_char_pattern           // Single character patterns
✓🔧 clippy::string_extend_chars           // String extend with chars
✓🔧 clippy::suspicious_arithmetic_impl    // Suspicious arithmetic implementations
✓🔧 clippy::suspicious_assignment_formatting // Suspicious assignment formatting
✓🔧 clippy::suspicious_else_formatting    // Suspicious else formatting
✓🔧 clippy::suspicious_op_assign_impl     // Suspicious op assign implementations
✓🔧 clippy::suspicious_unary_op_formatting // Suspicious unary op formatting
✓🔧 clippy::tabs_in_doc_comments          // Tabs in doc comments
✓🔧 clippy::to_digit_is_some              // .to_digit().is_some()
✓🔧 clippy::to_string_in_display          // to_string() in Display
✓🔧 clippy::try_err                       // try! with Err
✓🔧 clippy::unit_arg                      // Unit arguments
✓🔧 clippy::unnecessary_fold              // Unnecessary fold operations
✓🔧 clippy::unnecessary_mut_passed        // Unnecessary mut in function calls
✓🔧 clippy::unneeded_field_pattern        // Unneeded field patterns
✓🔧 clippy::unused_unit                   // Unused unit expressions
✓🔧 clippy::used_underscore_binding       // Used underscore bindings
✓🔧 clippy::useless_let_if_seq            // Useless let if sequences
✓🔧 clippy::verbose_bit_mask              // Verbose bit mask operations
✓🔧 clippy::while_let_loop                // while let loops
✓🔧 clippy::write_with_newline            // write! with newline
✓🔧 clippy::writeln_empty_string          // writeln!("")
✓🔧 clippy::zero_width_space              // Zero width space characters
Complexity (clippy::complexity)
rust✓🔧 clippy::bind_instead_of_map           // bind instead of map
✓🔧 clippy::bool_comparison               // Boolean comparisons
✓🔧 clippy::borrowed_box                  // &Box<T> instead of &T
✓🔧 clippy::char_lit_as_u8                // Character literal as u8
✓🔧 clippy::chars_last_cmp                // .chars().last() comparisons
✓🔧 clippy::clone_on_copy                 // Clone on Copy types
✓🔧 clippy::crosspointer_transmute        // Cross-pointer transmute
✓🔧 clippy::double_comparisons            // Double comparisons
✓🔧 clippy::duration_subsec               // Duration subsecond methods
✓🔧 clippy::explicit_write                // Explicit write! calls
✓🔧 clippy::extra_unused_lifetimes        // Extra unused lifetimes
✓🔧 clippy::filter_map                    // .filter().map()
✓🔧 clippy::filter_map_next               // .filter_map().next()
✓🔧 clippy::find_map                      // .find().map()
✓🔧 clippy::flat_map_identity             // flat_map with identity
✓🔧 clippy::for_loop_over_option          // for loops over Option
✓🔧 clippy::for_loop_over_result          // for loops over Result
✓🔧 clippy::identity_op                   // Identity operations
✓🔧 clippy::if_same_then_else             // Same if/else branches
✓🔧 clippy::int_plus_one                  // x + 1 comparisons
✓🔧 clippy::iter_cloned_collect           // .iter().cloned().collect()
✓🔧 clippy::manual_memcpy                 // Manual memory copy
✓🔧 clippy::manual_swap                   // Manual variable swapping
✓🔧 clippy::map_entry                     // HashMap entry API usage
✓🔧 clippy::map_flatten                   // .map().flatten()
✓🔧 clippy::map_identity                  // map with identity function
✓🔧 clippy::naive_bytecount               // Naive byte counting
✓🔧 clippy::needless_bool                 // Needless boolean expressions
✓🔧 clippy::needless_borrowed_reference   // Needless borrowed references
✓🔧 clippy::needless_collect              // Needless collect() calls
✓🔧 clippy::needless_continue             // Needless continue statements
✓🔧 clippy::needless_lifetimes            // Needless lifetime parameters
✓🔧 clippy::needless_pass_by_value        // Needless pass by value
✓🔧 clippy::needless_range_loop           // Needless range loops
✓🔧 clippy::needless_return               // Needless return statements
✓🔧 clippy::needless_update               // Needless struct updates
✓🔧 clippy::neg_cmp_op_on_partial_ord     // Negated comparison operators
✓🔧 clippy::neg_multiply                  // Multiplication by -1
✓🔧 clippy::no_effect                     // Statements with no effect
✓🔧 clippy::option_as_ref_deref           // option.as_ref().map(Deref::deref)
✓🔧 clippy::option_filter_map             // .filter().map() on Options
✓🔧 clippy::option_map_unwrap_or          // option.map().unwrap_or()
✓🔧 clippy::option_map_unwrap_or_else     // option.map().unwrap_or_else()
✓🔧 clippy::partialeq_ne_impl             // PartialEq ne() implementation
✓🔧 clippy::range_zip_with_len            // Range zip with length
✓🔧 clippy::redundant_clone               // Redundant clone() calls
✓🔧 clippy::redundant_closure_call        // Redundant closure calls
✓🔧 clippy::search_is_some                // .find().is_some()
✓🔧 clippy::short_circuit_statement       // Short circuit statements
✓🔧 clippy::single_char_pattern           // Single character patterns
✓🔧 clippy::single_element_loop           // Single element loops
✓🔧 clippy::string_lit_as_bytes           // String literal as bytes
✓🔧 clippy::too_many_arguments            // Too many function arguments
✓🔧 clippy::transmute_bytes_to_str        // Transmute bytes to str
✓🔧 clippy::transmute_ptr_to_ptr          // Transmute pointer to pointer
✓🔧 clippy::type_complexity               // Complex type definitions
✓🔧 clippy::unicode_not_nfc               // Unicode not in NFC
✓🔧 clippy::unit_arg                      // Unit arguments
✓🔧 clippy::unnecessary_cast              // Unnecessary type casts
✓🔧 clippy::unnecessary_filter_map        // Unnecessary filter_map
✓🔧 clippy::unnecessary_fold              // Unnecessary fold operations
✓🔧 clippy::unnecessary_unwrap            // Unnecessary unwrap() calls
✓🔧 clippy::useless_conversion            // Useless type conversions
✓🔧 clippy::useless_vec                   // Useless vec! calls
✓🔧 clippy::vec_box                       // Vec<Box<T>> instead of Vec<T>
✓🔧 clippy::while_let_on_iterator         // while let on iterators
✓🔧 clippy::zero_divided_by_zero          // 0.0 / 0.0 operations
Perf (clippy::perf)
rust✓🔧 clippy::box_vec                       // Box<Vec<T>> instead of Vec<T>
✓🔧 clippy::expect_fun_call               // expect() with function calls
✓🔧 clippy::extend_from_slice             // push_str() with &str
✓🔧 clippy::implicit_clone                // Implicit clone operations
✓🔧 clippy::inefficient_to_string         // Inefficient to_string() calls
✓🔧 clippy::large_enum_variant            // Large enum variants
✓🔧 clippy::manual_memcpy                 // Manual memory copy loops
✓🔧 clippy::map_clone                     // .map(|x| x.clone())
✓🔧 clippy::naive_bytecount               // Naive byte counting
✓🔧 clippy::or_fun_call                   // .or(function_call())
✓🔧 clippy::redundant_allocation          // Redundant allocations
✓🔧 clippy::redundant_clone               // Redundant clone() calls
✓🔧 clippy::single_char_pattern           // Single character string patterns
✓🔧 clippy::slow_vector_initialization    // Slow vector initialization
✓🔧 clippy::stable_sort_primitive         // stable_sort on primitives
✓🔧 clippy::too_many_arguments            // Functions with many arguments
✓🔧 clippy::trivial_regex                 // Trivial regex patterns
✓🔧 clippy::unnecessary_clone             // Unnecessary clone operations
✓🔧 clippy::useless_vec                   // Useless vec! macro calls
✓🔧 clippy::vec_box                       // Vec<Box<T>> allocations
Pedantic (clippy::pedantic) - Opt-in
rust✓🔧 clippy::cast_lossless                 // Lossless numeric casts
✓🔧 clippy::cast_possible_truncation      // Potentially truncating casts
✓🔧 clippy::cast_possible_wrap            // Potentially wrapping casts
✓🔧 clippy::cast_precision_loss           // Precision-losing casts
✓🔧 clippy::cast_sign_loss                // Sign-losing casts
✓🔧 clippy::checked_conversions           // Checked numeric conversions
✓🔧 clippy::copy_iterator                 // Copy iterators
✓🔧 clippy::default_trait_access          // Default trait access
✓🔧 clippy::doc_markdown                  // Markdown in doc comments
✓🔧 clippy::empty_enum                    // Empty enums
✓🔧 clippy::enum_glob_use                 // Enum glob imports
✓🔧 clippy::expl_impl_clone_on_copy       // Explicit Clone on Copy
✓🔧 clippy::explicit_deref_methods        // Explicit deref method calls
✓🔧 clippy::explicit_into_iter_loop       // Explicit into_iter() in loops
✓🔧 clippy::explicit_iter_loop            // Explicit iter() in loops
✓🔧 clippy::filter_map_next               // .filter_map().next()
✓🔧 clippy::find_map                      // .find().map() chains
✓🔧 clippy::float_cmp_const               // Float comparison with constants
✓🔧 clippy::fn_params_excessive_bools     // Functions with many bool params
✓🔧 clippy::if_not_else                   // if !condition patterns
✓🔧 clippy::implicit_clone                // Implicit clone operations
✓🔧 clippy::implicit_hasher               // Implicit default hashers
✓🔧 clippy::inconsistent_digit_grouping   // Inconsistent number formatting
✓🔧 clippy::inefficient_to_string         // Inefficient string conversion
✓🔧 clippy::inline_always                 // #[inline(always)] usage
✓🔧 clippy::invalid_upcast_comparisons    // Invalid upcast comparisons
✓🔧 clippy::items_after_statements        // Items after statements
✓🔧 clippy::large_digit_groups            // Large digit groups
✓🔧 clippy::large_stack_arrays            // Large stack-allocated arrays
✓🔧 clippy::large_types_passed_by_value   // Large types passed by value
✓🔧 clippy::linkedlist                    // LinkedList usage
✓🔧 clippy::macro_use_imports             // #[macro_use] imports
✓🔧 clippy::manual_ok_or                  // Manual ok_or implementations
✓🔧 clippy::map_flatten                   // .map().flatten() chains
✓🔧 clippy::map_unwrap_or                 // .map().unwrap_or() chains
✓🔧 clippy::match_on_vec_items            // Matching on Vec items
✓🔧 clippy::match_same_arms               // Match arms with same body
✓🔧 clippy::match_wild_err_arm            // Wildcard in error match
✓🔧 clippy::match_wildcard_for_single_variants // Wildcard for single variants
✓🔧 clippy::maybe_infinite_iter           // Potentially infinite iterators
✓🔧 clippy::mem_forget                    // mem::forget usage
✓🔧 clippy::missing_errors_doc            // Missing error documentation
✓🔧 clippy::missing_panics_doc            // Missing panic documentation
✓🔧 clippy::module_name_repetitions       // Module name repetitions
✓🔧 clippy::must_use_candidate            // Functions that should be must_use
✓🔧 clippy::must_use_unit                 // must_use on unit-returning functions
✓🔧 clippy::naive_bytecount               // Naive byte counting
✓🔧 clippy::needless_continue             // Needless continue statements
✓🔧 clippy::needless_pass_by_value        // Needless pass by value
✓🔧 clippy::non_ascii_literal             // Non-ASCII string literals
✓🔧 clippy::option_option                 // Option<Option<T>>
✓🔧 clippy::path_buf_push_overwrite       // PathBuf::push overwrites
✓🔧 clippy::ptr_as_ptr                    // Pointer casting patterns
✓🔧 clippy::pub_enum_variant_names        // Public enum variant naming
✓🔧 clippy::range_minus_one               // x..y-1 range patterns
✓🔧 clippy::range_plus_one                // x..y+1 range patterns
✓🔧 clippy::redundant_closure_for_method_calls // Redundant closures for methods
✓🔧 clippy::redundant_else                // Redundant else branches
✓🔧 clippy::ref_option_ref                // &Option<&T> patterns
✓🔧 clippy::same_functions_in_if_condition // Same function calls in conditions
✓🔧 clippy::semicolon_if_nothing_returned // Missing semicolons
✓🔧 clippy::similar_names                 // Similar variable names
✓🔧 clippy::single_match_else             // Single match with else
✓🔧 clippy::string_add                    // String concatenation with +
✓🔧 clippy::string_add_assign             // String concatenation with +=
✓🔧 clippy::struct_excessive_bools        // Structs with many bool fields
✓🔧 clippy::too_many_lines                // Functions with many lines
✓🔧 clippy::transmute_ptr_to_ptr          // Pointer transmutation
✓🔧 clippy::trivially_copy_pass_by_ref    // Trivially copyable by reference
✓🔧 clippy::unicode_not_nfc               // Non-NFC Unicode
✓🔧 clippy::unimplemented                 // unimplemented!() usage
✓🔧 clippy::uninlined_format_args         // Non-inlined format arguments
✓🔧 clippy::unnecessary_box               // Unnecessary Box allocations
✓🔧 clippy::unnecessary_wraps             // Unnecessary Result/Option wraps
✓🔧 clippy::unnested_or_patterns          // Unnested OR patterns
✓🔧 clippy::unused_self                   // Unused self parameters
✓🔧 clippy::used_underscore_binding       // Used underscore bindings
✓🔧 clippy::verbose_file_reads            // Verbose file reading
✓🔧 clippy::wildcard_imports              // Wildcard imports
Restriction (clippy::restriction) - Opt-in Only
rust✓   clippy::allow_attributes             // allow attribute usage
✓   clippy::arithmetic_side_effects      // Arithmetic operations
✓   clippy::as_conversions               // as conversions
✓   clippy::assertions_on_result_states  // Assertions on Result states
✓   clippy::clone_on_ref_ptr             // Clone on reference pointers
✓   clippy::create_dir                   // Directory creation
✓   clippy::dbg_macro                    // dbg! macro usage
✓   clippy::decimal_literal_representation // Decimal literal representation
✓   clippy::default_numeric_fallback     // Default numeric type fallback
✓   clippy::deref_by_slicing             // Deref by slicing
✓   clippy::disallowed_method            // Disallowed method calls
✓   clippy::disallowed_script_idents     // Disallowed script identifiers
✓   clippy::disallowed_type              // Disallowed types
✓   clippy::else_if_without_else         // else if without else
✓   clippy::empty_structs_with_brackets  // Empty structs with brackets
✓   clippy::exit                         // Process exit calls
✓   clippy::expect_used                  // expect() method usage
✓   clippy::filetype_is_file             // FileType::is_file() usage
✓   clippy::float_arithmetic             // Floating point arithmetic
✓   clippy::float_cmp_const              // Float comparison with constants
✓   clippy::fn_to_numeric_cast           // Function to numeric casts
✓   clippy::fn_to_numeric_cast_with_truncation // Truncating fn casts
✓   clippy::get_unwrap                   // get().unwrap() patterns
✓   clippy::if_then_some_else_none       // if then Some else None
✓   clippy::implicit_return              // Implicit return statements
✓   clippy::indexing_slicing             // Array/slice indexing
✓   clippy::inline_asm_x86_att_syntax    // Inline assembly AT&T syntax
✓   clippy::inline_asm_x86_intel_syntax  // Inline assembly Intel syntax
✓   clippy::integer_arithmetic           // Integer arithmetic
✓   clippy::integer_division             // Integer division
✓   clippy::let_underscore_must_use      // let _ = must_use_value
✓   clippy::lossy_float_literal          // Lossy float literals
✓   clippy::map_err_ignore               // map_err with ignored errors
✓   clippy::mem_forget                   // mem::forget usage
✓   clippy::missing_docs_in_private_items // Missing private docs
✓   clippy::missing_inline_in_public_items // Missing inline in public
✓   clippy::mixed_read_write_in_expression // Mixed read/write in expression
✓   clippy::mod_module_files             // mod.rs module files
✓   clippy::modulo_arithmetic            // Modulo arithmetic
✓   clippy::multiple_inherent_impl       // Multiple inherent impl blocks
✓   clippy::panic                        // panic! macro usage
✓   clippy::panic_in_result_fn           // panic in Result-returning functions
✓   clippy::partial_pub_fields           // Partially public struct fields
✓   clippy::pattern_type_mismatch        // Pattern type mismatches
✓   clippy::print_stderr                 // Print to stderr
✓   clippy::print_stdout                 // Print to stdout
✓   clippy::pub_use                      // pub use statements
✓   clippy::rc_buffer                    // Rc<Vec<T>> or similar
✓   clippy::rc_mutex                     // Rc<Mutex<T>>
✓   clippy::rest_pat_in_fully_bound_structs // Rest patterns in bound structs
✓   clippy::same_name_method             // Methods with same name
✓   clippy::self_named_module_files      // Self-named module files
✓   clippy::separated_literal_suffix     // Separated literal suffixes
✓   clippy::shadow_reuse                 // Variable shadowing with reuse
✓   clippy::shadow_same                  // Variable shadowing same name
✓   clippy::shadow_unrelated             // Variable shadowing unrelated
✓   clippy::single_char_lifetime         // Single character lifetimes
✓   clippy::str_to_string                // &str to String conversion
✓   clippy::string_add                   // String addition
✓   clippy::string_slice                 // String slicing
✓   clippy::string_to_string             // String to String conversion
✓   clippy::todo                         // todo! macro usage
✓   clippy::try_err                      // try! with Err
✓   clippy::undocumented_unsafe_blocks   // Undocumented unsafe blocks
✓   clippy::unimplemented                // unimplemented! macro usage
✓   clippy::unnecessary_self_imports     // Unnecessary self imports
✓   clippy::unneeded_field_pattern       // Unneeded field patterns
✓   clippy::unreachable                  // unreachable! macro usage
✓   clippy::unseparated_literal_suffix   // Unseparated literal suffixes
✓   clippy::unwrap_in_result             // unwrap in Result functions
✓   clippy::unwrap_used                  // unwrap() method usage
✓   clippy::use_debug                    // Debug trait usage in format
✓   clippy::verbose_file_reads           // Verbose file reading
✓   clippy::wildcard_enum_match_arm      // Wildcard enum match arms
Cargo (clippy::cargo) - Opt-in
rust✓   clippy::cargo_common_metadata        // Missing Cargo.toml metadata
✓   clippy::multiple_crate_versions      // Multiple versions of same crate
✓   clippy::negative_feature_names       // Negative feature names
✓   clippy::redundant_feature_names      // Redundant feature names
✓   clippy::wildcard_dependencies        // Wildcard version dependencies
Nursery (clippy::nursery) - Experimental
rust✓⚠️ clippy::cognitive_complexity         // Cognitive complexity measurement
✓⚠️ clippy::disallowed_method            // Disallowed method usage
✓⚠️ clippy::disallowed_type              // Disallowed type usage
✓⚠️ clippy::fallible_impl_from           // Fallible From implementations
✓⚠️ clippy::future_not_send              // Future not Send
✓⚠️ clippy::imprecise_flops              // Imprecise floating point ops
✓⚠️ clippy::mutex_integer                // Mutex around integer
✓⚠️ clippy::option_if_let_else           // Option if let else patterns
✓⚠️ clippy::path_buf_push_overwrite      // PathBuf push overwrites
✓⚠️ clippy::redundant_pub_crate          // Redundant pub(crate)
✓⚠️ clippy::string_lit_as_bytes          // String literals as bytes
✓⚠️ clippy::suboptimal_flops             // Suboptimal floating point ops
✓⚠️ clippy::suspicious_operation_groupings // Suspicious operation groupings
✓⚠️ clippy::use_self                     // Use Self in implementations

Usage Examples
Basic Lint Configuration
rust// Enable specific lints
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(missing_docs)]

// Disable problematic lints
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]

// Function-level lint control
#[allow(clippy::too_many_arguments)]
fn complex_function(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32) {
    // Implementation
}
Conditional Attribute Usage
rust// Platform-specific attributes
#[cfg(target_os = "linux")]
#[link(name = "ssl")]
extern "C" {
    fn ssl_connect() -> i32;
}

// Feature-gated code
#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    value: i32,
}
Performance-Critical Attributes
rust// Hot path optimization
#[inline(always)]
#[cold] // For error paths
fn critical_calculation(x: f64) -> f64 {
    x * x + 2.0 * x + 1.0
}

// Memory layout control
#[repr(C, packed)]
struct NetworkPacket {
    header: u32,
    payload: [u8; 1024],
}
Documentation Attributes
rust#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

/// Public API function with comprehensive documentation
///
/// # Examples
///
/// \```
/// let result = calculate(5, 10);
/// assert_eq!(result, 15);
/// \```
///
/// # Errors
///
/// Returns an error if the calculation overflows.
#[doc(alias = "add")]
pub fn calculate(a: i32, b: i32) -> Result<i32, ArithmeticError> {
    a.checked_add(b).ok_or(ArithmeticError::Overflow)
}
