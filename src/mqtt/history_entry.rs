pub struct HistoryEntry {
    pub time: crate::mqtt::Time,
    /// Protocol-specific metadata rendered in the second column of the history table.
    ///
    /// MQTT stores the `QoS`; Zenoh stores the express flag and priority.
    pub meta: Box<str>,
    pub payload_size: usize,
    pub payload: crate::payload::Payload,
}
