//! Entry point of Verazt crate

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

// Import all macros before anything else.
#[macro_use]
pub mod macros;

// Export sub modules
// pub mod cli;
pub mod arg;
pub mod debug;
pub mod report;
pub mod string;
pub mod system;
