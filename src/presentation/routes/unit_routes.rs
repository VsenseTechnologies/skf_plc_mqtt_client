use rumqttc::AsyncClient;
use std::sync::Arc;

use crate::domain::repositories::unit_repository::UnitRepository;
use crate::presentation::handlers::unit_handler::blower_run_update_handler;
use crate::presentation::handlers::unit_handler::blower_trip_update_handler;
use crate::presentation::handlers::unit_handler::elevator_run_update_handler;
use crate::presentation::handlers::unit_handler::elevator_trip_update_handler;
use crate::presentation::handlers::unit_handler::pid_valve_stream_handler;
use crate::presentation::handlers::unit_handler::pid_valve_update_handler;
use crate::presentation::handlers::unit_handler::rotor_run_update_handler;
use crate::presentation::handlers::unit_handler::rotor_trip_update_handler;
use crate::presentation::handlers::unit_handler::temperature_stream_handler;
use crate::presentation::handlers::unit_handler::temperature_update_handler;

pub async fn routes<T: UnitRepository>(
    client: Arc<AsyncClient>,
    unit_subscribe_topic: String,
    unit_publish_topic: String,
    unit_payload: String,
    unit_repo: T,
) {
    let unit_publish_topic = unit_publish_topic.as_str();
    match unit_publish_topic {
        "stream_temp" => {
            temperature_stream_handler(client, unit_subscribe_topic, unit_payload).await
        }
        "stream_pid" => pid_valve_stream_handler(client, unit_subscribe_topic, unit_payload).await,
        "update_temp" => {
            temperature_update_handler(client, unit_subscribe_topic, unit_payload, unit_repo).await
        }
        "update_pid" => {
            pid_valve_update_handler(client, unit_subscribe_topic, unit_payload, unit_repo).await
        }
        "update_publish_blower_trip_st" => {
            blower_trip_update_handler(client, unit_subscribe_topic, unit_payload, unit_repo).await
        }
        "update_publish_elevator_trip_st" => {
            elevator_trip_update_handler(client, unit_subscribe_topic, unit_payload, unit_repo)
                .await
        }
        "update_publish_rotor_trip_st" => {
            rotor_trip_update_handler(client, unit_subscribe_topic, unit_payload, unit_repo).await
        }
        "update_publish_blower_run_st" => {
            blower_run_update_handler(client, unit_subscribe_topic, unit_payload, unit_repo).await
        }
        "update_publish_elevator_run_st" => {
            elevator_run_update_handler(client, unit_subscribe_topic, unit_payload, unit_repo).await
        }
        "update_publish_rotor_run_st" => {
            rotor_run_update_handler(client, unit_subscribe_topic, unit_payload, unit_repo).await
        }
        _ => (),
    }
}
