use diesel::{r2d2::ConnectionManager, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::{info, warn};
use r2d2::{Pool, PooledConnection};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn establish_connection(db_url: String) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    match Pool::builder().build(manager) {
            Ok(pool) => {
                info!("🆗 Database connection established.");
                pool
            }
            Err(err) => {
                warn!("🆖 Could not build connection pool: {:?}", err);
                std::process::exit(1);
            }
        }
    }


pub fn run_pending_migrations(mut connection: PooledConnection<ConnectionManager<PgConnection>>) {
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("🆖 Failed to run migrations");
}