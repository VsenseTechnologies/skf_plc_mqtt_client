mod application;
mod domain;
mod infrastructure;
mod logger;
mod presentation;

use dotenv::dotenv;

use crate::infrastructure::app::run;
use logger::logger::init_logger;

#[tokio::main]
async fn main() {
    init_logger().expect("failed to intialize logger");

    dotenv().ok();

    run().await;
}
