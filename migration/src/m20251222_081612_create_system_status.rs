use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TAgentSystemStatus::Table)
                    .if_not_exists()
                    .col(pk_auto(TAgentSystemStatus::Id))
                    .col(integer(TAgentSystemStatus::AgentId))
                    .col(float(TAgentSystemStatus::CpuUsage))
                    .col(big_integer(TAgentSystemStatus::MemUsed))
                    .col(big_integer(TAgentSystemStatus::MemTotal))
                    .col(big_integer(TAgentSystemStatus::DiskUsed))
                    .col(big_integer(TAgentSystemStatus::DiskTotal))
                    .col(date_time(TAgentSystemStatus::CreateTime))
                    .col(date_time(TAgentSystemStatus::UpdateTime))
                    .index(
                        Index::create()
                            .name("uniq_agent_status_agent_id")
                            .table(TAgentSystemStatus::Table)
                            .col(TAgentSystemStatus::AgentId)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TAgentSystemStatus::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TAgentSystemStatus {
    #[sea_orm(iden = "t_agent_system_status")]
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
