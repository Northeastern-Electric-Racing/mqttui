use std::time::Duration;

use clap::Parser as _;
use mqttui::cli::{self, Subcommands};
use mqttui::interactive::MqttThread;
use mqttui::source::Capabilities;
use mqttui::{clean_retained, interactive, log, mqtt, publish, read_one};

fn main() -> anyhow::Result<()> {
    let matches = cli::Cli::parse();

    let keep_alive = if let Some(Subcommands::CleanRetained { timeout, .. }) = matches.subcommands {
        Some(Duration::from_secs_f32(timeout))
    } else {
        None
    };
    let qos = rumqttc::qos(matches.qos).unwrap();
    let (broker, client, connection) = mqtt::connect(matches.mqtt_connection, keep_alive)?;

    match matches.subcommands {
        Some(Subcommands::CleanRetained { topic, dry_run, .. }) => {
            client.subscribe(topic, qos)?;
            clean_retained::clean_retained(&client, connection, qos, dry_run);
        }
        Some(Subcommands::Log {
            topic,
            json,
            verbose,
        }) => {
            for topic in topic {
                client.subscribe(topic, qos)?;
            }
            log::show(connection, json, verbose);
        }
        Some(Subcommands::ReadOne {
            topic,
            ignore_retained,
            pretty,
        }) => {
            for topic in topic {
                client.subscribe(topic, qos)?;
            }
            read_one::show(&client, connection, ignore_retained, pretty);
        }
        Some(Subcommands::Publish {
            topic,
            payload,
            retain,
            verbose,
        }) => {
            let payload = payload.map_or_else(
                || {
                    use std::io::Read as _;
                    let mut buffer = Vec::new();
                    std::io::stdin()
                        .read_to_end(&mut buffer)
                        .expect("Should be able to read the payload from stdin");
                    buffer
                },
                String::into_bytes,
            );
            if matches!(qos, rumqttc::QoS::AtMostOnce) {
                eprintln!(
                    "With QoS 0 at most once there wont be an acknowledgement from the broker. Waiting for a ping..."
                );
            }
            client.publish(topic, qos, retain, payload)?;
            publish::eventloop(&client, connection, verbose);
        }
        None => {
            let mqtt_thread = MqttThread::new(
                client.clone(),
                connection,
                matches.topic,
                qos,
                matches.payload_size_limit,
            )?;
            let capabilities = Capabilities::mqtt(broker.to_string());
            interactive::show(Box::new(mqtt_thread), capabilities)?;
            client.disconnect()?;
        }
    }

    Ok(())
}
