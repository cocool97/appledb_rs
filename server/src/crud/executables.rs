use std::{path::PathBuf, str::FromStr};

use anyhow::{Result, anyhow};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait, FromQueryResult, JoinType,
    QueryFilter, QuerySelect, RelationTrait, SelectColumns,
};
use serde::Serialize;
use utoipa::ToSchema;

use crate::db_controller::DBController;

use super::DBStatus;

#[derive(FromQueryResult, ToSchema, Serialize)]
pub struct ExecutableVersion {
    pub id: i32,
    pub display_name: Option<String>,
    pub model_code: String,
    pub version: String,
}

impl DBController {
    pub async fn crud_get_executable_versions(
        &self,
        executable_id: i32,
    ) -> Result<Vec<ExecutableVersion>, DbErr> {
        entity::prelude::Device::find()
            .join(
                JoinType::LeftJoin,
                entity::device::Relation::OperatingSystemVersion.def(),
            )
            .join(
                JoinType::LeftJoin,
                entity::operating_system_version::Relation::ExecutableOperatingSystemVersion.def(),
            )
            .join(
                JoinType::LeftJoin,
                entity::executable_operating_system_version::Relation::Executable.def(),
            )
            .select_only()
            .select_column(entity::device::Column::DisplayName)
            .select_column(entity::device::Column::ModelCode)
            .select_column(entity::operating_system_version::Column::Version)
            .select_column(entity::executable_operating_system_version::Column::Id)
            .filter(entity::executable::Column::Id.eq(executable_id))
            .into_model::<ExecutableVersion>()
            .all(self.get_connection())
            .await
    }

    pub async fn crud_get_or_create_executable<S: ToString>(
        &self,
        operating_system_version_id: i32,
        full_path: S,
    ) -> Result<DBStatus> {
        let executable_name = full_path.to_string();
        let executable_name = PathBuf::from_str(executable_name.as_str())?;

        let executable_name = executable_name.file_name().ok_or(anyhow!(
            "cannot get file name from path {}",
            full_path.to_string()
        ))?;

        // Create executable
        let executable = if let Some(executable) = entity::prelude::Executable::find()
            .filter(entity::executable::Column::Name.eq(executable_name.to_string_lossy()))
            .filter(entity::executable::Column::FullPath.eq(full_path.to_string()))
            .one(self.get_connection())
            .await?
        {
            executable
        } else {
            let executable = entity::executable::ActiveModel {
                id: ActiveValue::NotSet,
                full_path: ActiveValue::Set(full_path.to_string()),
                name: ActiveValue::Set(executable_name.to_string_lossy().to_string()),
            };

            executable.insert(self.get_connection()).await?
        };

        // Create executable <-> operating_
        Ok(self
            .crud_get_or_create_executable_operating_system_version(
                executable.id,
                operating_system_version_id,
            )
            .await?)
    }
}
