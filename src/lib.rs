//! .

#![deny(
    clippy::print_stdout,
    clippy::expect_used,
    clippy::unwrap_used,
)]

pub mod args;
pub mod git;
/// all methods regarding plugins path
pub mod path;
/// all methods regarding plugins
pub mod plugins;

pub mod error;

