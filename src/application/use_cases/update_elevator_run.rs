use sqlx::Error as DBError;

use crate::domain::repositories::unit_repository::UnitRepository;
use crate::domain::services::unit_service::UnitService;

pub struct UpdateElevatorRunUseCase<T: UnitRepository> {
    unit_service: UnitService<T>,
}

impl<T: UnitRepository> UpdateElevatorRunUseCase<T> {
    pub fn new(unit_repo: T) -> Self {
        let unit_service = UnitService::new(unit_repo);
        UpdateElevatorRunUseCase { unit_service }
    }

    pub async fn update_elevator_run(&self, unit_id: &String, status: bool) -> Result<(), DBError> {
        self.unit_service
            .update_elevator_run_status(unit_id, status)
            .await
    }
}
