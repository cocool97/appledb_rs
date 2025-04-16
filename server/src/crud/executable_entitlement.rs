use std::collections::BTreeMap;

use anyhow::{Result, anyhow};
use appledb_common::{api_models::ExecutableInfos, db_models::Entitlement};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait, JoinType, ModelTrait,
    QueryFilter, QueryOrder, QuerySelect, RelationTrait, SqlErr,
};

use crate::db_controller::DBController;

impl DBController {
    pub async fn crud_create_executable_entitlement(
        &self,
        executable_operating_system_version_id: i64,
        entitlement_id: i64,
    ) -> Result<(), DbErr> {
        let executable_entitlement = entity::executable_entitlement::ActiveModel {
            executable_operating_system_version_id: ActiveValue::Set(
                executable_operating_system_version_id,
            ),
            entitlement_id: ActiveValue::Set(entitlement_id),
        };

        match executable_entitlement.insert(self.get_connection()).await {
            Ok(_) => Ok(()),
            Err(db_err) => {
                if let Some(SqlErr::UniqueConstraintViolation(_)) = db_err.sql_err() {
                    log::debug!(
                        "executable / entitlement association already exists {} / {}",
                        executable_operating_system_version_id,
                        entitlement_id
                    );
                    Ok(())
                } else {
                    Err(db_err)
                }
            }
        }
    }

    pub async fn crud_get_all_executables_entitlements(
        &self,
        operating_system_version_id: i64,
    ) -> Result<BTreeMap<String, ExecutableInfos>> {
        let executables_operating_system_version =
            entity::prelude::ExecutableOperatingSystemVersion::find()
                .filter(
                    entity::executable_operating_system_version::Column::OperatingSystemVersionId
                        .eq(operating_system_version_id),
                )
                .join(
                    JoinType::LeftJoin,
                    entity::executable_operating_system_version::Relation::Executable.def(),
                )
                .all(self.get_connection())
                .await?;

        let mut result = BTreeMap::new();
        for executable_os in executables_operating_system_version {
            let executable = executable_os
                .find_related(entity::prelude::Executable)
                .one(self.get_connection())
                .await?
                .ok_or(anyhow!("did not find matching executable"))?;

            // Get related entitlements
            let entitlements = entity::prelude::Entitlement::find()
                .join(
                    JoinType::LeftJoin,
                    entity::entitlement::Relation::ExecutableEntitlement.def(),
                )
                .filter(
                    entity::executable_entitlement::Column::ExecutableOperatingSystemVersionId
                        .eq(executable_os.id),
                )
                .order_by_asc(entity::entitlement::Column::Key)
                .order_by_asc(entity::entitlement::Column::Value)
                .all(self.get_connection())
                .await?
                .into_iter()
                .map(Entitlement::from)
                .collect::<Vec<Entitlement>>();

            result.insert(
                executable.full_path,
                ExecutableInfos {
                    name: executable.name,
                    entitlements,
                },
            );
        }

        Ok(result)
    }
}
