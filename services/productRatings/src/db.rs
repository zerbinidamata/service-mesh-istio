use mongodb::bson::{doc, document::Document, oid::ObjectId, Bson};
use mongodb::{options::ClientOptions, Client, Collection};

const DB_NAME: &str = "ratings";
const COLL: &str = "ratings";

const ID: &str = "_id";
const NAME: &str = "name";
const AUTHOR: &str = "author";
const NUM_PAGES: &str = "num_pages";
const ADDED_AT: &str = "added_at";
const TAGS: &str = "tags";

#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
}

impl DB {
  pub async fn init() -> Result<Self> {
      let mut client_options = ClientOptions::parse("mongodb://127.0.0.1:27017").await?;
      client_options.app_name = Some("booky".to_string());
      Ok(Self {
          client: Client::with_options(client_options)?,
      })
  }
}

pub async fn fetch_ratings(&self) -> Result<Vec<Book>> {
  let mut cursor = self
      .get_collection()
      .find(None, None)
      .await
      .map_err(MongoQueryError)?;

  let mut result: Vec<Book> = Vec::new();
  while let Some(doc) = cursor.next().await {
      result.push(self.doc_to_book(&doc?)?);
  }
  Ok(result)
}

fn get_collection(&self) -> Collection {
  self.client.database(DB_NAME).collection(COLL)
}