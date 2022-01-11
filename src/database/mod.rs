use crate::{
    errors::AppError,
    models::{book::Book, user::User},
};
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client, Collection,
};

#[derive(Debug)]
pub struct DB {
    pub mongo_client: Client,
    pub mongo_book_collection: Collection<Book>,
    pub mongo_user_collection: Collection<User>,
}

impl DB {
    pub async fn new(uri: &str, database_name: &str) -> Result<Self, AppError> {
        let options =
            ClientOptions::parse_with_resolver_config(uri, ResolverConfig::cloudflare()).await?;

        let client = Client::with_options(options)?;
        let db = client.database(database_name);

        let book_collection = db.collection::<Book>("books");
        let user_collection = db.collection::<User>("users");

        Ok(DB {
            mongo_client: client,
            mongo_book_collection: book_collection,
            mongo_user_collection: user_collection,
        })
    }
}
