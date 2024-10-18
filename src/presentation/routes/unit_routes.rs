use rumqttc::AsyncClient;
use std::sync::Arc;

use crate::domain::repositories::unit_repository::UnitRepository;
use crate::presentation::handlers::unit_handler::blower_run_stream_handler;
use crate::presentation::handlers::unit_handler::blower_trip_stream_handler;
use crate::presentation::handlers::unit_handler::elevator_run_stream_handler;
use crate::presentation::handlers::unit_handler::elevator_trip_stream_handler;
use crate::presentation::handlers::unit_handler::pid_valve_stream_handler;
use crate::presentation::handlers::unit_handler::rotor_run_stream_handler;
use crate::presentation::handlers::unit_handler::rotor_trip_stream_handler;
use crate::presentation::handlers::unit_handler::step1_temperature_stream_handler;
use crate::presentation::handlers::unit_handler::step1_time_stream_handler;
use crate::presentation::handlers::unit_handler::step2_temperature_stream_handler;
use crate::presentation::handlers::unit_handler::step2_time_stream_handler;
use crate::presentation::handlers::unit_handler::step3_temperature_stream_handler;
use crate::presentation::handlers::unit_handler::step3_time_stream_handler;
use crate::presentation::handlers::unit_handler::step4_temperature_stream_handler;
use crate::presentation::handlers::unit_handler::step4_time_stream_handler;
use crate::presentation::handlers::unit_handler::temperature_stream_handler;

pub async fn routes(
    client: Arc<AsyncClient>,
    unit_subscribe_topic: String,
    unit_register_address: String,
    data: String,
) {
    let register_address = unit_register_address.as_str();

    match register_address {
        "302" => temperature_stream_handler(client, unit_subscribe_topic, data).await,
        "322" => pid_valve_stream_handler(client, unit_subscribe_topic, data).await,
        "18" => blower_trip_stream_handler(client, unit_subscribe_topic, data).await,
        "20" => elevator_trip_stream_handler(client, unit_subscribe_topic, data).await,
        "22" => rotor_trip_stream_handler(client, unit_subscribe_topic, data).await,
        "24" => blower_run_stream_handler(client, unit_subscribe_topic, data).await,
        "26" => elevator_run_stream_handler(client, unit_subscribe_topic, data).await,
        "28" => rotor_run_stream_handler(client, unit_subscribe_topic, data).await,
        "304" => step1_temperature_stream_handler(client, unit_subscribe_topic, data).await,
        "306" => step2_temperature_stream_handler(client, unit_subscribe_topic, data).await,
        "308" => step3_temperature_stream_handler(client, unit_subscribe_topic, data).await,
        "310" => step4_temperature_stream_handler(client, unit_subscribe_topic, data).await,
        "124" => step1_time_stream_handler(client, unit_subscribe_topic, data).await,
        "126" => step2_time_stream_handler(client, unit_subscribe_topic, data).await,
        "128" => step3_time_stream_handler(client, unit_subscribe_topic, data).await,
        "130" => step4_time_stream_handler(client, unit_subscribe_topic, data).await,
        _ => (),
    }
}
