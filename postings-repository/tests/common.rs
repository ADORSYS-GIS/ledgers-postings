// tests/common.rs

use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;
use dotenv::dotenv;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use diesel::RunQueryDsl;
use std::fs;
use std::path::Path;
use diesel::connection::SimpleConnection;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

/// Establish a database connection and run pending migrations.
/// (Assuming you already have this function for seeding.)
pub fn establish_connection() -> PgConnection {
    use std::env;
    use dotenv::dotenv;
    ensure_docker_database_running();
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env for tests");
    let mut conn = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));
    // Optionally run migrations here
    conn.run_pending_migrations(MIGRATIONS)
    .expect("Failed to run migrations");    
    conn
}

/// Seed the database with fixture data from the specified SQL file.
///
/// # Arguments
///
/// * `conn` - A mutable reference to a PostgreSQL connection.
/// * `fixture_file` - The path to the SQL file to execute.
pub fn seed_database(conn: &mut PgConnection, fixture_file: &str) {
    let fixture_path = Path::new(fixture_file);
    let sql = fs::read_to_string(fixture_path)
        .expect(&format!("Failed to read fixture file: {}", fixture_file));
    conn.batch_execute(&sql)
        .expect("Failed to seed the database");
}

/// Clean up the database by executing the cleanup SQL file.
pub fn cleanup_database(conn: &mut PgConnection) {
    let cleanup_path = Path::new("tests/fixtures/cleanup.sql");
    let sql = fs::read_to_string(cleanup_path)
        .expect("Failed to read cleanup file");
    conn.batch_execute(&sql)
        .expect("Failed to clean up the database");
}

pub struct TestDatabaseGuard;

impl TestDatabaseGuard {
    pub fn new() -> Self {
        Self {}
    }
}

impl Drop for TestDatabaseGuard {
    fn drop(&mut self) {
        // Open a new connection for cleanup.
        let mut conn = establish_connection();
        cleanup_database(&mut conn);
    }
}

use std::process::Command;
use diesel::result::ConnectionError;

/// Try to establish a connection; if it fails, attempt to start the Docker container.
pub fn ensure_docker_database_running() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env for tests");

    // Try to connect once
    match PgConnection::establish(&database_url) {
        Ok(_) => {
            // Connection successful, nothing to do.
        },
        Err(ConnectionError::BadConnection(_)) => {
            // Could not connect. Attempt to start Docker Compose.
            println!("Database not reachable; attempting to start Docker Compose...");
            let output = Command::new("docker")
                .args(&["compose", "-f", "compose-postgres.yml", "up", "-d"])
                .output()
                .expect("Failed to execute docker compose command");

            if !output.status.success() {
                panic!(
                    "Docker compose failed to start the database: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }

            // Optionally, wait a few seconds here for the DB to be ready.
            std::thread::sleep(std::time::Duration::from_secs(5));
        },
        Err(err) => {
            panic!("Unexpected error connecting to database: {:?}", err);
        }
    }
}
