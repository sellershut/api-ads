pub mod driver;
pub mod errors;

use bb8::Pool;
use bb8_redis::RedisMultiplexedConnectionManager;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};
use tracing::{error, info, trace};

use self::errors::DatabaseError;

pub struct DatabaseConnection {
    surreal: Surreal<Client>,
    redis: Pool<RedisMultiplexedConnectionManager>,
}

impl DatabaseConnection {
    #[tracing::instrument(name = "db.conn")]
    pub async fn from_env() -> Result<Self, DatabaseError> {
        trace!("establishing database connection from env");
        let (surreal, redis) =
            futures_util::try_join!(Self::create_db_conn(), Self::create_redis_pool())?;
        Ok(Self { surreal, redis })
    }

    async fn create_db_conn() -> Result<Surreal<Client>, DatabaseError> {
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
        Ok(db)
    }

    async fn create_redis_pool() -> Result<Pool<RedisMultiplexedConnectionManager>, DatabaseError> {
        let redis_host = Self::read_env("REDIS_HOST");
        let redis_port = Self::read_env("REDIS_PORT");
        let redis_db = api_utils::unwrap_env_variable("REDIS_DB")
            .map(|f| format!("/{f}"))
            .unwrap_or_default();
        let redis_pass = api_utils::unwrap_env_variable("REDIS_AUTH")
            .map(|f| format!("{f}@"))
            .unwrap_or_default();
        let redis_con = format!("redis://{redis_pass}{redis_host}:{redis_port}{redis_db}");

        tracing::trace!(url = redis_host, "connecting to redis");
        let manager = bb8_redis::RedisMultiplexedConnectionManager::new(redis_con)?;
        let pool = bb8::Pool::builder().max_size(15).build(manager).await?;
        tracing::info!(url = redis_host, "redis connected");
        Ok(pool)
    }

    fn read_env(var: &str) -> String {
        trace!("[ENV] reading variable");
        let cb = || {
            error!(var = var, "variable is missing or empty");
        };
        api_utils::unwrap_env_variable(var)
            .or_else(|| {
                cb();
                None
            })
            .expect("variable is missing")
    }
}
