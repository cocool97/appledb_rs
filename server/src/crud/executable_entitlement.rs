use std::collections::HashMap;

use appledb_common::db_models::Entitlement;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait, ModelTrait, QueryFilter,
};

use crate::db_controller::DBController;

impl DBController {
    pub async fn crud_create_executable_entitlement(
        &self,
        executable_id: i32,
        entitlement_id: i32,
    ) -> Result<(), DbErr> {
        let executable_entitlement = entity::executable_entitlement::ActiveModel {
            executable_id: ActiveValue::Set(executable_id),
            entitlement_id: ActiveValue::Set(entitlement_id),
        };

        executable_entitlement.insert(self.get_connection()).await?;

        Ok(())
    }

    pub async fn crud_get_all_executables_entitlements(
        &self,
        operating_system_version_id: i32,
    ) -> Result<HashMap<String, Vec<Entitlement>>, DbErr> {
        let executables = entity::prelude::Executable::find()
            .filter(
                entity::executable::Column::OperatingSystemVersionId
                    .eq(operating_system_version_id),
            )
            .all(self.get_connection())
            .await?;

        let mut result = HashMap::new();
        for executable in executables {
            let executable_entitlements = executable
                .find_related(entity::prelude::Entitlement)
                .all(self.get_connection())
                .await?
                .into_iter()
                .map(Entitlement::from)
                .collect::<Vec<Entitlement>>();
            result.insert(executable.name, executable_entitlements);
        }

        Ok(result)
    }
}
