mod mqtt_cli {
    include!("src/cli.rs");
}
mod zenoh_cli {
    include!("src/cli_zenoh.rs");
}

fn main() -> std::io::Result<()> {
    use clap::{CommandFactory as _, ValueEnum as _};

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/cli.rs");
    println!("cargo:rerun-if-changed=src/cli_zenoh.rs");

    let target_dir = std::path::Path::new("target");
    let compl_dir = &target_dir.join("completions");
    let man_dir = &target_dir.join("manpages");
    _ = std::fs::remove_dir_all(compl_dir);
    _ = std::fs::remove_dir_all(man_dir);
    std::fs::create_dir_all(compl_dir)?;
    std::fs::create_dir_all(man_dir)?;

    for &shell in clap_complete::Shell::value_variants() {
        clap_complete::generate_to(shell, &mut mqtt_cli::Cli::command(), "mqttui", compl_dir)?;
        clap_complete::generate_to(shell, &mut zenoh_cli::Cli::command(), "zenohui", compl_dir)?;
    }

    clap_mangen::generate_to(mqtt_cli::Cli::command(), man_dir)?;
    clap_mangen::generate_to(zenoh_cli::Cli::command(), man_dir)?;

    Ok(())
}
