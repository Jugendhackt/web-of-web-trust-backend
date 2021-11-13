use crate::core::config::CONFIG;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    Pool, Postgres,
};

pub type DbPool = Pool<Postgres>;

pub async fn init_pool() -> DbPool {
    // apply database migration
    match PgPoolOptions::new()
        .max_connections(5)
        .connect_with(
            PgConnectOptions::new()
                .host(&CONFIG.database.host)
                .port(CONFIG.database.port)
                .username(&CONFIG.database.user)
                .password(&CONFIG.database.password),
        )
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to init database pool at backend boot: {}", e);
            std::process::exit(1)
        }
    }
}

// Trim \0 from Strings. This is required for db inserts since \0 is
// used as string terminator in PostgreSQL
pub fn trim_zero(s: &str) -> String {
    s.trim_matches('\0').to_owned()
}
