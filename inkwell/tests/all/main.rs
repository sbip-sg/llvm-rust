//! Having a main.rs in a directory w/ mods will force tests to be built in a single binary

//--------------------------------------------------------------------
// ATTRIBUTES TO GUARANTEE CODE QUALITY. DO NOT MODIFY.
// Warning on future incompatible features
#![warn(future_incompatible)]
// Linting rules, enabled when running Cargo with `--features linting`
#![cfg_attr(feature = "linting", deny(missing_docs))]
#![cfg_attr(feature = "linting", deny(clippy::missing_docs_in_private_items))]
#![cfg_attr(feature = "linting", deny(unused))]
#![cfg_attr(feature = "linting", deny(nonstandard_style))]
#![cfg_attr(feature = "linting", deny(clippy::perf))]
#![cfg_attr(feature = "linting", deny(clippy::style))]
#![cfg_attr(feature = "linting", deny(clippy::complexity))]
#![cfg_attr(feature = "linting", deny(clippy::suspicious))]
#![cfg_attr(feature = "linting", deny(clippy::doc_markdown))]
#![cfg_attr(feature = "linting", deny(rustdoc::broken_intra_doc_links))]
#![cfg_attr(feature = "linting", deny(rustdoc::bare_urls))]
//---------------------------------------------------------------------

#[macro_use]
extern crate inkwell_internals;

mod test_alias_analysis;
#[cfg(not(any(
    feature = "llvm3-6",
    feature = "llvm3-7",
    feature = "llvm3-8"
)))]
mod test_attributes;
mod test_basic_block;
mod test_builder;
mod test_context;
#[cfg(not(any(
    feature = "llvm3-6",
    feature = "llvm3-7",
    feature = "llvm3-8",
    feature = "llvm3-9",
    feature = "llvm4-0",
    feature = "llvm5-0",
    feature = "llvm6-0"
)))]
mod test_debug_info;
mod test_execution_engine;
mod test_instruction_conversion;
mod test_instruction_values;
mod test_intrinsics;
mod test_module;
mod test_object_file;
mod test_passes;
mod test_targets;
mod test_tari_example;
mod test_types;
mod test_values;
