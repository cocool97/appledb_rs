use serde::Serialize;
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
