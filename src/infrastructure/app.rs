use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, Packet, QoS};
use std::env;

pub async fn run() {
    let client_id = env::var("MQTT_CLIENT_ID").expect("missing MQTT_CLIENT_ID env variable");
    let host = env::var("MQTT_HOST").expect("missing MQTT_HOST env variable");
    let port = env::var("MQTT_PORT").expect("missing MQTT_PORT env variable");

    let port: u16 = port
        .trim()
        .parse()
        .expect("failed to parse MQTT_PORT env variable");

    let mut mqtt_options = MqttOptions::new(client_id, host, port);
    mqtt_options.set_keep_alive(std::time::Duration::from_secs(10));

    let (client, mut event_loop) = AsyncClient::new(mqtt_options, 10);

    client
        .subscribe("+/rt_temp", QoS::AtLeastOnce)
        .await
        .expect("failed to subscribe");
    client
        .subscribe("+/rt_pid", QoS::AtLeastOnce)
        .await
        .expect("failed to subscribe");
    client
        .subscribe("+/blower_trip_fb", QoS::AtLeastOnce)
        .await
        .expect("failed to subscribe");
    client
        .subscribe("+/elevator_trip_fb", QoS::AtLeastOnce)
        .await
        .expect("failed to subscribe");
    client
        .subscribe("+/rotor_trip_fb", QoS::AtLeastOnce)
        .await
        .expect("failed to subscribe");
    client
        .subscribe("+/blower_run_fb", QoS::AtLeastOnce)
        .await
        .expect("failed to subscribe");
    client
        .subscribe("+/elevator_run_fb", QoS::AtLeastOnce)
        .await
        .expect("failed to subscribe");
    client
        .subscribe("+/rotor_run_fb", QoS::AtLeastOnce)
        .await
        .expect("failed to subscribe");

    loop {
        if let Ok(event) = event_loop.poll().await {
            match event {
                Event::Incoming(Incoming::ConnAck(_)) => {
                    println!("connected to broker");
                }
                Event::Incoming(Packet::Publish(publish)) => {
                    let topic = publish.topic;
                    if let Ok(payload) = String::from_utf8(publish.payload.to_vec()) {
                        //topic route
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
