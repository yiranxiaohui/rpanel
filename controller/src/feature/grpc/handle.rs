use migration::entity::t_agent;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use crate::feature::database::get_database;

// use chrono::{Utc};
// use migration::entity::{t_agent, t_agent_system_status, t_agent_docker_info};
// use rpanel_common::status::Status;
// use rpanel_common::agent::AgentRegisterRequest;
// use rpanel_grpc::docker::grpc::{DockerRequest};
// use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
// use migration::OnConflict;
// use crate::feature::database::get_database;
// 
// pub async fn handle_upload_status_message(docker_request: DockerRequest) {
//     if let Ok(status) = serde_json::from_str::<Status>(docker_request.payload.as_str()) {
//         let system = status.system;
//         let system_status = t_agent_system_status::ActiveModel {
//             id: Default::default(),
//             agent_id: Set(docker_request.agent_id),
//             cpu_usage: Set(system.cpu_usage),
//             mem_used: Set(system.mem_used as i64),
//             mem_total: Set(system.mem_total as i64),
//             disk_used: Set(system.disk_used as i64),
//             disk_total: Set(system.disk_total as i64),
//             create_time: Set(Utc::now().naive_utc()),
//             update_time: Set(Utc::now().naive_utc()),
//         };
//         let db = get_database().await;
//         t_agent_system_status::Entity::insert(system_status)
//             .on_conflict(
//                 OnConflict::column(t_agent_system_status::Column::AgentId).update_columns([
//                     t_agent_system_status::Column::CpuUsage,
//                     t_agent_system_status::Column::MemUsed,
//                     t_agent_system_status::Column::MemTotal,
//                     t_agent_system_status::Column::DiskUsed,
//                     t_agent_system_status::Column::DiskTotal,
//                     t_agent_system_status::Column::UpdateTime
//                 ]).to_owned(),
//             ).exec(&db)
//             .await
//             .expect("Entity should be executed");
//     }
// }
// 
// pub async fn handle_register_agent(docker_request: DockerRequest) {
//     if let Ok(req) = serde_json::from_str::<AgentRegisterRequest>(docker_request.payload.as_str()) {
//         let db = get_database().await;
//         
//         // 检查 Agent 是否存在
//         let existing_agent = t_agent::Entity::find()
//             .filter(t_agent::Column::Uuid.eq(&docker_request.agent_id))
//             .one(&db)
//             .await
//             .unwrap_or(None);
// 
//         match existing_agent {
//             Some(model) => {
//                 // 更新现有 Agent 信息
//                 let mut active_model: t_agent::ActiveModel = model.into();
//                 active_model.host_name = Set(req.host_name);
//                 active_model.ip_address = Set(req.ip_address);
//                 active_model.os_info = Set(req.os_info);
//                 active_model.version = Set(req.version);
//                 active_model.update(&db).await.ok();
//             }
//             None => {
//                 // 创建新 Agent
//                 let new_agent = t_agent::ActiveModel {
//                     uuid: Set(docker_request.agent_id),
//                     name: Set(req.name),
//                     host_name: Set(req.host_name),
//                     ip_address: Set(req.ip_address),
//                     os_info: Set(req.os_info),
//                     version: Set(req.version),
//                     create_time: Set(Utc::now().naive_utc()),
//                     status: Set(1), // 默认为在线
//                     ..Default::default()
//                 };
//                 new_agent.insert(&db).await.ok();
//             }
//         }
//     }
// }
// 
// pub async fn handle_docker_info_message(agent_id: String, type_: i32, payload: String) {
//     let db = get_database().await;
//     
//     let existing = t_agent_docker_info::Entity::find()
//         .filter(t_agent_docker_info::Column::AgentId.eq(&agent_id))
//         .filter(t_agent_docker_info::Column::DataType.eq(type_))
//         .one(&db)
//         .await
//         .unwrap_or(None);
//         
//     match existing {
//         Some(model) => {
//              let mut active: t_agent_docker_info::ActiveModel = model.into();
//              active.content = Set(payload);
//              active.update_time = Set(Utc::now().naive_utc());
//              active.update(&db).await.ok();
//         },
//         None => {
//              let info = t_agent_docker_info::ActiveModel {
//                 id: Default::default(),
//                 agent_id: Set(agent_id),
//                 data_type: Set(type_),
//                 content: Set(payload),
//                 update_time: Set(Utc::now().naive_utc()),
//             };
//              info.insert(&db).await.ok();
//         }
//     }
// }
// 
// 
pub async fn set_agent_online(agent_id: String) {
    let db = get_database().await;
    t_agent::Entity::update_many()
        .col_expr(t_agent::Column::Status, sea_orm::sea_query::Expr::value(1))
        .filter(t_agent::Column::Uuid.eq(agent_id))
        .exec(&db)
        .await
        .ok();
}

pub async fn set_agent_offline(agent_id: String) {
    let db = get_database().await;
    t_agent::Entity::update_many()
        .col_expr(t_agent::Column::Status, sea_orm::sea_query::Expr::value(0))
        .filter(t_agent::Column::Uuid.eq(agent_id))
        .exec(&db)
        .await
        .ok();
}