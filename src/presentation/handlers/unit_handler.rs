use log::error;
use rumqttc::AsyncClient;
use rumqttc::QoS;
use serde::Deserialize;
use serde_json::json;
use sqlx::Error;
use std::sync::Arc;

use crate::application::use_cases::update_blower_run::UpdateBlowerRunUseCase;
use crate::application::use_cases::update_blower_trip::UpdateBlowerTripUseCase;
use crate::application::use_cases::update_elevator_run::UpdateElevatorRunUseCase;
use crate::application::use_cases::update_elevator_trip::UpdateElevatorTripUseCase;
use crate::application::use_cases::update_pid_valve::UpdatePIDValveUseCase;
use crate::application::use_cases::update_rotor_run::UpdateRotorRunUseCase;
use crate::application::use_cases::update_rotor_trip::UpdateRotorTripUseCase;
use crate::application::use_cases::update_temperature::UpdateTemperatureUseCase;
use crate::domain::repositories::unit_repository::UnitRepository;

#[derive(Debug, Deserialize)]
pub struct Temperature {
    pub temp: String,
}

#[derive(Debug, Deserialize)]
pub struct PIDValve {
    pub opening: String,
}

#[derive(Debug, Deserialize)]
pub struct BlowerTrip {
    pub status: bool,
}

#[derive(Debug, Deserialize)]
pub struct ElevatorTrip {
    pub status: bool,
}

#[derive(Debug, Deserialize)]
pub struct RotorTrip {
    pub status: bool,
}

#[derive(Debug, Deserialize)]
pub struct BlowerRun {
    pub status: bool,
}

#[derive(Debug, Deserialize)]
pub struct ElevatorRun {
    pub status: bool,
}

#[derive(Debug, Deserialize)]
pub struct RotorRun {
    pub status: bool,
}

pub async fn temperature_update_handler<T: UnitRepository>(
    client: Arc<AsyncClient>,
    unit_subscribe_topic: String,
    temperature: String,
    unit_repo: T,
) {
    let update_temperature_use_case = UpdateTemperatureUseCase::new(unit_repo);
    let update_result = update_temperature_use_case
        .update_temperature(unit_subscribe_topic.clone(), temperature.clone())
        .await;

    if let Err(error) = update_result {
        error!(
            "error occured while updating the real time temperature error -> {:?} unit_subscribe_topic -> {}",
                error, unit_subscribe_topic
            );
        return;
    }

    let app_publish_topic = format!("app/{}", unit_subscribe_topic.clone());
    let app_publish_json_message = json!({"temperature":temperature});

    let encode_result = serde_json::to_vec(&app_publish_json_message);

    match encode_result {
        Ok(payload) => {
            let publish_result = client
                .publish(
                    app_publish_topic,
                    QoS::AtLeastOnce,
                    false,
                    payload,
                )
                .await;

            if let Err(error) = publish_result {
                error!("error occurred while publishing the real time temperature message to app error -> {:?} unit_subscribe_topic -> {}",error,unit_subscribe_topic);
                return;
            }
        }
        Err(error) => error!("error occurred while encoding the real time temperature message to app response message error -> {:?} unit_subscribe_id -> {}",error,unit_subscribe_topic)
    }
}

pub async fn pid_valve_update_handler<T: UnitRepository>(
    client: Arc<AsyncClient>,
    unit_subscribe_topic: String,
    opening: String,
    unit_repo: T,
) {
    let update_pid_valve_use_case = UpdatePIDValveUseCase::new(unit_repo);

    let update_result = update_pid_valve_use_case
        .update_pid_valve(unit_subscribe_topic.clone(), opening.clone())
        .await;

    if let Err(error) = update_result {
        error!(
            "error occured while updating the real time pid valve error -> {:?} unit_subscribe_topic -> {}",
            error, unit_subscribe_topic.clone()
        );
        return;
    }

    let app_publish_topic = format!("app/{}", unit_subscribe_topic.clone());
    let app_publish_json_message = json!({"pid_valve_opening":opening});

    let encode_result = serde_json::to_vec(&app_publish_json_message);

    match encode_result {
        Ok(payload) => {
            let publish_result = client.publish(app_publish_topic, QoS::AtLeastOnce, false, payload).await;

            if let Err(error) = publish_result {
                error!("error occured while publishing the real time pid valve message to app error -> {:?} unit_subscribe_topic -> {}",error,unit_subscribe_topic);
                return;
            }
        },
        Err(error) => error!("error occurred while encoding the real time pid valve message to app response message error -> {:?} unit_subscribe_topic -> {}",error,unit_subscribe_topic),
    }
}
