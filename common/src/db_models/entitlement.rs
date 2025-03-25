use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, PartialEq, Eq, Hash, Clone, ToSchema, Debug)]
pub struct Entitlement {
    pub id: i32,
    pub key: String,
    pub value: String,
}

impl From<entity::entitlement::Model> for Entitlement {
    fn from(value: entity::entitlement::Model) -> Self {
        Self {
            id: value.id,
            key: value.key,
            value: value.value,
        }
    }
}
