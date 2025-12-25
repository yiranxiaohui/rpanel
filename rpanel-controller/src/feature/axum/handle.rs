use axum::Json;
use sea_orm::{ConnectionTrait, EntityTrait, FromQueryResult};
use sea_orm::prelude::DateTime;
use sea_orm::sea_query::{Expr, Query};
use serde::Serialize;
use migration::entity::{t_agent, t_agent_system_status};
use crate::feature::database::get_database;

#[derive(Debug, Serialize, FromQueryResult)]
pub struct AgentDetail {
    pub id: i32,
    pub uuid: String,
    pub name: String,
    pub host_name: Option<String>,
    pub ip_address: Option<String>,
    pub os_info: Option<String>,
    pub version: Option<String>,
    pub status: i32,
    
    // System status fields (can be null if no status reported yet)
    pub cpu_usage: Option<f32>,
    pub mem_used: Option<i64>,
    pub mem_total: Option<i64>,
    pub disk_used: Option<i64>,
    pub disk_total: Option<i64>,
    pub last_update: Option<DateTime>,
}

pub async fn get_agent_list() -> Json<Vec<AgentDetail>> {
    let db = get_database().await;
    let backend = db.get_database_backend();

    let query = Query::select()
        .columns([
            (t_agent::Entity, t_agent::Column::Id),
            (t_agent::Entity, t_agent::Column::Uuid),
            (t_agent::Entity, t_agent::Column::Name),
            (t_agent::Entity, t_agent::Column::HostName),
            (t_agent::Entity, t_agent::Column::IpAddress),
            (t_agent::Entity, t_agent::Column::OsInfo),
            (t_agent::Entity, t_agent::Column::Version),
            (t_agent::Entity, t_agent::Column::Status),
        ])
        .columns([
            (t_agent_system_status::Entity, t_agent_system_status::Column::CpuUsage),
            (t_agent_system_status::Entity, t_agent_system_status::Column::MemUsed),
            (t_agent_system_status::Entity, t_agent_system_status::Column::MemTotal),
            (t_agent_system_status::Entity, t_agent_system_status::Column::DiskUsed),
            (t_agent_system_status::Entity, t_agent_system_status::Column::DiskTotal),
        ])
        .expr_as(
             sea_orm::sea_query::Expr::col((t_agent_system_status::Entity, t_agent_system_status::Column::UpdateTime)),
             sea_orm::sea_query::Alias::new("last_update")
        )
        .from(t_agent::Entity)
        .left_join(
            t_agent_system_status::Entity,
            Expr::col((t_agent::Entity, t_agent::Column::Uuid))
                .equals((t_agent_system_status::Entity, t_agent_system_status::Column::AgentId)),
        )
        .to_owned();

    let list = AgentDetail::find_by_statement(backend.build(&query))
        .all(&db)
        .await
        .unwrap_or_default();
        
    Json(list)
}