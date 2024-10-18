use sqlx::Error as DBError;

use crate::domain::repositories::unit_repository::UnitRepository;
use crate::domain::services::unit_service::UnitService;

pub struct UpdatePIDValveUseCase<T: UnitRepository> {
    unit_service: UnitService<T>,
}

impl<T: UnitRepository> UpdatePIDValveUseCase<T> {
    pub fn new(unit_repo: T) -> Self {
        let unit_service = UnitService::new(unit_repo);
        UpdatePIDValveUseCase { unit_service }
    }

    pub async fn update_pid_valve(
        &self,
        unit_id: &String,
        opening: &String,
    ) -> Result<(), DBError> {
        self.unit_service
            .update_pid_valve_opening(unit_id, opening)
            .await
    }
}
