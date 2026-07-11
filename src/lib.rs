// This crate is an internal implementation detail shared by the `mqttui` and
// `zenohui` binaries rather than a published library. The public-API documentation
// lints therefore only add noise, so they are allowed crate-wide.
#![expect(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::must_use_candidate,
    reason = "internal shared crate, not a published API"
)]

pub mod clean_retained;
pub mod cli;
pub mod cli_zenoh;
pub mod format;
pub mod interactive;
pub mod log;
pub mod mqtt;
pub mod payload;
pub mod publish;
pub mod read_one;
pub mod source;
pub mod zenoh;
