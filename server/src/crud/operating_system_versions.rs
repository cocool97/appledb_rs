use appledb_common::{
    api_models::ExtendedOperatingSystemVersions,
    db_models::{Executable, OperatingSystemVersion},
};

use anyhow::{Result, anyhow};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait, JoinType, PaginatorTrait,
    QueryFilter, QuerySelect, RelationTrait,
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
        let operating_system_version = match entity::prelude::OperatingSystemVersion::find()
            .filter(entity::operating_system_version::Column::Version.eq(&version))
            .find_also_related(entity::prelude::OperatingSystem)
            .find_also_related(entity::prelude::Device)
            .filter(entity::operating_system::Column::Name.eq(&platform_name))
            .filter(entity::device::Column::ModelCode.eq(&model_code))
            .one(self.get_connection())
            .await?
        {
            Some((operating_system_version, _, _)) => operating_system_version,
            None => {
                let operating_system = entity::prelude::OperatingSystem::find()
                    .filter(entity::operating_system::Column::Name.eq(&platform_name))
                    .one(self.get_connection())
                    .await?
                    .ok_or(anyhow!("Operating system not found"))?;

                let device_id = self
                    .crud_get_or_create_device(model_code)
                    .await?
                    .db_identifier();

                let operating_system_version = entity::operating_system_version::ActiveModel {
                    id: ActiveValue::NotSet,
                    device_id: ActiveValue::Set(device_id),
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
        device_id: i32,
        version: String,
    ) -> Result<i32, DbErr> {
        let operating_system_version = entity::operating_system_version::ActiveModel {
            id: ActiveValue::NotSet,
            device_id: ActiveValue::Set(device_id),
            operating_system_id: ActiveValue::set(operating_system_id),
            version: ActiveValue::Set(version),
        };

        let res = operating_system_version
            .insert(self.get_connection())
            .await?;

        Ok(res.id)
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
