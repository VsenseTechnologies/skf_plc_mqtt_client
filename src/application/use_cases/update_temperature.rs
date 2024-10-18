use sqlx::Error as DBError;

use crate::domain::repositories::unit_repository::UnitRepository;
use crate::domain::services::unit_service::UnitService;

pub struct UpdateTemperatureUseCase<T: UnitRepository> {
    unit_service: UnitService<T>,
}

impl<T: UnitRepository> UpdateTemperatureUseCase<T> {
    pub fn new(unit_repo: T) -> Self {
        let unit_service = UnitService::new(unit_repo);
        UpdateTemperatureUseCase { unit_service }
    }

    pub async fn update_temperature(
        &self,
        unit_id: &String,
        temperature: &String,
    ) -> Result<(), DBError> {
        self.unit_service
            .update_temparature(unit_id, temperature)
            .await
    }
}
