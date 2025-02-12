pub use sea_orm_migration::prelude::*;

mod m1_operating_system;
mod m2_operating_system_version;
mod m3_executable;
mod m4_entitlement;
mod m5_executable_entitlement;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m1_operating_system::Migration),
            Box::new(m2_operating_system_version::Migration),
            Box::new(m3_executable::Migration),
            Box::new(m4_entitlement::Migration),
            Box::new(m5_executable_entitlement::Migration),
        ]
    }
}
