use cache::PostgresCache;
use database::Database;
use http_gateway::http;
use http_gateway::{Config, Error};
use sqlx::postgres::PgPoolOptions;
use tokio::runtime::Builder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = Builder::new_multi_thread()
        .worker_threads(2)  // Limits threads to 2 (change if needed)
        .enable_all()
        .build()
        .expect("Failed to create Tokio runtime");

    runtime.block_on(async {
        run().await
    })
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_envvar();

    let db_opts = PgPoolOptions::new()
        .min_connections(1)
        .max_connections(config.database_threads);

    let db = Database::connect(&*config.database_uri, db_opts)
        .await
        .map_err(|e| {
            eprintln!("Database connection failed: {:?}", e);
            Error::DatabaseError(e)
        })?;

    let cache_opts = cache::Options {
        users: true,
        guilds: false,
        members: true,
        channels: false,
        threads: false,
        roles: false,
        emojis: false,
        voice_states: false,
    };

    let cache = PostgresCache::connect(config.cache_uri.clone(), cache_opts, config.cache_threads)
        .await
        .map_err(|e| {
            eprintln!("Cache connection failed: {:?}", e);
            Error::CacheError(e)
        })?;

    let server = http::Server::new(config, db, cache);
    server.start().await?;

    Ok(())
}
