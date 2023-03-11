///
///  you must know the Bson to use this code
/// 

use futures::TryStreamExt;
use mongodb::{
    Client, options::{ClientOptions, FindOptions},
    error::Error, Database, Collection, bson::{Document}
};
use serde::{Serialize, de::DeserializeOwned};
use crate::settings;

#[derive(Debug, Clone)]
pub struct MongoDB {
    client: Client,
    db: Database
}

impl MongoDB {
    pub async fn init(cfg: settings::MongoCfg) -> Result<Self, Error>{
        let url = format!("mongodb://{}:{}", cfg.host, cfg.port);
        let mut client_options = ClientOptions::parse(url).await?;
        client_options.app_name = Some("Authentication".to_string());
        let client = Client::with_options(client_options)?;
        let db = client.database(&cfg.database);
        Ok(Self {client, db})        
    }

    /// if you want to change database just nee to run this once
    pub fn set_db(&mut self, name: &str) -> &mut Self {
        let db = self.client.database(name);
        self.db = db;        
        self
    }

    fn get_collection<T>(&self, name: &str) -> Collection<T> {        
        self.db.collection::<T>(name)        
    }

    pub async fn find<T>(
        &self, collection_name: &str,
        filter: Option<Document>,
        find_option: Option<Document>,
        result: &mut Vec<T>
    )  -> Result<(), Error>       
    where T: DeserializeOwned + Unpin + Send + Sync {
        let c = &self.get_collection::<T>(collection_name);
        let options = build_options(find_option);
        let mut cursor = c.find(filter, options).await?;

        while let Some(item) = cursor.try_next().await? {
            result.push(item);
        }
    
        Ok(())
    }

    pub async fn find_one<T>(&self, collection_name: &str, filter: Document) -> Result<Option<T>, Error>
    where T: DeserializeOwned + Unpin + Send + Sync {
        let c = &self.get_collection::<T>(collection_name);        
        c.find_one(filter, None).await
    }

    pub async fn insert_many<T:Serialize>(&self, collection_name: &str, docs: Vec<T>) 
    -> Result<mongodb::results::InsertManyResult, Error> {
        let c = &self.get_collection::<T>(collection_name);
        c.insert_many(docs, None).await
    }
    
    pub async fn insert_one<T:Serialize>(&self, collection_name: &str, doc: T) 
    -> Result<mongodb::results::InsertOneResult, Error> {
        let c = &self.get_collection::<T>(collection_name);
        c.insert_one(doc, None).await
    }
    
    pub async fn update_many<T:Serialize>(&self, collection_name: &str, query: Document, update: Document) 
    -> Result<mongodb::results::UpdateResult, Error> {
        let c = &self.get_collection::<T>(collection_name);
        c.update_many(query, update, None).await
    }

    pub async fn update_one<T:Serialize>(&self, collection_name: &str, query: Document, update: Document) 
    -> Result<mongodb::results::UpdateResult, Error> {
        let c = &self.get_collection::<T>(collection_name);
        c.update_one(query, update, None).await
    }

    pub async fn delete_many<T:Serialize>(&self, collection_name: &str, query: Document) 
    -> Result<mongodb::results::DeleteResult, Error> {
        let c = &self.get_collection::<T>(collection_name);
        c.delete_many(query, None).await
    }

    pub async fn delete_one<T:Serialize>(&self, collection_name: &str, query: Document) 
    -> Result<mongodb::results::DeleteResult, Error> {
        let c = &self.get_collection::<T>(collection_name);
        c.delete_one(query, None).await
    }

    pub async fn replace_one<T:Serialize>(&self, collection_name: &str, query: Document, replacement: T) 
    -> Result<mongodb::results::UpdateResult, Error> {
        let c = &self.get_collection::<T>(collection_name);
        c.replace_one(query, replacement, None).await
    }

    pub async fn drop<T:Serialize>(&self, collection_name: &str) 
    -> Result<(), Error> {
        let c = &self.get_collection::<T>(collection_name);
        c.drop(None).await
    }

    pub async fn count_documents<T:Serialize>(&self, collection_name: &str, filter: Document) 
    -> Result<u64, Error> {
        let c = &self.get_collection::<T>(collection_name);
        c.count_documents(filter, None).await
    }
    
    



    

}

fn build_options(find_option: Option<Document>) -> FindOptions {
    FindOptions::builder().sort(find_option).build()
}



