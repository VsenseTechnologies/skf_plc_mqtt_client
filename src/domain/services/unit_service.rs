use crate::domain::repositories::unit_repository::UnitRepository;
use sqlx::Error as DBError;

pub struct UnitService<T: UnitRepository> {
    unit_repo: T,
}

impl<T: UnitRepository> UnitService<T> {
    pub fn new(unit_repo: T) -> Self {
        UnitService { unit_repo }
    }

    pub async fn update_temparature(
        &self,
        unit_id: String,
        temperature: String,
    ) -> Result<(), DBError> {
        self.unit_repo
            .update_temperature(unit_id, temperature)
            .await?;
        Ok(())
    }

    pub async fn update_pid_valve_opening(
        &self,
        unit_id: String,
        opening: String,
    ) -> Result<(), DBError> {
        self.unit_repo
            .update_pid_valve_opening(unit_id, opening)
            .await?;
        Ok(())
    }

    pub async fn update_blower_trip_status(
        &self,
        unit_id: String,
        status: bool,
    ) -> Result<(), DBError> {
        self.unit_repo
            .update_blower_trip_status(unit_id, status)
            .await?;
        Ok(())
    }

    pub async fn update_elevator_trip_status(
        &self,
        unit_id: String,
        status: bool,
    ) -> Result<(), DBError> {
        self.unit_repo
            .update_elevator_trip_status(unit_id, status)
            .await?;
        Ok(())
    }

    pub async fn update_rotor_trip_status(
        &self,
        unit_id: String,
        status: bool,
    ) -> Result<(), DBError> {
        self.unit_repo
            .update_rotor_trip_status(unit_id, status)
            .await?;
        Ok(())
    }

    pub async fn update_blower_run_status(
        &self,
        unit_id: String,
        status: bool,
    ) -> Result<(), DBError> {
        self.unit_repo
            .update_blower_run_status(unit_id, status)
            .await?;
        Ok(())
    }

    pub async fn update_elevator_run_status(
        &self,
        unit_id: String,
        status: bool,
    ) -> Result<(), DBError> {
        self.unit_repo
            .update_blower_run_status(unit_id, status)
            .await?;
        Ok(())
    }

    pub async fn update_rotor_run_status(
        &self,
        unit_id: String,
        status: bool,
    ) -> Result<(), DBError> {
        self.unit_repo
            .update_rotor_run_status(unit_id, status)
            .await?;
        Ok(())
    }
}
