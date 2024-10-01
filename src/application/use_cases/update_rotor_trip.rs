use sqlx::Error as DBError;

use crate::domain::repositories::unit_repository::UnitRepository;
use crate::domain::services::unit_service::UnitService;

pub struct UpdateRotorTripUseCase<T: UnitRepository> {
    unit_service: UnitService<T>,
}

impl<T: UnitRepository> UpdateRotorTripUseCase<T> {
    pub fn new(unit_repo: T) -> Self {
        let unit_service = UnitService::new(unit_repo);
        UpdateRotorTripUseCase { unit_service }
    }

    pub async fn update_rotor_trip(&self, unit_id: String, status: bool) -> Result<(), DBError> {
        self.unit_service
            .update_rotor_trip_status(unit_id, status)
            .await
    }
}
