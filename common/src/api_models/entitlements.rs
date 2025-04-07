use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::db_models::Entitlement;

#[derive(Serialize, ToSchema)]
pub struct Diff<T: Serialize> {
    pub added: Vec<T>,
    pub removed: Vec<T>,
    pub unchanged: Vec<T>,
}

#[derive(Serialize, ToSchema)]
pub struct ExecutableInfos {
    pub name: String,
    pub entitlements: Vec<Entitlement>,
}

#[derive(Deserialize, Serialize, ToSchema, Default, Debug)]
pub struct EntitlementsInsertionStatus {
    pub inserted_executables: u32,
    pub existing_executables: u32,
    pub inserted_entitlements: u32,
    pub existing_entitlements: u32,
}
