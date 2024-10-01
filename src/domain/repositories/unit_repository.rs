use async_trait::async_trait;
use sqlx::Error as DBError;

#[async_trait]
pub trait UnitRepository {
    async fn update_temperature(&self, unit_id: String, temperature: String)
        -> Result<(), DBError>;

    async fn update_pid_valve_opening(
        &self,
        unit_id: String,
        opening: String,
    ) -> Result<(), DBError>;

    async fn update_blower_trip_status(&self, unit_id: String, status: bool)
        -> Result<(), DBError>;

    async fn update_elevator_trip_status(
        &self,
        unit_id: String,
        status: bool,
    ) -> Result<(), DBError>;

    async fn update_rotor_trip_status(&self, unit_id: String, status: bool) -> Result<(), DBError>;

    async fn update_blower_run_status(&self, unit_id: String, status: bool) -> Result<(), DBError>;

    async fn update_elevator_run_status(
        &self,
        unit_id: String,
        status: bool,
    ) -> Result<(), DBError>;

    async fn update_rotor_run_status(&self, unit_id: String, status: bool) -> Result<(), DBError>;
}
