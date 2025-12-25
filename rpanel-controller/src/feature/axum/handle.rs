use axum::Json;
use sea_orm::{EntityTrait};
use migration::entity::t_agent_system_status;
use crate::feature::database::get_database;

pub async fn get_agent_list() -> Json<Vec<t_agent_system_status::Model>> {
    let db = get_database().await;
    let list = t_agent_system_status::Entity::find().all(&db).await.unwrap();
    Json(list)
}