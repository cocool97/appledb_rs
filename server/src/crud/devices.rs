use appledb_common::db_models::Device;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};

use crate::Result;
use crate::db_controller::DBController;

use super::DBStatus;

impl DBController {
    pub async fn crud_get_devices(&self) -> Result<Vec<Device>> {
        Ok(entity::prelude::Device::find()
            .all(self.get_connection())
            .await?
            .into_iter()
            .map(Device::from)
            .collect::<Vec<Device>>())
    }

    pub async fn crud_get_or_create_device<S: ToString>(&self, model: S) -> Result<DBStatus> {
        if let Some(device) = entity::prelude::Device::find()
            .filter(entity::device::Column::Model.eq(model.to_string()))
            .one(self.get_connection())
            .await?
        {
            // Already exists in DB
            return Ok(DBStatus::AlreadyExists(device.id));
        }

        let device = entity::device::ActiveModel {
            id: ActiveValue::NotSet,
            model: ActiveValue::Set(model.to_string()),
            display_name: ActiveValue::NotSet,
        };

        let res = device.insert(self.get_connection()).await?;

        Ok(DBStatus::Created(res.id))
    }
}
