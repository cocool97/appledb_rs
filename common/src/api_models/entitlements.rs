use serde::Serialize;
use utoipa::ToSchema;

use crate::db_models::Entitlement;

#[derive(Serialize, ToSchema)]
pub struct EntitlementsDiff {
    pub added: Vec<Entitlement>,
    pub removed: Vec<Entitlement>,
    pub unchanged: Vec<Entitlement>,
}
