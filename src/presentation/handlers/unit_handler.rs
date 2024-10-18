use log::error;
use rumqttc::AsyncClient;
use rumqttc::QoS;
use serde_json::json;
use std::sync::Arc;

pub async fn temperature_stream_handler(
    client: Arc<AsyncClient>,
    unit_subscribe_topic: String,
    temperature: String,
) {
    let app_publish_topic = format!("app/{}", &unit_subscribe_topic);
    let app_publish_json_message = json!({"mt":"0","tmp":temperature});

    let bytes_encode_result = serde_json::to_vec(&app_publish_json_message);

    match bytes_encode_result {
        Ok(payload) => {
            let publish_result = client.publish(app_publish_topic, QoS::AtMostOnce, false, payload).await;
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
    let app_publish_json_message = json!({"mt":"1","pid":opening});

    let bytes_encode_result = serde_json::to_vec(&app_publish_json_message);

    match bytes_encode_result {
        Ok(payload) => {
            let publish_result = client.publish(app_publish_topic, QoS::AtMostOnce,false, payload).await;
            if let Err(error) = publish_result {
                error!("error occurred while publishing the pid valve opening stream message to app error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
                return;
            }
        },
        Err(error) => error!("error occurred while encoding pid valve opening stream message to bytes error -> {:?} unit_id -> {}",error,unit_subscribe_topic)
    }
}

pub async fn step1_temperature_stream_handler(
    client: Arc<AsyncClient>,
    unit_subscribe_topic: String,
    temperature: String,
) {
    let app_publish_topic = format!("app/{}", &unit_subscribe_topic);
    let app_publish_json_message = json!({"mt":"2","tmp":temperature});

    let bytes_encode_result = serde_json::to_vec(&app_publish_json_message);

    match bytes_encode_result {
        Ok(payload) => {
            let publish_result = client
                .publish(app_publish_topic, QoS::AtMostOnce, false, payload)
                .await;
            if let Err(error) = publish_result {
                error!("error occurred while publishing the step1 temperature message error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
            }
        }
        Err(error) => {
            error!("error occurred while encoding step1 temperature message to bytes error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
        }
    }
}

pub async fn step2_temperature_stream_handler(
    client: Arc<AsyncClient>,
    unit_subscribe_topic: String,
    temperature: String,
) {
    let app_publish_topic = format!("app/{}", &unit_subscribe_topic);
    let app_publish_json_message = json!({"mt":"3","tmp":temperature});

    let bytes_encode_result = serde_json::to_vec(&app_publish_json_message);

    match bytes_encode_result {
        Ok(payload) => {
            let publish_result = client
                .publish(app_publish_topic, QoS::AtMostOnce, false, payload)
                .await;
            if let Err(error) = publish_result {
                error!("error occurred while publishing the step2 temperature message error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
            }
        }
        Err(error) => {
            error!("error occurred while encoding step2 temperature message to bytes error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
        }
    }
}

pub async fn step3_temperature_stream_handler(
    client: Arc<AsyncClient>,
    unit_subscribe_topic: String,
    temperature: String,
) {
    let app_publish_topic = format!("app/{}", &unit_subscribe_topic);
    let app_publish_json_message = json!({"mt":"4","tmp":temperature});

    let bytes_encode_result = serde_json::to_vec(&app_publish_json_message);

    match bytes_encode_result {
        Ok(payload) => {
            let publish_result = client
                .publish(app_publish_topic, QoS::AtMostOnce, false, payload)
                .await;
            if let Err(error) = publish_result {
                error!("error occurred while publishing the step3 temperature message error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
            }
        }
        Err(error) => {
            error!("error occurred while encoding step3 temperature message to bytes error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
        }
    }
}

pub async fn step4_temperature_stream_handler(
    client: Arc<AsyncClient>,
    unit_subscribe_topic: String,
    temperature: String,
) {
    let app_publish_topic = format!("app/{}", &unit_subscribe_topic);
    let app_publish_json_message = json!({"mt":"5","tmp":temperature});

    let bytes_encode_result = serde_json::to_vec(&app_publish_json_message);

    match bytes_encode_result {
        Ok(payload) => {
            let publish_result = client
                .publish(app_publish_topic, QoS::AtMostOnce, false, payload)
                .await;
            if let Err(error) = publish_result {
                error!("error occurred while publishing the step4 temperature message error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
            }
        }
        Err(error) => {
            error!("error occurred while encoding step4 temperature message to bytes error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
        }
    }
}

pub async fn step1_time_stream_handler(
    client: Arc<AsyncClient>,
    unit_subscribe_topic: String,
    time: String,
) {
    let app_publish_topic = format!("app/{}", &unit_subscribe_topic);
    let app_publish_json_message = json!({"mt":"6","tm":time});

    let bytes_encode_result = serde_json::to_vec(&app_publish_json_message);

    match bytes_encode_result {
        Ok(payload) => {
            let publish_result = client
                .publish(app_publish_topic, QoS::AtMostOnce, false, payload)
                .await;
            if let Err(error) = publish_result {
                error!("error occurred while publishing the step1 time message error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
            }
        }
        Err(error) => {
            error!("error occurred while encoding step1 time message to bytes error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
        }
    }
}

pub async fn step2_time_stream_handler(
    client: Arc<AsyncClient>,
    unit_subscribe_topic: String,
    time: String,
) {
    let app_publish_topic = format!("app/{}", &unit_subscribe_topic);
    let app_publish_json_message = json!({"mt":"7","tm":time});

    let bytes_encode_result = serde_json::to_vec(&app_publish_json_message);

    match bytes_encode_result {
        Ok(payload) => {
            let publish_result = client
                .publish(app_publish_topic, QoS::AtMostOnce, false, payload)
                .await;
            if let Err(error) = publish_result {
                error!("error occurred while publishing the step2 time message error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
            }
        }
        Err(error) => {
            error!("error occurred while encoding step2 time message to bytes error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
        }
    }
}

pub async fn step3_time_stream_handler(
    client: Arc<AsyncClient>,
    unit_subscribe_topic: String,
    time: String,
) {
    let app_publish_topic = format!("app/{}", &unit_subscribe_topic);
    let app_publish_json_message = json!({"mt":"8","tm":time});

    let bytes_encode_result = serde_json::to_vec(&app_publish_json_message);

    match bytes_encode_result {
        Ok(payload) => {
            let publish_result = client
                .publish(app_publish_topic, QoS::AtMostOnce, false, payload)
                .await;
            if let Err(error) = publish_result {
                error!("error occurred while publishing the step3 time message error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
            }
        }
        Err(error) => {
            error!("error occurred while encoding step3 time message to bytes error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
        }
    }
}

pub async fn step4_time_stream_handler(
    client: Arc<AsyncClient>,
    unit_subscribe_topic: String,
    time: String,
) {
    let app_publish_topic = format!("app/{}", &unit_subscribe_topic);
    let app_publish_json_message = json!({"mt":"9","tm":time});

    let bytes_encode_result = serde_json::to_vec(&app_publish_json_message);

    match bytes_encode_result {
        Ok(payload) => {
            let publish_result = client
                .publish(app_publish_topic, QoS::AtMostOnce, false, payload)
                .await;
            if let Err(error) = publish_result {
                error!("error occurred while publishing the step4 time message error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
            }
        }
        Err(error) => {
            error!("error occurred while encoding step4 time message to bytes error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
        }
    }
}

pub async fn blower_trip_stream_handler(
    client: Arc<AsyncClient>,
    unit_subscribe_topic: String,
    status: String,
) {
    let app_publish_topic = format!("app/{}", &unit_subscribe_topic);
    let app_publish_json_message = json!({"mt":"10","st":status});

    let bytes_encode_result = serde_json::to_vec(&app_publish_json_message);

    match bytes_encode_result {
        Ok(payload) => {
            let publish_result = client
                .publish(app_publish_topic, QoS::AtLeastOnce, false, payload)
                .await;
            if let Err(error) = publish_result {
                error!("error occurred while publishing the blower trip status message error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
            }
        }
        Err(error) => {
            error!("error occurred while encoding blower trip message to bytes error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
        }
    }
}

pub async fn elevator_trip_stream_handler(
    client: Arc<AsyncClient>,
    unit_subscribe_topic: String,
    status: String,
) {
    let app_publish_topic = format!("app/{}", &unit_subscribe_topic);
    let app_publish_json_message = json!({"mt":"11","st":status});

    let bytes_encode_result = serde_json::to_vec(&app_publish_json_message);

    match bytes_encode_result {
        Ok(payload) => {
            let publish_result = client
                .publish(app_publish_topic, QoS::AtLeastOnce, false, payload)
                .await;
            if let Err(error) = publish_result {
                error!("error occurred while publishing the elevator trip status message error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
            }
        }
        Err(error) => {
            error!("error occurred while encoding elevator trip status message to bytes error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
        }
    }
}

pub async fn rotor_trip_stream_handler(
    client: Arc<AsyncClient>,
    unit_subscribe_topic: String,
    status: String,
) {
    let app_publish_topic = format!("app/{}", &unit_subscribe_topic);
    let app_publish_json_message = json!({"mt":"12","st":status});

    let bytes_encode_result = serde_json::to_vec(&app_publish_json_message);

    match bytes_encode_result {
        Ok(payload) => {
            let publish_result = client
                .publish(app_publish_topic, QoS::AtLeastOnce, false, payload)
                .await;
            if let Err(error) = publish_result {
                error!("error occurred while publishing the rotor trip status message error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
            }
        }
        Err(error) => {
            error!("error occurred while encoding rotor trip status message to bytes error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
        }
    }
}

pub async fn blower_run_stream_handler(
    client: Arc<AsyncClient>,
    unit_subscribe_topic: String,
    status: String,
) {
    let app_publish_topic = format!("app/{}", &unit_subscribe_topic);
    let app_publish_json_message = json!({"mt":"13","st":status});

    let bytes_encode_result = serde_json::to_vec(&app_publish_json_message);

    match bytes_encode_result {
        Ok(payload) => {
            let publish_result = client
                .publish(app_publish_topic, QoS::AtLeastOnce, false, payload)
                .await;
            if let Err(error) = publish_result {
                error!("error occurred while publishing the blower run status message error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
            }
        }
        Err(error) => {
            error!("error occurred while encoding blower run status message to bytes error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
        }
    }
}

pub async fn elevator_run_stream_handler(
    client: Arc<AsyncClient>,
    unit_subscribe_topic: String,
    status: String,
) {
    let app_publish_topic = format!("app/{}", &unit_subscribe_topic);
    let app_publish_json_message = json!({"mt":"14","st":status});

    let bytes_encode_result = serde_json::to_vec(&app_publish_json_message);

    match bytes_encode_result {
        Ok(payload) => {
            let publish_result = client
                .publish(app_publish_topic, QoS::AtLeastOnce, false, payload)
                .await;
            if let Err(error) = publish_result {
                error!("error occurred while publishing the elevator run status message error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
            }
        }
        Err(error) => {
            error!("error occurred while encoding elevator run status message to bytes error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
        }
    }
}

pub async fn rotor_run_stream_handler(
    client: Arc<AsyncClient>,
    unit_subscribe_topic: String,
    status: String,
) {
    let app_publish_topic = format!("app/{}", &unit_subscribe_topic);
    let app_publish_json_message = json!({"mt":"15","st":status});

    let bytes_encode_result = serde_json::to_vec(&app_publish_json_message);

    match bytes_encode_result {
        Ok(payload) => {
            let publish_result = client
                .publish(app_publish_topic, QoS::AtLeastOnce, false, payload)
                .await;
            if let Err(error) = publish_result {
                error!("error occurred while publishing the rotor run status message error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
            }
        }
        Err(error) => {
            error!("error occurred while encoding rotor run status message to bytes error -> {:?} unit_id -> {}",error,unit_subscribe_topic);
        }
    }
}
