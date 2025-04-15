use std::path::Path;

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait, QueryFilter, SqlErr,
};

use crate::db_controller::DBController;

use super::DBStatus;

impl DBController {
    pub async fn crud_get_or_create_framework<P: AsRef<Path>>(
        &self,
        framework_full_path: P,
    ) -> Result<DBStatus, DbErr> {
        let framework_full_path = framework_full_path.as_ref();
        let full_path_str = framework_full_path.display().to_string();

        let new_framework = entity::framework::ActiveModel {
            id: ActiveValue::NotSet,
            full_path: ActiveValue::Set(full_path_str.clone()),
        };

        match new_framework.insert(self.get_connection()).await {
            Ok(inserted) => Ok(DBStatus::Created(inserted.id)),
            Err(db_err) => {
                if let Some(SqlErr::UniqueConstraintViolation(_)) = db_err.sql_err() {
                    let existing = entity::prelude::Framework::find()
                        .filter(entity::framework::Column::FullPath.eq(&full_path_str))
                        .one(self.get_connection())
                        .await?
                        .ok_or_else(|| {
                            DbErr::Custom("Framework exists but can't be retrieved after unique constraint violation".into())
                        })?;

                    return Ok(DBStatus::AlreadyExists(existing.id));
                }

                Err(db_err)
            }
        }
    }
}
