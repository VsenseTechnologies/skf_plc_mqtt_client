use log::error;
use rumqttc::AsyncClient;
use rumqttc::QoS;
use serde_json::json;
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

pub async fn temperature_stream_handler(
    client: Arc<AsyncClient>,
    unit_subscribe_topic: String,
    temperature: String,
) {
    let app_publish_topic = format!("app/{}", &unit_subscribe_topic);
    let app_publish_json_message = json!({"message_type":0,"temp":temperature});

    let bytes_encode_result = serde_json::to_vec(&app_publish_json_message);

    match bytes_encode_result {
        Ok(payload) => {
            let publish_result = client.publish(app_publish_topic, QoS::AtLeastOnce, false, payload).await;
            if let Err(error) = publish_result {
                error!("error occurred while publishing the temperature stream message to app error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
                return;
            }
        },
        Err(error) => error!("error occurred while encoding temperature stream message to bytes error -> {:?} unit_id ->{}",error,unit_subscribe_topic)
    }
}

pub async fn pid_valve_stream_handler(
    client: Arc<AsyncClient>,
    unit_subscribe_topic: String,
    opening: String,
) {
    let app_publish_topic = format!("app/{}", &unit_subscribe_topic);
    let app_publish_json_message = json!({"message_type":1,"pid":opening});

    let bytes_encode_result = serde_json::to_vec(&app_publish_json_message);

    match bytes_encode_result {
        Ok(payload) => {
            let publish_result = client.publish(app_publish_topic, QoS::AtLeastOnce, false, payload).await;
            if let Err(error) = publish_result {
                error!("error occurred while publishing the pid valve opening stream message to app error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
                return;
            }
        },
        Err(error) => error!("error occurred while encoding pid valve opening stream message to bytes error -> {:?} unit_id -> {}",error,unit_subscribe_topic)
    }
}

pub async fn temperature_update_handler<T: UnitRepository>(
    _client: Arc<AsyncClient>,
    unit_subscribe_topic: String,
    temperature: String,
    unit_repo: T,
) {
    let update_temperature_use_case = UpdateTemperatureUseCase::new(unit_repo);
    let update_result = update_temperature_use_case
        .update_temperature(&unit_subscribe_topic, &temperature)
        .await;

    if let Err(error) = update_result {
        error!(
            "error occurred while updating the temperature to database error -> {:?} unit_id -> {}",
            error, unit_subscribe_topic
        );
        return;
    }
}

pub async fn pid_valve_update_handler<T: UnitRepository>(
    _client: Arc<AsyncClient>,
    unit_subscribe_topic: String,
    opening: String,
    unit_repo: T,
) {
    let update_pid_valve_use_case = UpdatePIDValveUseCase::new(unit_repo);

    let update_result = update_pid_valve_use_case
        .update_pid_valve(&unit_subscribe_topic, &opening)
        .await;

    if let Err(error) = update_result {
        error!(
            "error occured while updating the pid valve opening to databse error -> {:?} unit_id -> {}",
            error, &unit_subscribe_topic
        );
        return;
    }
}

pub async fn blower_trip_update_handler<T: UnitRepository>(
    client: Arc<AsyncClient>,
    unit_subscribe_topic: String,
    status: String,
    unit_repo: T,
) {
    let status: u8 = status.parse().unwrap_or(0 as u8);
    let status = if status == 0 { false } else { true };
    let update_blower_trip_usecase = UpdateBlowerTripUseCase::new(unit_repo);

    let update_result = update_blower_trip_usecase
        .update_blower_trip(&unit_subscribe_topic, status)
        .await;

    if let Err(error) = update_result {
        error!(
            "error occured while updating the blower trip status to databse error -> {:?} unit_id -> {}",
            error, &unit_subscribe_topic
        );
        return;
    }

    let app_publish_topic = format!("app/{}", &unit_subscribe_topic);
    let app_publish_json_message = json!({"message_type":2,"status":status});

    let bytes_encode_result = serde_json::to_vec(&app_publish_json_message);

    match bytes_encode_result {
        Ok(payload) => {
            let publish_result=client.publish(app_publish_topic,QoS::AtLeastOnce,false, payload).await;

            if let Err(error) = publish_result {
                error!("error occured while publishing the blower trip status to app error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
                return;
            }
        },
        Err(error) => error!("error occured while encoding the blower trip status message to bytes error -> {:?} unit_id -> {}",error,unit_subscribe_topic)
    }
}

pub async fn elevator_trip_update_handler<T: UnitRepository>(
    client: Arc<AsyncClient>,
    unit_subscribe_topic: String,
    status: String,
    unit_repo: T,
) {
    let status: u8 = status.parse().unwrap_or(0 as u8);
    let status = if status == 0 { false } else { true };
    let update_elevator_trip_usecase = UpdateElevatorTripUseCase::new(unit_repo);
    let update_result = update_elevator_trip_usecase
        .update_elevator_trip(&unit_subscribe_topic, status)
        .await;

    if let Err(error) = update_result {
        error!(
            "error occurred while updating the elevator trip status to database error -> {:?} unit_id -> {}",
            error, &unit_subscribe_topic,
        );
        return;
    }

    let app_publish_topic = format!("app/{}", &unit_subscribe_topic);
    let app_publish_json_message = json!({"message_type":3,"status":status});

    let bytes_encode_result = serde_json::to_vec(&app_publish_json_message);

    match bytes_encode_result {
        Ok(payload) => {
            let publish_result = client
                .publish(app_publish_topic, QoS::AtLeastOnce, false, payload)
                .await;
            if let Err(error) = publish_result {
                error!("error occured while publishing the elevator trip status to app error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
                return;
            }
        }
        Err(error) => error!("error occured while encoding the elevator trip status message to bytes error -> {:?} unit_id -> {}",error,unit_subscribe_topic),
    }
}

pub async fn rotor_trip_update_handler<T: UnitRepository>(
    client: Arc<AsyncClient>,
    unit_subscribe_topic: String,
    status: String,
    unit_repo: T,
) {
    let status: u8 = status.parse().unwrap_or(0 as u8);
    let status = if status == 0 { false } else { true };
    let update_rotor_trip_usecase = UpdateRotorTripUseCase::new(unit_repo);
    let update_result = update_rotor_trip_usecase
        .update_rotor_trip(&unit_subscribe_topic, status)
        .await;

    if let Err(error) = update_result {
        error!(
            "error occurred while updating the rotor trip status to database error -> {:?} unit_id -> {}",
            error, &unit_subscribe_topic
        );
        return;
    }

    let app_publish_topic = format!("app/{}", &unit_subscribe_topic);
    let app_publish_json_message = json!({"message_type":4,"status":status});

    let bytes_encode_result = serde_json::to_vec(&app_publish_json_message);

    match bytes_encode_result {
        Ok(payload) => {
            let publish_result=client.publish(app_publish_topic,QoS::AtLeastOnce, false, payload).await;
            if let Err(error) = publish_result {
                error!("error occured while publishing rotor trip status to app error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
                return;
            }
        },
        Err(error) => error!("error occurred while encoding rotor trip status message to bytes error -> {:?} unit_id -> {}",error,unit_subscribe_topic)
    }
}

pub async fn blower_run_update_handler<T: UnitRepository>(
    client: Arc<AsyncClient>,
    unit_subscribe_topic: String,
    status: String,
    unit_repo: T,
) {
    let status: u8 = status.parse().unwrap_or(0 as u8);
    let status = if status == 0 { false } else { true };
    let update_blower_run_usecase = UpdateBlowerRunUseCase::new(unit_repo);

    let update_result = update_blower_run_usecase
        .update_blower_run(&unit_subscribe_topic, status)
        .await;

    if let Err(error) = update_result {
        error!("error occurred while updating the blower run status to database error -> {:?} unit_id -> {}",error,&unit_subscribe_topic);
        return;
    }

    let app_publish_topic = format!("app/{}", &unit_subscribe_topic);
    let app_publish_json_message = json!({"message_type":5,"status":status});

    let bytes_encode_result = serde_json::to_vec(&app_publish_json_message);

    match bytes_encode_result {
        Ok(payload) => {
            let publish_result = client.publish(app_publish_topic, QoS::AtLeastOnce, false, payload).await;
            if let Err(error) = publish_result {
                error!("error occurred while publishing the blower run status to app error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
                return;
            }
        },
        Err(error) => error!("error occured while encoding blower run status message to bytes error -> {:?} unit_id -> {}",error,unit_subscribe_topic)
    }
}

pub async fn elevator_run_update_handler<T: UnitRepository>(
    client: Arc<AsyncClient>,
    unit_subscribe_topic: String,
    status: String,
    unit_repo: T,
) {
    let status: u8 = status.parse().unwrap_or(0 as u8);
    let status = if status == 0 { false } else { true };
    let update_elevator_run_usecase = UpdateElevatorRunUseCase::new(unit_repo);

    let update_result = update_elevator_run_usecase
        .update_elevator_run(&unit_subscribe_topic, status)
        .await;

    if let Err(error) = update_result {
        error!("error occurred while updating the elevator run status to database error -> {:?} unit_id -> {}",error,&unit_subscribe_topic);
        return;
    }

    let app_publish_topic = format!("app/{}", &unit_subscribe_topic);
    let app_publish_json_message = json!({"message_type": 6,"status":status});

    let bytes_encode_result = serde_json::to_vec(&app_publish_json_message);

    match bytes_encode_result {
        Ok(payload) => {
            let publish_result = client
                .publish(app_publish_topic, QoS::AtLeastOnce, false, payload)
                .await;
            if let Err(error) = publish_result {
                error!("error occured while publishing the elevator run status to app error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
                return;
            }
        },
        Err(error) => error!("error occurred while encoding the elevator run status message to bytes error -> {:?} unit_id -> {}",error,unit_subscribe_topic)
    }
}

pub async fn rotor_run_update_handler<T: UnitRepository>(
    client: Arc<AsyncClient>,
    unit_subscribe_topic: String,
    status: String,
    unit_repo: T,
) {
    let status: u8 = status.parse().unwrap_or(0 as u8);
    let status = if status == 0 { false } else { true };
    let update_rotor_run_usecase = UpdateRotorRunUseCase::new(unit_repo);
    let update_result = update_rotor_run_usecase
        .update_rotor_run(&unit_subscribe_topic, status)
        .await;

    if let Err(error) = update_result {
        error!("error occured while updating the rotor run status to database error -> {:?} unit_id -> {}",error,&unit_subscribe_topic);
        return;
    }

    let app_publish_topic = format!("app/{}", &unit_subscribe_topic);
    let app_publish_json_message = json!({"message_type":7,"status":status});

    let bytes_encode_result = serde_json::to_vec(&app_publish_json_message);

    match bytes_encode_result {
        Ok(payload) => {
            let publish_result = client.publish(app_publish_topic,QoS::AtLeastOnce, false, payload).await;

            if let Err(error) = publish_result {
                error!("error occured while publishing the rotor run status to app error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
                return;
            }
        },
        Err(error) => error!("error occurred while encoding rotor run status message to bytes error -> {:?} unit_id -> {}",error,unit_subscribe_topic)
    }
}
