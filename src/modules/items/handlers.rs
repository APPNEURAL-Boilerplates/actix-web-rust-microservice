use actix_web::{web, HttpResponse};
use serde_json::json;
use uuid::Uuid;

use super::dto::CreateItemRequest;
use crate::{
    common::{error::AppError, response},
    state::AppState,
};

pub async fn list_items(state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let items = state.item_service.list();
    let count = items.len();
    Ok(response::ok(json!({ "items": items, "count": count })))
}

pub async fn create_item(
    state: web::Data<AppState>,
    payload: web::Json<CreateItemRequest>,
) -> Result<HttpResponse, AppError> {
    let item = state.item_service.create(payload.into_inner())?;
    state
        .event_publisher
        .publish(
            "item.created",
            json!({
                "id": item.id.to_string(),
                "name": item.name.clone(),
                "created_at": item.created_at.to_rfc3339(),
            }),
        )
        .await?;
    Ok(response::created(item))
}

pub async fn get_item(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let item_id = Uuid::parse_str(&path.into_inner())
        .map_err(|_| AppError::bad_request("Item id must be a valid UUID"))?;
    let item = state.item_service.get(item_id)?;
    Ok(response::ok(item))
}
