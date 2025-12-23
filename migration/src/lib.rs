pub use sea_orm_migration::prelude::*;

pub mod m20251222_081612_create_system_status;
pub mod entity;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251222_081612_create_system_status::Migration),
        ]
    }
}
