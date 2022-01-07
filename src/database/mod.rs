use crate::errors::AppError;
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client, Database,
};

#[derive(Debug)]
pub struct DB {
    pub mongo_db: Database,
}

impl DB {
    pub async fn new(uri: &str, database_name: &str) -> Result<Self, AppError> {
        let options =
            ClientOptions::parse_with_resolver_config(uri, ResolverConfig::cloudflare()).await?;

        let client = Client::with_options(options)?;
        let mongo_db = client.database(database_name);
        Ok(DB { mongo_db })
    }
}
