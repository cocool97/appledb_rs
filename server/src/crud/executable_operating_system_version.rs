use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait, QueryFilter, SqlErr,
};

use crate::db_controller::DBController;

use super::DBStatus;

impl DBController {
    pub async fn crud_get_or_create_executable_operating_system_version(
        &self,
        executable_id: i64,
        operating_system_version_id: i64,
    ) -> Result<DBStatus, DbErr> {
        let executable_os = entity::executable_operating_system_version::ActiveModel {
            id: ActiveValue::NotSet,
            executable_id: ActiveValue::Set(executable_id),
            operating_system_version_id: ActiveValue::Set(operating_system_version_id),
        };

        match executable_os.insert(self.get_connection()).await {
            Ok(inserted) => Ok(DBStatus::Created(inserted.id)),
            Err(db_err) => {
                if let Some(SqlErr::UniqueConstraintViolation(_)) = db_err.sql_err() {
                    let existing = entity::prelude::ExecutableOperatingSystemVersion::find()
                        .filter(entity::executable_operating_system_version::Column::ExecutableId.eq(executable_id))
                        .filter(entity::executable_operating_system_version::Column::OperatingSystemVersionId.eq(operating_system_version_id))
                        .one(self.get_connection())
                        .await?
                        .ok_or_else(|| {
                            DbErr::Custom(
                                "Entry exists but cannot be retrieved after unique constraint violation".into(),
                            )
                        })?;

                    Ok(DBStatus::AlreadyExists(existing.id))
                } else {
                    Err(db_err)
                }
            }
        }
    }
}
