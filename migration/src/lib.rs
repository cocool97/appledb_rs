pub use sea_orm_migration::prelude::*;

mod m1_operating_system;
mod m2_device;
mod m3_operating_system_version;
mod m4_executable;
mod m5_entitlement;
mod m6_executable_operating_system_version;
mod m7_executable_entitlement;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m1_operating_system::Migration),
            Box::new(m2_device::Migration),
            Box::new(m3_operating_system_version::Migration),
            Box::new(m4_executable::Migration),
            Box::new(m5_entitlement::Migration),
            Box::new(m6_executable_operating_system_version::Migration),
            Box::new(m7_executable_entitlement::Migration),
        ]
    }
}
