use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, Packet, QoS};
use serde::Deserialize;
use std::env;
use std::sync::Arc;

use crate::presentation::routes::unit_routes::routes;

pub async fn run() {
    let client_id = env::var("MQTT_CLIENT_ID").expect("missing MQTT_CLIENT_ID env variable");
    let host = env::var("MQTT_BROKER_HOST").expect("missing MQTT_BROKER_HOST env variable");
    let port = env::var("MQTT_BROKER_PORT").expect("missing MQTT_BROKER_PORT env variable");

    let port: u16 = port
        .trim()
        .parse()
        .expect("failed to parse MQTT_BROKER_PORT env variable");

    let mut mqtt_options = MqttOptions::new(client_id, host, port);
    mqtt_options.set_keep_alive(std::time::Duration::from_secs(10));

    let (client, mut event_loop) = AsyncClient::new(mqtt_options, 10);

    let client = Arc::new(client);

    client
        .subscribe("+/skf/data", QoS::AtLeastOnce)
        .await
        .expect("failed to subscribe");

    #[derive(Deserialize)]
    struct UnitRequest {
        #[serde(rename = "RegAd")]
        regad: String,
        #[serde(rename = "D1")]
        d1: String,
    }

    loop {
        let client = client.clone();

        if let Ok(event) = event_loop.poll().await {
            match event {
                Event::Incoming(Incoming::ConnAck(_)) => {
                    println!("connected to broker");
                }
                Event::Incoming(Packet::Publish(publish)) => {
                    let topic: Vec<&str> = publish.topic.split("/").collect();

                    if let Ok(unit_payload) = String::from_utf8(publish.payload.to_vec()) {
                        let unit_subscribe_topic = topic[0].to_string();
                        let unit_request = serde_json::from_str::<UnitRequest>(&unit_payload)
                            .unwrap_or(UnitRequest {
                                regad: String::from("0"),
                                d1: String::from("0"),
                            });
                        let register_address = unit_request.regad;
                        let data = unit_request.d1;

                        // let unit_publish_topic = topic[1].to_string();

                        tokio::task::spawn(async move {
                            routes(client, unit_subscribe_topic, register_address, data).await
                        });
                    }
                }
                Event::Incoming(Incoming::Disconnect) => {
                    println!("disconnected from the broker");
                }
                _ => (),
            }
        }
    }
}
