use appledb_common::{
    api_models::ExtendedOperatingSystemVersions,
    db_models::{Executable, OperatingSystemVersion},
};

use anyhow::{Result, anyhow};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait, JoinType, PaginatorTrait,
    QueryFilter, QuerySelect, RelationTrait, SqlErr,
};

use crate::db_controller::DBController;

impl DBController {
    pub async fn crud_get_operating_system_version(
        &self,
    ) -> Result<Vec<OperatingSystemVersion>, DbErr> {
        Ok(entity::prelude::OperatingSystemVersion::find()
            .all(self.get_connection())
            .await?
            .into_iter()
            .map(OperatingSystemVersion::from)
            .collect::<Vec<OperatingSystemVersion>>())
    }

    pub async fn crud_get_operating_system_version_count(&self) -> Result<u64, DbErr> {
        entity::prelude::OperatingSystemVersion::find()
            .count(self.get_connection())
            .await
    }

    pub async fn crud_get_or_create_operating_system_version_by_platform_and_version(
        &self,
        platform_name: String,
        model_code: String,
        version: String,
    ) -> Result<OperatingSystemVersion> {
        let operating_system = entity::prelude::OperatingSystem::find()
            .filter(entity::operating_system::Column::Name.eq(&platform_name))
            .one(self.get_connection())
            .await?
            .ok_or(anyhow!("Operating system not found"))?;

        let device_id = self
            .crud_get_or_create_device(model_code.clone())
            .await?
            .db_identifier();

        let new_os_version = entity::operating_system_version::ActiveModel {
            id: ActiveValue::NotSet,
            device_id: ActiveValue::Set(device_id),
            operating_system_id: ActiveValue::Set(operating_system.id),
            version: ActiveValue::Set(version.clone()),
        };

        match new_os_version.insert(self.get_connection()).await {
            Ok(inserted) => {
                log::info!("Created new OS version {version} for {platform_name}/{model_code}");
                Ok(inserted.into())
            }
            Err(db_err) => {
                if let Some(SqlErr::UniqueConstraintViolation(_)) = db_err.sql_err() {
                    let existing = entity::prelude::OperatingSystemVersion::find()
                        .filter(entity::operating_system_version::Column::Version.eq(&version))
                        .filter(entity::operating_system_version::Column::OperatingSystemId.eq(operating_system.id))
                        .filter(entity::operating_system_version::Column::DeviceId.eq(device_id))
                        .one(self.get_connection())
                        .await?
                        .ok_or_else(|| {
                            anyhow!(
                                "OS version exists but can't be found after unique constraint violation"
                            )
                        })?;

                    return Ok(existing.into());
                }
                Err(db_err.into())
            }
        }
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

    pub async fn crud_get_operating_system_version_executables(
        &self,
        operating_system_version_id: i32,
    ) -> Result<Vec<Executable>, DbErr> {
        Ok(entity::prelude::Executable::find()
            .join(
                JoinType::LeftJoin,
                entity::executable::Relation::ExecutableOperatingSystemVersion.def(),
            )
            .join(
                JoinType::LeftJoin,
                entity::executable_operating_system_version::Relation::OperatingSystemVersion.def(),
            )
            .filter(entity::operating_system_version::Column::Id.eq(operating_system_version_id))
            .all(self.get_connection())
            .await?
            .into_iter()
            .map(Executable::from)
            .collect())
    }

    pub async fn crud_get_extended_operating_system_versions(
        &self,
    ) -> Result<Vec<ExtendedOperatingSystemVersions>, DbErr> {
        let os_versions_with_os = entity::prelude::OperatingSystemVersion::find()
            .find_also_related(entity::prelude::Device)
            .all(self.get_connection())
            .await?;

        Ok(os_versions_with_os
            .into_iter()
            .filter_map(|(os_version, device_opt)| {
                let device = match device_opt {
                    Some(device) => device,
                    None => return None,
                };

                Some(ExtendedOperatingSystemVersions::from((os_version, device)))
            })
            .collect())
    }
}
