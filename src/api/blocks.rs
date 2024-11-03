use rocket::get;
use rocket::serde::json::Json;
use crate::services::blockchain::fetch_recent_blocks;
use crate::models::blocks::Block;

#[get("/blocks")]
pub async fn get_blocks() -> Json<Vec<Block>> {
    let blocks = fetch_recent_blocks().await;
    Json(blocks)
}
