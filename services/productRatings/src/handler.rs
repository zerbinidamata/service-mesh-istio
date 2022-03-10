use crate::{db::DB, WebResult};
use serde::{Deserialize, Serialize};
use warp::{http::StatusCode, reject, reply::json, Reply};

#[derive(Serialize, Deserialize, Debug)]
pub struct RatingRequest {
    pub user_id: Integer,
    pub rating: Integer,
}

pub async fn ratings_list_handler(db: DB) -> WebResult<impl Reply> {
    let ratings = db.fetch_ratings().await.map_err(|e| reject::custom(e))?;
    Ok(json(&ratings))
}

pub async fn create_rating_handler(body: RatingRequest, db: DB) -> WebResult<impl Reply> {
    db.create_rating(&body).await.map_err(|e| reject::custom(e))?;
    Ok(StatusCode::CREATED)
}

pub async fn edit_rating_handler(id: String, body: RatingRequest, db: DB) -> WebResult<impl Reply> {
    db.edit_rating(&id, &body)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}

pub async fn delete_rating_handler(id: String, db: DB) -> WebResult<impl Reply> {
    db.delete_rating(&id).await.map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}