use std::path::Path;

// The external crate shares its name with this module, so it is addressed as `::zenoh`.
use ::zenoh::Config;

/// Build a Zenoh [`Config`] from an optional JSON5 config file.
///
/// Falls back to the default configuration when no path is given.
pub fn config(path: Option<&Path>) -> anyhow::Result<Config> {
    path.map_or_else(
        || Ok(Config::default()),
        |path| {
            Config::from_file(path).map_err(|err| {
                anyhow::anyhow!("Failed to load Zenoh config from {}: {err}", path.display())
            })
        },
    )
}
