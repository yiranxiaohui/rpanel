use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Agent::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Agent::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Agent::Uuid).string().not_null().unique_key())
                    .col(ColumnDef::new(Agent::Name).string().not_null())
                    .col(ColumnDef::new(Agent::SecretKey).string().null())
                    .col(ColumnDef::new(Agent::HostName).string().null())
                    .col(ColumnDef::new(Agent::IpAddress).string().null())
                    .col(ColumnDef::new(Agent::OsInfo).string().null())
                    .col(ColumnDef::new(Agent::Version).string().null())
                    // 0-offline, 1-online, 2-disabled
                    .col(ColumnDef::new(Agent::Status).integer().not_null().default(0))
                    // JSON string for tags
                    .col(ColumnDef::new(Agent::Tags).string().null())
                    .col(
                        ColumnDef::new(Agent::CreateTime)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Agent::HeartbeatTime).date_time().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Agent::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Agent {
    #[sea_orm(iden = "t_agent")]
    Table,
    Id,
    Uuid,
    Name,
    SecretKey,
    HostName,
    IpAddress,
    OsInfo,
    Version,
    Status,
    Tags,
    CreateTime,
    HeartbeatTime,
}
