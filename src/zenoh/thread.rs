use std::sync::{Arc, RwLock, RwLockReadGuard};
use std::thread;

// The external crate shares its name with this module, so it is addressed as `::zenoh`.
use ::zenoh::qos::Priority;
use ::zenoh::sample::Sample;
use ::zenoh::{Config, Wait as _, open};

use crate::interactive::mqtt_history::MqttHistory;
use crate::mqtt::{HistoryEntry, Time};
use crate::payload::Payload;
use crate::source::HistorySource;

type ConnectionErrorArc = Arc<RwLock<Option<String>>>;
type HistoryArc = Arc<RwLock<MqttHistory>>;

pub struct ZenohThread {
    connection_err: ConnectionErrorArc,
    history: HistoryArc,
}

impl ZenohThread {
    pub fn new(
        config: Config,
        key_expr: String,
        payload_size_limit: usize,
    ) -> anyhow::Result<Self> {
        let session = open(config)
            .wait()
            .map_err(|err| anyhow::anyhow!("Failed to open the Zenoh session: {err}"))?;
        let subscriber = session
            .declare_subscriber(key_expr)
            .wait()
            .map_err(|err| anyhow::anyhow!("Failed to declare the Zenoh subscriber: {err}"))?;

        let connection_err = Arc::new(RwLock::new(None));
        let history = Arc::new(RwLock::new(MqttHistory::new()));

        {
            let connection_err = Arc::clone(&connection_err);
            let history = Arc::clone(&history);
            thread::Builder::new()
                .name("zenoh session".to_owned())
                .spawn(move || {
                    // The subscriber stops receiving once the session is dropped, so keep it alive.
                    let _session = session;
                    loop {
                        match subscriber.recv() {
                            Ok(sample) => add_sample(&history, &sample, payload_size_limit),
                            Err(err) => {
                                *connection_err.write().unwrap() =
                                    Some(format!("Zenoh subscriber stopped receiving: {err}"));
                                break;
                            }
                        }
                    }
                })
                .expect("should be able to spawn a thread");
        }

        Ok(Self {
            connection_err,
            history,
        })
    }
}

fn add_sample(history: &HistoryArc, sample: &Sample, payload_size_limit: usize) {
    let topic = sample.key_expr().as_str().to_owned();
    let payload = sample.payload().to_bytes().into_owned();
    history.write().unwrap().add(
        topic,
        HistoryEntry {
            time: Time::new_now(false),
            meta: format_meta(sample.express(), sample.priority()),
            payload_size: payload.len(),
            payload: Payload::truncated(payload, payload_size_limit),
        },
    );
}

/// Render the express flag and priority into the history table's metadata column.
fn format_meta(express: bool, priority: Priority) -> Box<str> {
    let priority = priority as u8;
    if express {
        format!("EXPRESS P{priority}").into()
    } else {
        format!("P{priority}").into()
    }
}

impl HistorySource for ZenohThread {
    fn get_history(&self) -> RwLockReadGuard<'_, MqttHistory> {
        self.history.read().expect("zenoh session thread panicked")
    }

    fn connection_err(&self) -> Option<String> {
        self.connection_err
            .read()
            .expect("zenoh session thread panicked")
            .clone()
    }

    fn uncache_topic_entry(&self, topic: &str, index: usize) -> Option<HistoryEntry> {
        self.history
            .write()
            .expect("zenoh session thread panicked")
            .uncache_topic_entry(topic, index)
    }

    /// Zenoh has no concept of retained messages to clean, so this is a no-op.
    fn clean_below(&self, _topic: &str) -> anyhow::Result<()> {
        Ok(())
    }
}
