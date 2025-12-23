use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(TSystemStatus::Table)
                    .if_not_exists()
                    .col(pk_auto(TSystemStatus::Id))
                    .col(string(TSystemStatus::AgentId))
                    .col(float(TSystemStatus::CpuUsage))
                    .col(big_integer(TSystemStatus::MemUsed))
                    .col(big_integer(TSystemStatus::MemTotal))
                    .col(big_integer(TSystemStatus::DiskUsed))
                    .col(big_integer(TSystemStatus::DiskTotal))
                    .col(date_time(TSystemStatus::CreateTime))
                    .col(date_time(TSystemStatus::UpdateTime))
                    .index(
                        Index::create()
                            .name("uniq_agent_id")
                            .table(TSystemStatus::Table)
                            .col(TSystemStatus::AgentId)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(TSystemStatus::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TSystemStatus {
    Table,
    Id,
    AgentId,
    CpuUsage,
    MemUsed,
    MemTotal,
    DiskUsed,
    DiskTotal,
    CreateTime,
    UpdateTime,
}
