use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

use dotenvy::dotenv;
use lazy_static::lazy_static;
use mongodb::{bson::doc, options::ClientOptions, Client};
use std::env;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

lazy_static! {
    static ref CLIENT: Arc<Mutex<Client>> = {
        dotenv().ok();
        let mut client_options =
            ClientOptions::parse(&env::var("MONGODB_URI").expect("MONGODB_URI must be set"))
                .expect("Failed to parse MONGODB_URI");

        let client = Client::with_options(client_options).expect("Failed to create MongoDB client");

        Arc::new(Mutex::new(client))
    };
}

pub async fn find_one<T>(
    collection_name: &str,
    filter: Option<bson::Document>,
) -> Result<Option<T>, Box<dyn Error>>
where
    T: for<'de> serde::Deserialize<'de>,
{
    let client = CLIENT.clone();
    let db = client.lock().await.database("your_database_name");
    let collection = db.collection(collection_name);

    let result = match filter {
        Some(filter) => collection.find_one(filter, None).await?,
        None => collection.find_one(doc! {}, None).await?,
    };

    match result {
        Some(document) => {
            let item: T = bson::from_document(document)?;
            Ok(Some(item))
        }
        None => Ok(None),
    }
}

pub async fn find_many<T>(
    collection_name: &str,
    filter: Option<bson::Document>,
) -> Result<Vec<T>, Box<dyn Error>>
where
    T: for<'de> serde::Deserialize<'de>,
{
    let client = CLIENT.clone();
    let db = client.lock().await.database("your_database_name");
    let collection = db.collection(collection_name);

    let mut cursor = match filter {
        Some(filter) => collection.find(filter, None).await?,
        None => collection.find(doc! {}, None).await?,
    };

    let mut results = Vec::new();
    while let Some(result) = cursor.try_next().await? {
        let item: T = bson::from_document(result)?;
        results.push(item);
    }

    Ok(results)
}

pub async fn insert_one<T>(collection_name: &str, document: T) -> Result<(), Box<dyn Error>>
where
    T: serde::Serialize,
{
    let client = CLIENT.clone();
    let db = client.lock().await.database("your_database_name");
    let collection = db.collection(collection_name);

    let serialized_doc = bson::to_document(&document)?;
    collection.insert_one(serialized_doc, None).await?;

    Ok(())
}

pub async fn insert_many<T>(collection_name: &str, documents: Vec<T>) -> Result<(), Box<dyn Error>>
where
    T: serde::Serialize,
{
    let client = CLIENT.clone();
    let db = client.lock().await.database("your_database_name");
    let collection = db.collection(collection_name);

    let serialized_docs: Vec<bson::Document> = documents
        .into_iter()
        .map(|doc| bson::to_document(&doc).unwrap())
        .collect();

    collection.insert_many(serialized_docs, None).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bson::{bson, doc};

    #[tokio::test]
    async fn test_find_one() {
        let collection_name = "users";

        // Insert a document for testing
        let document = bson!({ "name": "John", "age": 30 });
        insert_one(collection_name, document.clone()).await.unwrap();

        // Find the inserted document
        let filter = Some(doc! { "name": "John" });
        let result: Option<bson::Document> = find_one(collection_name, filter).await.unwrap();

        assert_eq!(result, Some(document));
    }

    #[tokio::test]
    async fn test_find_many() {
        let collection_name = "users";

        // Insert multiple documents for testing
        let documents = vec![
            bson!({ "name": "John", "age": 30 }),
            bson!({ "name": "Alice", "age": 25 }),
            bson!({ "name": "Bob", "age": 35 }),
        ];
        insert_many(collection_name, documents.clone())
            .await
            .unwrap();

        // Find all documents
        let result: Vec<bson::Document> = find_many(collection_name, None).await.unwrap();

        assert_eq!(result, documents);
    }

    #[tokio::test]
    async fn test_insert_one() {
        let collection_name = "users";

        // Insert a document
        let document = bson!({ "name": "John", "age": 30 });
        insert_one(collection_name, document.clone()).await.unwrap();

        // Find the inserted document
        let filter = Some(doc! { "name": "John" });
        let result: Option<bson::Document> = find_one(collection_name, filter).await.unwrap();

        assert_eq!(result, Some(document));
    }

    #[tokio::test]
    async fn test_insert_many() {
        let collection_name = "users";

        // Insert multiple documents
        let documents = vec![
            bson!({ "name": "John", "age": 30 }),
            bson!({ "name": "Alice", "age": 25 }),
            bson!({ "name": "Bob", "age": 35 }),
        ];
        insert_many(collection_name, documents.clone())
            .await
            .unwrap();

        // Find all documents
        let result: Vec<bson::Document> = find_many(collection_name, None).await.unwrap();

        assert_eq!(result, documents);
    }
}
