use chrono::{Utc};
use migration::entity::t_agent_system_status;
use rpanel_common::status::Status;
use rpanel_grpc::docker::grpc::DockerRequest;
use sea_orm::{EntityTrait, Set};
use migration::OnConflict;
use crate::feature::database::get_database;

pub async fn handle_upload_status_message(docker_request: DockerRequest) {
    if let Ok(status) = serde_json::from_str::<Status>(docker_request.payload.as_str()) {
        let system = status.system;
        let system_status = t_agent_system_status::ActiveModel {
            id: Default::default(),
            agent_id: Set(docker_request.agent_id),
            cpu_usage: Set(system.cpu_usage),
            mem_used: Set(system.mem_used as i64),
            mem_total: Set(system.mem_total as i64),
            disk_used: Set(system.disk_used as i64),
            disk_total: Set(system.disk_total as i64),
            create_time: Set(Utc::now().naive_utc()),
            update_time: Set(Utc::now().naive_utc()),
        };
        let db = get_database().await;
        t_agent_system_status::Entity::insert(system_status)
            .on_conflict(
                OnConflict::column(t_agent_system_status::Column::AgentId).update_columns([
                    t_agent_system_status::Column::CpuUsage,
                    t_agent_system_status::Column::MemUsed,
                    t_agent_system_status::Column::MemTotal,
                    t_agent_system_status::Column::DiskUsed,
                    t_agent_system_status::Column::DiskTotal,
                    t_agent_system_status::Column::UpdateTime
                ]).to_owned(),
            ).exec(&db)
            .await
            .expect("Entity should be executed");
    }
}