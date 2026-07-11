use clap::Parser as _;
use mqttui::cli_zenoh::Cli;
use mqttui::source::Capabilities;
use mqttui::{interactive, zenoh};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let config = zenoh::config(cli.config.as_deref())?;
    let target = format!("zenoh · {}", cli.key_expr);
    let zenoh_thread = zenoh::ZenohThread::new(config, cli.key_expr, cli.payload_size_limit)?;

    let capabilities = Capabilities::zenoh(target);
    interactive::show(Box::new(zenoh_thread), capabilities)?;

    Ok(())
}
