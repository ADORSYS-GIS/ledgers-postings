# Postings Repository (Rust Version)

This project is the Rust version of the ledger solution originally available at [adorsys/ledgers](https://github.com/adorsys/ledgers). It implements the postings repository module in Rust using Diesel and other modern Rust libraries.

## Overview

- **Rust Implementation:** A modern rewrite of the ledger postings repository in Rust.
- **Data Layer:** Uses Diesel ORM with PostgreSQL and custom enum mappings.
- **Testing:** Integration tests are available (e.g., `tests/account_stmt_repository_test.rs`).

## Getting Started

### 0. Short form

To run the tests, ensure docker is running and simply execute:

```bash
cargo test
```

### 1. Start the Development PostgreSQL Database (optional)

This is done automatically if the container is not on. But the test procedure do not stop the containers after the test.

Use Docker Compose to start a development PostgreSQL instance and Adminer for easy database access.

1. Open a terminal and navigate to the `postings-repository` directory.
2. Run the following command:

   ```bash
   docker compose -f postings-repository/compose-postgres.yml up -d
   ```

This command does the following:
- **Starts a naked PostgreSQL instance** with a database named `mydb`.
- **Installs Adminer** for managing the PostgreSQL instance.  
  You can access Adminer at:  
  [http://localhost:18080/?pgsql=postgres&username=user](http://localhost:18080/?pgsql=postgres&username=user)

### 2. Initialize the Database Schema (optional)

This is also done automatically by the test scripts.

After starting the PostgreSQL instance, run the Diesel migrations to create the tables and seed the database.

From the `postings-repository` directory, execute:

```bash
diesel migration run
```

Make sure your `.env` file is configured with the correct `DATABASE_URL` (e.g., `postgres://user:password@localhost/mydb`).

### 3. Running the Tests

Integration tests have been written for the repository layer. The current available test is:

- `postings-repository/tests/account_stmt_repository_test.rs`

To run the tests, simply execute:

```bash
cargo test
```

The tests will:
- Seed the database with fixture data.
- Run repository functions.
- Clean up the database after each test run.

## Project Structure

- **src/** – Contains the main Rust source code:
  - **models/** – Domain models, including custom enums.
  - **repository/** – Repository layer with Diesel queries.
  - **schema.rs** – Diesel-generated schema definitions.
- **tests/** – Integration tests:
  - **common.rs** – Common test utilities for establishing connections, seeding, and cleanup.
  - **account_stmt_repository_test.rs** – Sample integration test for the account statement repository.
- **migrations/** – Diesel migrations to create and seed the database.
- **postings-repository/compose-postgres.yml** – Docker Compose file to run PostgreSQL and Adminer.

## License

This project is licensed under the **AGPL-3.0-or-later** license. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please see the contributing guidelines in this repository before submitting pull requests.
