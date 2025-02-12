use appledb_common::db_models::OperatingSystemVersion;

use anyhow::{Result, anyhow};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};

use crate::db_controller::DBController;

impl DBController {
    pub async fn crud_get_operating_system_version(&self) -> Result<Vec<OperatingSystemVersion>> {
        Ok(entity::prelude::OperatingSystemVersion::find()
            .all(self.get_connection())
            .await?
            .into_iter()
            .map(OperatingSystemVersion::from)
            .collect::<Vec<OperatingSystemVersion>>())
    }

    pub async fn crud_get_or_create_operating_system_version_by_platform_and_version(
        &self,
        platform_name: String,
        version: String,
    ) -> Result<OperatingSystemVersion> {
        let operating_system_version = match entity::prelude::OperatingSystemVersion::find()
            .filter(entity::operating_system_version::Column::Version.eq(&version))
            .find_also_related(entity::prelude::OperatingSystem)
            .filter(entity::operating_system::Column::Name.eq(&platform_name))
            .one(self.get_connection())
            .await?
        {
            Some((operating_system_version, _)) => operating_system_version,
            None => {
                let operating_system = entity::prelude::OperatingSystem::find()
                    .filter(entity::operating_system::Column::Name.eq(&platform_name))
                    .one(self.get_connection())
                    .await?
                    .ok_or(anyhow!("Operating system not found"))?;

                let operating_system_version = entity::operating_system_version::ActiveModel {
                    id: ActiveValue::NotSet,
                    operating_system_id: ActiveValue::Set(operating_system.id),
                    version: ActiveValue::Set(version.clone()),
                };

                let res = operating_system_version
                    .insert(self.get_connection())
                    .await?;

                log::info!("Created new operating system version {version} for {platform_name}");

                res
            }
        };

        Ok(operating_system_version.into())
    }

    pub async fn crud_get_operating_system_version_by_id(
        &self,
        id: i32,
    ) -> Result<OperatingSystemVersion> {
        let conn = self.get_connection();
        let operating_system_version = entity::prelude::OperatingSystemVersion::find_by_id(id)
            .one(conn)
            .await?
            .ok_or(anyhow!("Operating system version not found"))?;

        Ok(operating_system_version.into())
    }

    pub async fn crud_create_operating_system_version(
        &self,
        operating_system_id: i32,
        version: String,
    ) -> Result<i32> {
        let operating_system_version = entity::operating_system_version::ActiveModel {
            id: ActiveValue::NotSet,
            operating_system_id: ActiveValue::set(operating_system_id),
            version: ActiveValue::Set(version),
        };

        let res = operating_system_version
            .insert(self.get_connection())
            .await?;

        Ok(res.id)
    }
}
