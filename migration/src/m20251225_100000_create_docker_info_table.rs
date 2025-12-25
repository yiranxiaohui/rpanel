use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AgentDockerInfo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AgentDockerInfo::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AgentDockerInfo::AgentId).string().not_null())
                    // 1: Containers, 2: Images
                    .col(ColumnDef::new(AgentDockerInfo::DataType).integer().not_null())
                    .col(ColumnDef::new(AgentDockerInfo::Content).text().not_null())
                    .col(
                        ColumnDef::new(AgentDockerInfo::UpdateTime)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_agent_docker_type")
                    .table(AgentDockerInfo::Table)
                    .col(AgentDockerInfo::AgentId)
                    .col(AgentDockerInfo::DataType)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AgentDockerInfo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AgentDockerInfo {
    #[sea_orm(iden = "t_agent_docker_info")]
    Table,
    Id,
    AgentId,
    DataType,
    Content,
    UpdateTime,
}
