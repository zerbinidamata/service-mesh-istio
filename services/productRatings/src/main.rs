use chrono::prelude::*;
use db::DB;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use warp::{Filter, Rejection};

type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;

mod db;
mod handler;

#[derive(Serialize, Deserialize, Debug)]
pub struct Rating {
    pub user_id: Integeer,
    pub rating: Integeer
}


#[tokio::main]
async fn main() -> Result<()> {
    let db = DB::init().await?;

    let ratings = warp::path("ratings");

    let ratings_routes = ratings
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handler::create_ratings_handler)
        .or(ratings
            .and(warp::put())
            .and(warp::path::param())
            .and(warp::body::json())
            .and(with_db(db.clone()))
            .and_then(handler::edit_ratings_handler))
        .or(ratings
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(handler::delete_ratings_handler))
        .or(ratings
            .and(warp::get())
            .and(with_db(db.clone()))
            .and_then(handler::ratingss_list_handler));


    println!("Started on port 8080");
    warp::serve(ratings_routes).run(([0, 0, 0, 0], 8080)).await;
    Ok(())
}