//! .

#![deny(
    clippy::print_stdout,
    clippy::expect_used,
    clippy::unwrap_used,
    missing_docs,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc
)]

///
pub mod args;

#[allow(
    clippy::print_stdout,
    clippy::expect_used,
    clippy::unwrap_used,
    missing_docs,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc
)]
pub mod git;


#[allow(
    clippy::print_stdout,
    clippy::expect_used,
    clippy::unwrap_used,
    missing_docs,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc
)]
/// all methods regarding plugins path
pub mod path;

#[allow(
    clippy::print_stdout,
    clippy::expect_used,
    clippy::unwrap_used,
    missing_docs,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc
)]
/// all methods regarding plugins
pub mod plugins;
