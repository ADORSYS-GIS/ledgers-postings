// tests/account_stmt_repository_test.rs
//
// Copyright (c) 2018-2024 adorsys GmbH and Co. KG
// All rights are reserved.

mod common;

use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use rust_decimal::Decimal;
use postings_repository::models::{AccountStmt, NewAccountStmt};
use postings_repository::models::enums::StmtStatus;
use postings_repository::repository::{account_stmt_repository, ledger_account_repository};
use common::{establish_connection, seed_database, TestDatabaseGuard};

#[test]
fn test_create_financial_statement_ok() {
    // Establish a connection using our common test setup.
    let mut conn = establish_connection();

    // Seed fixture data
    seed_database(&mut conn);    

    // Create a guard that will clean up after the test.
    let _guard = TestDatabaseGuard::new();
    
    // Retrieve an existing LedgerAccount by ID from the test database.
    let ledger_account_id = "xVgaTPMcRty9ik3BTQDh1Q_BS_1_0_0";
    let account = ledger_account_repository::find_by_id(&mut conn, ledger_account_id)
        .expect("Error fetching LedgerAccount")
        .expect("Missing LedgerAccount with id xVgaTPMcRty9ik3BTQDh1Q_BS_1_0_0");

    // Create a posting time for December 31, 2017 at 23:59.
    let pst_time = NaiveDateTime::parse_from_str("2017-12-31 23:59:00", "%Y-%m-%d %H:%M:%S")
        .expect("Failed to parse posting time");

    // Construct a new AccountStmt (using a NewAccountStmt struct for insertion).
    let new_stmt = NewAccountStmt {
        id: "generated-id-001".to_string(), // In production, you might generate this dynamically.
        account_id: account.id.clone(),
        stmt_status: StmtStatus::SIMULATED,
        stmt_seq_nbr: 0,
        pst_time,
        total_credit: Decimal::ZERO,
        total_debit: Decimal::ZERO,
        // BaseEntity fields (or equivalent) can be set here:
        created: Some(Utc::now().naive_utc()),
        user_details: Some("Test User".to_string()),
        short_desc: None,
        long_desc: None,
        latest_pst_id: None,
        posting_id: None,
        youngest_pst_id: None
    };

    // Insert the new AccountStmt via the repository function.
    let saved_stmt = account_stmt_repository::save(&mut conn, new_stmt)
        .expect("Failed to save AccountStmt");

    // Assert that the saved statement has a non-empty ID.
    assert!(!saved_stmt.id.is_empty(), "Saved AccountStmt should have a non-empty id");
}
