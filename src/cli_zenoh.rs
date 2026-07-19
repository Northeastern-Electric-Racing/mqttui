use clap::{Parser, ValueHint};

#[expect(clippy::doc_markdown)]
#[derive(Debug, Parser)]
#[command(
    name = "zenohui",
    version,
    about = "Subscribe to a Zenoh key expression and inspect it in the terminal"
)]
pub struct Cli {
    /// Key expression to subscribe to.
    ///
    /// Defaults to the global namespace `**` which matches every key.
    #[arg(
        env = "ZENOHUI_KEY_EXPR",
        value_hint = ValueHint::Other,
        value_name = "KEY_EXPR",
        default_value = "**",
    )]
    pub key_expr: String,

    /// Path to a Zenoh configuration file (JSON5).
    ///
    /// When omitted the default configuration is used.
    /// Connection details like endpoints and the mode (peer/client/router) are configured here.
    #[arg(
        short,
        long,
        env = "ZENOHUI_CONFIG",
        value_hint = ValueHint::FilePath,
        value_name = "FILE",
    )]
    pub config: Option<std::path::PathBuf>,

    /// Truncate the payloads stored to the given size.
    ///
    /// Payloads bigger than that are truncated and not inspected for formats like JSON or MessagePack.
    /// Only their beginning up to the specified amount of bytes can be viewed.
    /// Increasing this value might result in higher memory consumption especially over time.
    #[arg(
        long,
        env = "ZENOHUI_PAYLOAD_SIZE_LIMIT",
        value_hint = ValueHint::Other,
        default_value_t = 8_000,
    )]
    pub payload_size_limit: usize,
}

#[test]
fn verify() {
    use clap::CommandFactory as _;
    Cli::command().debug_assert();
}
