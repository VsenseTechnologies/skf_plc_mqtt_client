use sqlx::Error as DBError;

use crate::domain::repositories::unit_repository::UnitRepository;
use crate::domain::services::unit_service::UnitService;

pub struct UpdateRotorRunUseCase<T: UnitRepository> {
    unit_service: UnitService<T>,
}

impl <T: UnitRepository> UpdateRotorRunUseCase<T> {
    pub fn new(unit_repo) -> Self {
        let unit_service = UnitService::new(unit_repo);
        UpdateRotorRunUseCase { unit_service }
    }

    pub async fn update_rotor_run(&self, unit_id: String, status: bool) -> Result<(),DBError> {
        self.unit_service.update_rotor_run_status(unit_id, status).await
    }
}
