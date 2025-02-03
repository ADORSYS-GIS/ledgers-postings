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
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env for tests");
    let conn = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));
    // Optionally run migrations here
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