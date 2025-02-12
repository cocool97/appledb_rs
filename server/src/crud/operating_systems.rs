use appledb_common::db_models::OperatingSystem;

use anyhow::{Result, anyhow};
use sea_orm::EntityTrait;

use crate::db_controller::DBController;

impl DBController {
    pub async fn crud_get_operating_systems(&self) -> Result<Vec<OperatingSystem>> {
        entity::prelude::OperatingSystem::find()
            .all(self.get_connection())
            .await?
            .into_iter()
            .map(OperatingSystem::try_from)
            .collect::<Result<Vec<OperatingSystem>>>()
    }

    pub async fn crud_get_operating_system_by_id(&self, id: i32) -> Result<OperatingSystem> {
        entity::prelude::OperatingSystem::find_by_id(id)
            .one(self.get_connection())
            .await?
            .ok_or(anyhow!("unknown operating system id {id}"))?
            .try_into()
    }
}
