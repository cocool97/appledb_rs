use std::path::Path;

use anyhow::{Result, anyhow};
use appledb_common::db_models::Executable;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait, FromQueryResult, JoinType,
    QueryFilter, QuerySelect, RelationTrait, SelectColumns, SqlErr,
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
    pub async fn crud_get_all_executables(&self) -> Result<Vec<Executable>> {
        Ok(entity::prelude::Executable::find()
            .all(self.get_connection())
            .await?
            .into_iter()
            .map(Executable::from)
            .collect())
    }

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

    pub async fn crud_get_or_create_executable<P: AsRef<Path>>(
        &self,
        operating_system_version_id: i32,
        full_path: P,
    ) -> Result<DBStatus> {
        let full_path = full_path.as_ref();

        let executable_name = full_path
            .file_name()
            .ok_or_else(|| {
                anyhow!(
                    "Cannot extract file name from path: {}",
                    full_path.display()
                )
            })?
            .to_string_lossy()
            .to_string();

        let new_executable = entity::executable::ActiveModel {
            id: ActiveValue::NotSet,
            full_path: ActiveValue::Set(full_path.display().to_string()),
            name: ActiveValue::Set(executable_name.clone()),
        };

        let executable = match new_executable.insert(self.get_connection()).await {
            Ok(inserted) => inserted,
            Err(db_err) => {
                if let Some(SqlErr::UniqueConstraintViolation(_)) = db_err.sql_err() {
                    entity::prelude::Executable::find()
                        .filter(entity::executable::Column::Name.eq(&executable_name))
                        .filter(entity::executable::Column::FullPath.eq(full_path.to_string_lossy()))
                        .one(self.get_connection())
                        .await?
                        .ok_or_else(|| {
                            anyhow!("Executable exists but can't be retrieved after unique constraint violation")
                        })?
                } else {
                    return Err(db_err.into());
                }
            }
        };

        Ok(self
            .crud_get_or_create_executable_operating_system_version(
                executable.id,
                operating_system_version_id,
            )
            .await?)
    }
}
