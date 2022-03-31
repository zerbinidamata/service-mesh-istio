use crate::{error::Error::*, handler::RatingRequest, Rating, Result};
use chrono::prelude::*;
use futures::StreamExt;
use mongodb::bson::{doc, document::Document, oid::ObjectId, Bson};
use mongodb::{options::ClientOptions, Client, Collection};

const DB_NAME: &str = "rating_db";
const COLL: &str = "ratings";

const ID: &str = "_id";
const RATING: &str = "rating";
const AUTHOR: &str = "author_id";
const PRODUCT: &str = "product_id";
const ADDED_AT: &str = "added_at";

#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
}

impl DB {
  pub async fn init() -> Result<Self> {
      let mut client_options = ClientOptions::parse("mongodb://127.0.0.1:27017").await?;
      client_options.app_name = Some("rating_db".to_string());
      Ok(Self {
          client: Client::with_options(client_options)?,
      })
  }

  pub async fn fetch_ratings(&self, product_id: &str) -> Result<Vec<Rating>> {
    let filter = doc! { "product_id": product_id  };
    // let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
  
    let mut cursor = self
        .get_collection()
        .find(filter, None)
        .await
        .map_err(MongoQueryError)?;
  
    let mut result: Vec<Rating> = Vec::new();
    while let Some(doc) = cursor.next().await {
        result.push(self.doc_to_rating(&doc?)?);
    }
    Ok(result)
  }
  
  pub async fn edit_rating(&self, id: &str, entry: &RatingRequest) -> Result<()> {
    let oid = ObjectId::with_string(id).map_err(|_| InvalidIDError(id.to_owned()))?;
    let query = doc! {
        "_id": oid,
    };
    let doc = doc! {
      PRODUCT: entry.product_id.clone(),
      AUTHOR: entry.author_id.clone(),
      RATING: entry.rating.clone(),
      ADDED_AT: Utc::now(),
    };

    self.get_collection()
        .update_one(query, doc, None)
        .await
        .map_err(MongoQueryError)?;
    Ok(())
  }
  
  pub async fn create_rating(&self, entry: RatingRequest) -> Result<()> {
    let doc = doc! {
      PRODUCT: entry.product_id.clone(),
      AUTHOR: entry.author_id.clone(),
      RATING: entry.rating.clone(),
      ADDED_AT: Utc::now(),
  };

  self.get_collection()
      .insert_one(doc, None)
      .await
      .map_err(MongoQueryError)?;
  Ok(())
  }
  
  pub async fn delete_rating(&self, id: &str) -> Result<()> {
    let oid = ObjectId::with_string(id).map_err(|_| InvalidIDError(id.to_owned()))?;
    let filter = doc! {
        "_id": oid,
    };

    self.get_collection()
        .delete_one(filter, None)
        .await
        .map_err(MongoQueryError)?;
    Ok(())
}  
  
  
  fn get_collection(&self) -> Collection {
    self.client.database(DB_NAME).collection(COLL)
  }
  
  fn doc_to_rating(&self, doc: &Document) -> Result<Rating> {
    let id = doc.get_object_id(ID)?;
    let rating = doc.get_str(RATING)?;
    let author = doc.get_str(AUTHOR)?;
    let product = doc.get_str(PRODUCT)?;
    let added_at = doc.get_datetime(ADDED_AT)?;
  
    let rating = Rating {
        id: id.to_hex(),
        author_id: author.to_owned(),
        rating: rating.to_owned(),
        product_id: product.to_owned(),
        added_at: *added_at,
    };
    Ok(rating)
  }
}

