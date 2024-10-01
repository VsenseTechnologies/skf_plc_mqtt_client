use rumqttc::AsyncClient;
use std::sync::Arc;

use crate::domain::repositories::unit_repository::UnitRepository;

pub fn routes<T: UnitRepository>(client: Arc<AsyncClient>, topic: &str, payload: String, db: T) {
    match topic {
        "rt_temp" => temperature_update_handler(client, payload, db).await,
        "rt_pid" => pid_valve_opeing_update_handler(client, payload, db).await,
        "blower_trip_fb" => blower_trip_status_update_handler(client, payload, db).await,
        "elevator_trip_fb" => elevator_trip_status_update_handler(client, payload, db).await,
        "rotor_trip_fb" => rotor_trip_status_update_handler(client, payload, db).await,
        "blower_run_fb" => blower_run_status_update_handler(client, payload, db).await,
        "elevator_run_fb" => elevator_run_status_update_handler(client, payload, db).await,
        "rotor_run_fb" => rotor_run_status_update_handler(client, payload, db).await,
        "" => (),
    }
}
