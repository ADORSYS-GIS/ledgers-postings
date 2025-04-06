# Ledgers-Postings (Rust Version)

This project is a Rust implementation of a ledger solution based on double-entry bookkeeping principles, originally inspired by [adorsys/ledgers](https://github.com/adorsys/ledgers). It utilizes Diesel ORM with PostgreSQL for data persistence and features a clear separation of concerns following the Repository Pattern.

## Overview

*   **Rust Implementation:** A modern ledger system backend written in Rust.
*   **Layered Architecture:** Organized into a workspace with distinct crates for data (`postings-repository`), services (`postings-service`), and potentially a REST API (`postings-rest-server`). Currently, the core logic resides in `postings-repository`.
*   **Data Layer (`postings-repository`):**
    *   Uses Diesel ORM for PostgreSQL interaction.
    *   Implements the Repository Pattern: Data access logic is separated into dedicated repository modules (e.g., `posting_repository`) rather than being part of the data models.
    *   Defines clear data models (structs mapping to tables) and custom PostgreSQL ENUM types.
*   **Testing:** Comprehensive integration tests are provided for the repository layer, ensuring database interactions work as expected. Tests use `serial_test` for sequential execution and automated cleanup.

## Workspace Structure

This project is a Rust workspace containing the following crates:

*   **`postings-repository/`**: The core data persistence layer. Contains Diesel schema, migrations, data models, and repository query implementations. **This is the most developed part of the project.**
*   **`postings-service/`**: Intended for business logic and service implementations. (Currently contains placeholder code).
*   **`postings-rest-server/`**: Intended for exposing ledger functionalities via a REST API. (Currently contains placeholder code).

## Getting Started (`postings-repository`)

The primary functionality and tests are currently within the `postings-repository` crate.

### Prerequisites

*   Rust toolchain (Cargo)
*   Docker and Docker Compose (for running the test database)
*   `diesel_cli` (Install with `cargo install diesel_cli --no-default-features --features postgres`)

### Setup

1.  **Configure Database:** Ensure a `.env` file exists in the `postings-repository` directory with the `DATABASE_URL`. Example:
    ```dotenv
    DATABASE_URL=postgres://user:password@localhost:5432/mydb
    ```
    *Note: The test setup can automatically start a compatible PostgreSQL database using Docker Compose if one isn't running.*

2.  **Run Migrations (Optional):** The tests run migrations automatically. However, you can run them manually from the `postings-repository` directory:
    ```bash
    diesel migration run
    ```

### Running Tests

The easiest way to verify the setup and functionality is to run the integration tests.

From the **workspace root directory** (`ledgers-postings/`):

```bash
cargo test
```

This command will attempt to:

1.  Check if the expected PostgreSQL container (defined in `postings-repository/compose-postgres.yml`) is running. If not, it will try to start it using Docker Compose.
2.  Execute Diesel migrations to set up the schema in the test database.
3.  Run all tests within the `postings-repository` crate:
    *   Each test seeds the database with specific fixture data (`tests/fixtures/`).
    *   Repository functions are called and their results asserted.
    *   Database tables are automatically cleaned up after each test using a `TestDatabaseGuard` (`tests/common.rs`).
    *   Tests run sequentially (`#[serial]`) to avoid database conflicts.

#### Troubleshooting Test Failures

*   **Connection Errors:** If tests fail early with errors related to database connection, authentication (password), or finding the database (`mydb`), it might be because another PostgreSQL instance is running on your machine (perhaps on the default port 5432) and interfering with the one managed by Docker Compose for these tests.
    *   **Check running containers:** List your running Docker containers to see if another PostgreSQL container is active:
        ```bash
        docker ps
        ```
        Look for containers with `postgres` in the name or image column.
    *   **Stop the conflicting container:** If you find an unexpected PostgreSQL container running, stop it using its name or ID:
        ```bash
        # Replace <container_name_or_id> with the actual name or ID
        docker stop <container_name_or_id>
        ```
    *   **Remove the conflicting container (optional):** If you don't need the stopped container anymore, you can remove it:
        ```bash
        # Replace <container_name_or_id> with the actual name or ID
        docker rm <container_name_or_id>
        ```
    *   **Retry:** After stopping/removing the conflicting container, run `cargo test` again. The test setup should now be able to start and connect to the correct PostgreSQL instance defined in `compose-postgres.yml`.
*   **Migration Errors:** Ensure migrations have run correctly. You can try running `diesel migration run` manually from the `postings-repository` directory.
*   **Fixture Errors:** Check the SQL syntax in the relevant `tests/fixtures/*.sql` file.

### Starting Development Database Manually (Optional)

If you want to run the PostgreSQL database and Adminer (web UI) independently:

1.  Navigate to the `postings-repository` directory.
2.  Run:
    ```bash
    docker compose -f postings-repository/compose-postgres.yml up -d
    ```
3.  Access Adminer at `http://localhost:18080` (or the configured port). Use server `postgres`, username `user`, password `password` (or as configured in `compose-postgres.yml` and `.env`).

## `postings-repository` Crate Structure

*   **`src/`**: Contains the main Rust source code:
    *   **`models/`**: Defines Rust structs (`mod.rs`) representing database tables (e.g., `Posting`, `LedgerAccount`) and custom PostgreSQL ENUM types (`enums.rs`) mapped using `diesel-derive-enum`.
    *   **`repository/`**: Contains modules (e.g., `posting_repository`, `ledger_account_repository`) implementing the Repository Pattern. Each module holds functions for querying its corresponding model.
    *   **`schema.rs`**: Auto-generated by `diesel print-schema`. Defines table structures and relationships for Diesel.
    *   **`lib.rs`**: Crate entry point, declaring the modules.
*   **`tests/`**: Integration tests for the repository functions:
    *   **`common.rs`**: Test utilities for database connection, seeding (`seed_database`), and automated cleanup (`TestDatabaseGuard`).
    *   **`fixtures/`**: SQL files containing data used to seed the database for specific tests.
    *   **`*_repository_test.rs`**: Test files for each repository module.
*   **`migrations/`**: Diesel migration files (`up.sql`, `down.sql`) defining the database schema changes over time.
*   **`compose-postgres.yml`**: Docker Compose file to run PostgreSQL and Adminer for development/testing.
*   **`.env`**: Stores the database connection URL (ignored by git).

## License

This project is licensed under the **AGPL-3.0-or-later** license. See the LICENSE file for details.

## Contributing

Contributions are welcome! Please adhere to conventional commit message formats and ensure tests pass before submitting pull requests.
