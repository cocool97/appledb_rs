use entity::executable_framework::ActiveModel;
use sea_orm::{ActiveModelTrait, DbErr, SqlErr};

use crate::db_controller::DBController;

impl DBController {
    pub async fn crud_create_executable_framework(
        &self,
        executable_operating_system_version_id: i64,
        framework_id: i64,
    ) -> Result<(), DbErr> {
        let executable_framework = ActiveModel {
            executable_operating_system_version_id: sea_orm::ActiveValue::Set(
                executable_operating_system_version_id,
            ),
            framework_id: sea_orm::ActiveValue::Set(framework_id),
        };

        match executable_framework.insert(self.get_connection()).await {
            Ok(_) => Ok(()),
            Err(db_err) => {
                if let Some(SqlErr::UniqueConstraintViolation(_)) = db_err.sql_err() {
                    log::debug!(
                        "executable / framework association already exists {} / {}",
                        executable_operating_system_version_id,
                        framework_id
                    );
                    Ok(())
                } else {
                    Err(db_err)
                }
            }
        }
    }
}
