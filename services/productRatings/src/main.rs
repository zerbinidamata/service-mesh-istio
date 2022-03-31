use chrono::prelude::*;
use db::DB;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use warp::{Filter, Rejection};

type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;

mod db;
mod handler;
mod error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Rating {
    id: String,
    pub author_id: String,
    pub rating: String,
    pub product_id: String,
    pub added_at: DateTime<Utc>,
}


#[tokio::main]
async fn main() -> Result<()> {
    let db = DB::init().await?;

    let ratings = warp::path("ratings");

    let ratings_routes = ratings
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handler::create_rating_handler)
        .or(ratings
            .and(warp::put())
            .and(warp::path::param())
            .and(warp::body::json())
            .and(with_db(db.clone()))
            .and_then(handler::edit_rating_handler))
        .or(ratings
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(handler::delete_rating_handler))
        .or(ratings
            .and(warp::get())
            .and(with_db(db.clone()))
            .and_then(handler::ratings_list_handler));
    
    let routes = ratings_routes.recover(error::handle_rejection);


    println!("Started on port 8080");
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
    Ok(())
}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}