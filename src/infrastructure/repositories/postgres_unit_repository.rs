use async_trait::async_trait;
use sqlx::query;
use sqlx::Error as DBError;
use sqlx::{Pool, Postgres};
use std::sync::Arc;

use crate::domain::repositories::unit_repository::UnitRepository;
use crate::infrastructure::db::connection::establish_connection;

pub struct PostgresUnitRepository {
    pool: Pool<Postgres>,
}

impl PostgresUnitRepository {
    pub async fn new() -> Self {
        let database_url =
            std::env::var("DATABASE_URL").expect("missing DATABASE_URL env variable");

        PostgresUnitRepository {
            pool: establish_connection(&database_url).await,
        }
    }
}

#[async_trait]
impl UnitRepository for Arc<PostgresUnitRepository> {
    async fn update_temperature(
        &self,
        unit_id: &String,
        temperature: &String,
    ) -> Result<(), DBError> {
        query!(
            "UPDATE units SET rt_temp = $1 WHERE unit_id = $2",
            temperature,
            unit_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_pid_valve_opening(
        &self,
        unit_id: &String,
        opening: &String,
    ) -> Result<(), DBError> {
        query!(
            "UPDATE units SET rt_pid = $1 WHERE unit_id = $2",
            opening,
            unit_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_blower_trip_status(
        &self,
        unit_id: &String,
        status: bool,
    ) -> Result<(), DBError> {
        query!(
            "UPDATE units SET blower_trip_fb = $1 WHERE unit_id = $2",
            status,
            unit_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_elevator_trip_status(
        &self,
        unit_id: &String,
        status: bool,
    ) -> Result<(), DBError> {
        query!(
            "UPDATE units SET elevator_trip_fb = $1 WHERE unit_id = $2",
            status,
            unit_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_rotor_trip_status(
        &self,
        unit_id: &String,
        status: bool,
    ) -> Result<(), DBError> {
        query!(
            "UPDATE units SET rotor_trip_fb = $1 WHERE unit_id = $2",
            status,
            unit_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_blower_run_status(
        &self,
        unit_id: &String,
        status: bool,
    ) -> Result<(), DBError> {
        query!(
            "UPDATE units SET blower_run_fb = $1 WHERE unit_id = $2",
            status,
            unit_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_elevator_run_status(
        &self,
        unit_id: &String,
        status: bool,
    ) -> Result<(), DBError> {
        query!(
            "UPDATE units SET elevator_run_fb = $1 WHERE unit_id = $2",
            status,
            unit_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_rotor_run_status(&self, unit_id: &String, status: bool) -> Result<(), DBError> {
        query!(
            "UPDATE units SET rotor_run_fb = $1 WHERE unit_id = $2",
            status,
            unit_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
