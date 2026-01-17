// use axum::{Json, extract::Path};
// use axum::http::StatusCode;
// use axum::response::sse::{Event, KeepAlive, Sse};
// use futures::stream::Stream;
// use tokio_stream::StreamExt;
// use sea_orm::{ConnectionTrait, EntityTrait, FromQueryResult};
// use sea_orm::prelude::DateTime;
// use sea_orm::sea_query::{Expr, Query};
// use sea_orm::{ColumnTrait, QueryFilter};
// use serde::{Deserialize, Serialize};
// use migration::entity::{t_agent, t_agent_system_status, t_agent_docker_info};
// use common::docker::{
//     PullImageRequest, ContainerActionRequest, RemoveContainerRequest,
//     RemoveImageRequest, RunContainerRequest
// };
// use crate::feature::database::get_database;
// use crate::feature::event::get_event_bus;
//
// #[derive(Debug, Serialize, FromQueryResult)]
// pub struct AgentDetail {
//     pub id: i32,
//     pub uuid: String,
//     pub name: String,
//     pub host_name: Option<String>,
//     pub ip_address: Option<String>,
//     pub os_info: Option<String>,
//     pub version: Option<String>,
//     pub status: i32,
//
//     // System status fields (can be null if no status reported yet)
//     pub cpu_usage: Option<f32>,
//     pub mem_used: Option<i64>,
//     pub mem_total: Option<i64>,
//     pub disk_used: Option<i64>,
//     pub disk_total: Option<i64>,
//     pub last_update: Option<DateTime>,
// }
//
// pub async fn get_agent_list() -> Json<Vec<AgentDetail>> {
//     let db = get_database().await;
//     let backend = db.get_database_backend();
//
//     let query = Query::select()
//         .columns([
//             (t_agent::Entity, t_agent::Column::Id),
//             (t_agent::Entity, t_agent::Column::Uuid),
//             (t_agent::Entity, t_agent::Column::Name),
//             (t_agent::Entity, t_agent::Column::HostName),
//             (t_agent::Entity, t_agent::Column::IpAddress),
//             (t_agent::Entity, t_agent::Column::OsInfo),
//             (t_agent::Entity, t_agent::Column::Version),
//             (t_agent::Entity, t_agent::Column::Status),
//         ])
//         .columns([
//             (t_agent_system_status::Entity, t_agent_system_status::Column::CpuUsage),
//             (t_agent_system_status::Entity, t_agent_system_status::Column::MemUsed),
//             (t_agent_system_status::Entity, t_agent_system_status::Column::MemTotal),
//             (t_agent_system_status::Entity, t_agent_system_status::Column::DiskUsed),
//             (t_agent_system_status::Entity, t_agent_system_status::Column::DiskTotal),
//         ])
//         .expr_as(
//              sea_orm::sea_query::Expr::col((t_agent_system_status::Entity, t_agent_system_status::Column::UpdateTime)),
//              sea_orm::sea_query::Alias::new("last_update")
//         )
//         .from(t_agent::Entity)
//         .left_join(
//             t_agent_system_status::Entity,
//             Expr::col((t_agent::Entity, t_agent::Column::Uuid))
//                 .equals((t_agent_system_status::Entity, t_agent_system_status::Column::AgentId)),
//         )
//         .to_owned();
//
//     let list = AgentDetail::find_by_statement(backend.build(&query))
//         .all(&db)
//         .await
//         .unwrap_or_default();
//
//     Json(list)
// }
//
// #[derive(Deserialize)]
// pub struct PullImageBody {
//     pub agent_id: String,
//     pub image: String,
// }
//
// pub async fn trigger_pull_image(Json(payload): Json<PullImageBody>) -> StatusCode {
//     let req = PullImageRequest {
//         image: payload.image.clone(),
//     };
//
//     let reply = DockerReply {
//         action: Action::PullImage as i32,
//         payload: serde_json::to_string(&req).unwrap(),
//     };
//
//     if send_to_agent(&payload.agent_id, reply).await {
//         StatusCode::OK
//     } else {
//         StatusCode::NOT_FOUND
//     }
// }
//
// pub async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, axum::Error>>> {
//     let rx = get_event_bus().subscribe();
//     let stream = tokio_stream::wrappers::BroadcastStream::new(rx);
//
//     let stream = stream.map(|msg| {
//         match msg {
//             Ok(web_event) => {
//                 Ok(Event::default()
//                     .event(web_event.event_type)
//                     .data(web_event.payload))
//             }
//             Err(_e) => {
//                  Ok(Event::default().comment("keep-alive"))
//             }
//         }
//     });
//
//     Sse::new(stream).keep_alive(KeepAlive::default())
// }
//
// // Docker Handlers
//
// pub async fn get_docker_containers(Path(agent_id): Path<String>) -> Json<Option<String>> {
//     let db = get_database().await;
//     let info = t_agent_docker_info::Entity::find()
//         .filter(t_agent_docker_info::Column::AgentId.eq(&agent_id))
//         .filter(t_agent_docker_info::Column::DataType.eq(1))
//         .one(&db)
//         .await
//         .unwrap_or(None);
//
//     Json(info.map(|i| i.content))
// }
//
// pub async fn get_docker_images(Path(agent_id): Path<String>) -> Json<Option<String>> {
//     let db = get_database().await;
//     let info = t_agent_docker_info::Entity::find()
//         .filter(t_agent_docker_info::Column::AgentId.eq(&agent_id))
//         .filter(t_agent_docker_info::Column::DataType.eq(2))
//         .one(&db)
//         .await
//         .unwrap_or(None);
//
//     Json(info.map(|i| i.content))
// }
//
// pub async fn trigger_refresh_docker(Path(agent_id): Path<String>) -> StatusCode {
//     let reply_c = DockerReply {
//         action: Action::ListContainers as i32,
//         payload: "".to_string(),
//     };
//     let reply_i = DockerReply {
//         action: Action::ListImages as i32,
//         payload: "".to_string(),
//     };
//
//     // We try to send both
//     let sent_c = send_to_agent(&agent_id, reply_c).await;
//     let sent_i = send_to_agent(&agent_id, reply_i).await;
//
//     if sent_c || sent_i {
//         StatusCode::OK
//     } else {
//         StatusCode::NOT_FOUND
//     }
// }
//
// pub async fn trigger_start_container(Path(agent_id): Path<String>, Json(payload): Json<ContainerActionRequest>) -> StatusCode {
//     let reply = DockerReply {
//         action: Action::StartContainer as i32,
//         payload: serde_json::to_string(&payload).unwrap(),
//     };
//     if send_to_agent(&agent_id, reply).await { StatusCode::OK } else { StatusCode::NOT_FOUND }
// }
//
// pub async fn trigger_stop_container(Path(agent_id): Path<String>, Json(payload): Json<ContainerActionRequest>) -> StatusCode {
//     let reply = DockerReply {
//         action: Action::StopContainer as i32,
//         payload: serde_json::to_string(&payload).unwrap(),
//     };
//     if send_to_agent(&agent_id, reply).await { StatusCode::OK } else { StatusCode::NOT_FOUND }
// }
//
// pub async fn trigger_remove_container(Path(agent_id): Path<String>, Json(payload): Json<RemoveContainerRequest>) -> StatusCode {
//     let reply = DockerReply {
//         action: Action::RemoveContainer as i32,
//         payload: serde_json::to_string(&payload).unwrap(),
//     };
//     if send_to_agent(&agent_id, reply).await { StatusCode::OK } else { StatusCode::NOT_FOUND }
// }
//
// pub async fn trigger_remove_image(Path(agent_id): Path<String>, Json(payload): Json<RemoveImageRequest>) -> StatusCode {
//     let reply = DockerReply {
//         action: Action::RemoveImage as i32,
//         payload: serde_json::to_string(&payload).unwrap(),
//     };
//     if send_to_agent(&agent_id, reply).await { StatusCode::OK } else { StatusCode::NOT_FOUND }
// }
//
// pub async fn trigger_run_container(Path(agent_id): Path<String>, Json(payload): Json<RunContainerRequest>) -> StatusCode {
//     let reply = DockerReply {
//         action: Action::RunContainer as i32,
//         payload: serde_json::to_string(&payload).unwrap(),
//     };
//     if send_to_agent(&agent_id, reply).await { StatusCode::OK } else { StatusCode::NOT_FOUND }
// }
