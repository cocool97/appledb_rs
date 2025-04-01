use anyhow::anyhow;
use appledb_common::db_models::{Device, OperatingSystemVersion};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder,
};

use crate::db_controller::DBController;
use crate::{APPLE_MODELS, Result};

use super::DBStatus;

impl DBController {
    pub async fn crud_get_devices(&self) -> Result<Vec<Device>, DbErr> {
        Ok(entity::prelude::Device::find()
            .order_by_desc(entity::device::Column::ModelCode)
            .all(self.get_connection())
            .await?
            .into_iter()
            .map(Device::from)
            .collect::<Vec<Device>>())
    }

    pub async fn crud_get_devices_count(&self) -> Result<u64, DbErr> {
        entity::prelude::Device::find()
            .count(self.get_connection())
            .await
    }

    pub async fn crud_get_or_create_device<S: ToString>(
        &self,
        model: S,
    ) -> Result<DBStatus, DbErr> {
        let model_code = model.to_string();
        if let Some(device) = entity::prelude::Device::find()
            .filter(entity::device::Column::ModelCode.eq(&model_code))
            .one(self.get_connection())
            .await?
        {
            // Already exists in DB
            return Ok(DBStatus::AlreadyExists(device.id));
        }

        let display_name = if let Some(display_name) = APPLE_MODELS.get(&model_code) {
            log::info!("Found display name for device {model_code} -> {display_name}");
            Some(display_name.clone())
        } else {
            None
        };

        let device = entity::device::ActiveModel {
            id: ActiveValue::NotSet,
            model_code: ActiveValue::Set(model_code),
            display_name: ActiveValue::Set(display_name),
        };

        let res = device.insert(self.get_connection()).await?;

        Ok(DBStatus::Created(res.id))
    }

    pub async fn crud_get_device_operating_system_versions(
        &self,
        device_id: i32,
    ) -> Result<Vec<OperatingSystemVersion>, DbErr> {
        Ok(entity::prelude::OperatingSystemVersion::find()
            .filter(entity::operating_system_version::Column::DeviceId.eq(device_id))
            .all(self.get_connection())
            .await?
            .into_iter()
            .map(OperatingSystemVersion::from)
            .collect::<Vec<OperatingSystemVersion>>())
    }

    pub async fn crud_set_devices_unknown_display_names(&self) -> Result<()> {
        let missing_display_names = entity::prelude::Device::find()
            .filter(entity::device::Column::DisplayName.is_null())
            .all(self.get_connection())
            .await?;

        for model in missing_display_names {
            let mut model: entity::device::ActiveModel = model.into();
            let model_code = model
                .model_code
                .try_as_ref()
                .ok_or(anyhow!("missing model value..."))?;

            match APPLE_MODELS.get(model_code) {
                Some(display_name) => {
                    model.display_name = ActiveValue::Set(Some(display_name.clone()));
                    model.update(self.get_connection()).await?;
                }
                None => {
                    log::error!("Unknown device model {model_code}...")
                }
            }
        }

        Ok(())
    }
}
