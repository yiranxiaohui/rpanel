pub use sea_orm_migration::prelude::*;

pub mod m20251222_081612_create_system_status;
mod m20251224_091017_create_agent_table;
pub mod entity;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251222_081612_create_system_status::Migration),
            Box::new(m20251224_091017_create_agent_table::Migration),
        ]
    }
}
