pub mod driver;
pub mod errors;

use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};
use tracing::{error, info, trace};

use self::errors::DatabaseError;

pub struct DatabaseConnection {
    database: Surreal<Client>,
}

impl DatabaseConnection {
    #[tracing::instrument(name = "db.conn")]
    pub async fn from_env() -> Result<Self, DatabaseError> {
        trace!("establishing database connection from env");

        let address = Self::read_env("DATABASE_URL");
        let namespace = Self::read_env("DATABASE_NAMESPACE");
        let username = Self::read_env("DATABASE_USERNAME");
        let password = Self::read_env("DATABASE_PASSWORD");
        let database = Self::read_env("DATABASE_NAME");

        trace!("environment locked and loaded. connecting...");

        let db = Surreal::new::<Ws>(&address).await?;

        db.signin(Root {
            username: &username,
            password: &password,
        })
        .await?;

        db.use_ns(&namespace).use_db(&database).await?;
        info!(address = address, user = username, "database connected");

        Ok(Self { database: db })
    }

    fn read_env(var: &str) -> String {
        trace!("[ENV] reading variable");
        let cb = || {
            error!("variable is missing or empty");
        };
        api_utils::unwrap_env_variable(var)
            .or_else(|| {
                cb();
                None
            })
            .expect("variable is missing")
    }
}
