use std::sync::RwLockReadGuard;

use crate::interactive::mqtt_history::MqttHistory;
use crate::mqtt::HistoryEntry;

/// Abstraction over a message source (MQTT broker, Zenoh session, …) feeding the interactive UI.
///
/// Implementors run their own background thread accumulating messages into a shared
/// [`MqttHistory`] and expose it read-only to the UI.
pub trait HistorySource {
    /// Read access to the accumulated topic history.
    fn get_history(&self) -> RwLockReadGuard<'_, MqttHistory>;

    /// Current connection error, if any.
    fn connection_err(&self) -> Option<String>;

    /// Remove a single entry from the local history cache.
    fn uncache_topic_entry(&self, topic: &str, index: usize) -> Option<HistoryEntry>;

    /// Remove all retained messages below the given topic.
    ///
    /// Only meaningful for MQTT; other sources implement this as a no-op.
    fn clean_below(&self, topic: &str) -> anyhow::Result<()>;
}

/// Protocol-specific presentation details for the otherwise shared UI.
pub struct Capabilities {
    /// Whether the "clean retained" action is offered.
    pub supports_clean: bool,
    /// Header label for the metadata column of the history table.
    pub meta_header: &'static str,
    /// Title for the connection error widget.
    pub error_title: &'static str,
    /// Connection target shown in the footer (broker URL, Zenoh endpoint, …).
    pub target: Box<str>,
}

impl Capabilities {
    pub fn mqtt(target: String) -> Self {
        Self {
            supports_clean: true,
            meta_header: "QoS",
            error_title: "MQTT Connection Error",
            target: target.into(),
        }
    }

    pub fn zenoh(target: String) -> Self {
        Self {
            supports_clean: false,
            meta_header: "Exp/Prio",
            error_title: "Zenoh Connection Error",
            target: target.into(),
        }
    }
}
