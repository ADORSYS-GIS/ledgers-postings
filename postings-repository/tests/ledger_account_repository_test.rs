// tests/ledger_account_repository_test.rs
//
// Copyright (c) 2018-2024 adorsys GmbH and Co. KG
// All rights are reserved.

mod common;

use chrono::Local;
use diesel::prelude::*;
use postings_repository::models::{LedgerAccount, NewLedgerAccount, ChartOfAccount, Ledger};
use postings_repository::models::enums::{AccountCategory, BalanceSide};
use postings_repository::repository::{ledger_account_repository, ledger_repository};
use common::{establish_connection, seed_database, TestDatabaseGuard};
use std::env;
use serial_test::serial;

#[test]
#[serial]
fn test_create_ledger_account_ok() {
    // Establish a connection and seed fixture data.
    let mut conn = establish_connection();
    seed_database(&mut conn, "tests/fixtures/ledger_account_dataset.sql");

    // Create a guard that will clean up after the test.
    let _guard = TestDatabaseGuard::new();

    // Retrieve a ledger by its ID.
    let ledger_id = "Zd0ND5YwSzGwIfZilhumPg";
    let ledger: Ledger = ledger_repository::find_by_id(&mut conn, ledger_id)
        .expect("Error fetching Ledger")
        .expect("Ledger with id Zd0ND5YwSzGwIfZilhumPg not found");

    // Retrieve a parent LedgerAccount by ledger and name ("3.0.0").
    let parent_account: LedgerAccount = ledger_account_repository::find_optional_by_ledger_and_name(&mut conn, ledger_id, "3.0.0")
        .expect("Error fetching parent LedgerAccount")
        .expect("Parent LedgerAccount with name '3.0.0' not found");

    // Construct a new LedgerAccount.
    let new_account = NewLedgerAccount {
        id: "generated-ledgeraccount-id-001".to_string(),
        created: Local::now().naive_local(),
        user_details: "Sample User".to_string(),
        short_desc: Some("Long lasting liability".to_string()),
        long_desc: Some("Long lasting liability (from 1 year to 3 years)".to_string()),
        name: "Long lasting liability".to_string(),
        ledger_id: ledger.id.clone(),
        parent_id: Some(parent_account.id.clone()),
        coa_id: ledger.coa_id.clone(),
        // Choose a proper balance side for liability accounts. For example, if liabilities increase on the credit side:
        balance_side: BalanceSide::Cr,
        category: AccountCategory::LI,
    };

    let created_account = ledger_account_repository::save(&mut conn, new_account)
        .expect("Failed to save LedgerAccount");

    assert!(!created_account.id.is_empty(), "Created LedgerAccount should have a non-empty id");
}

#[test]
#[serial]
fn test_create_ledger_account_no_ledger() {
    let mut conn = establish_connection();
    seed_database(&mut conn, "tests/fixtures/ledger_account_dataset.sql");
    let _guard = TestDatabaseGuard::new();

    // Create a LedgerAccount with missing required foreign keys (no ledger, no chart)
    let new_account = NewLedgerAccount {
        id: "generated-ledgeraccount-id-002".to_string(),
        created: Local::now().naive_local(),
        user_details: "Sample User".to_string(),
        short_desc: None,
        long_desc: None,
        name: "Sample Ledger Account".to_string(),
        ledger_id: "".to_string(), // Missing ledger id
        parent_id: None,
        coa_id: "".to_string(),    // Missing chart of account id
        balance_side: BalanceSide::Dr,
        category: AccountCategory::AS,
    };

    let result = ledger_account_repository::save(&mut conn, new_account);
    assert!(result.is_err(), "Expected error when saving LedgerAccount with missing ledger");
}

#[test]
#[serial]
fn test_create_ledger_account_unique_constrain_violation_ledger_name_valid_from() {
    let mut conn = establish_connection();
    seed_database(&mut conn, "tests/fixtures/ledger_account_dataset.sql");
    let _guard = TestDatabaseGuard::new();

    // Retrieve an existing LedgerAccount (e.g., the one with id "xVgaTPMcRty9ik3BTQDh1Q_BS_3_0_0").
    let existing_account = ledger_account_repository::find_by_id(&mut conn, "xVgaTPMcRty9ik3BTQDh1Q_BS_3_0_0")
        .expect("Error fetching LedgerAccount")
        .expect("LedgerAccount with id xVgaTPMcRty9ik3BTQDh1Q_BS_3_0_0 should exist");

    // Create a new LedgerAccount with the same name as the existing one.
    let new_account = NewLedgerAccount {
        id: "generated-ledgeraccount-id-003".to_string(),
        created: Local::now().naive_local(),
        user_details: "Sample User".to_string(),
        short_desc: Some("Long lasting liability".to_string()),
        long_desc: Some("Long lasting liability (from 1 year to 3 years)".to_string()),
        name: existing_account.name.clone(), // Duplicate name
        ledger_id: existing_account.ledger_id.clone(),
        parent_id: existing_account.parent_id.clone(),
        coa_id: existing_account.coa_id.clone(),
        balance_side: existing_account.balance_side,
        category: existing_account.category,
    };

    let result = ledger_account_repository::save(&mut conn, new_account);
    assert!(result.is_err(), "Expected a unique constraint violation when saving a duplicate LedgerAccount name");
}

#[test]
#[serial]
fn test_find_by_ledger_and_name_ok() {
    let mut conn = establish_connection();
    seed_database(&mut conn, "tests/fixtures/ledger_account_dataset.sql");
    let _guard = TestDatabaseGuard::new();

    let ledger_id = "Zd0ND5YwSzGwIfZilhumPg";
    let ledger = ledger_repository::find_by_id(&mut conn, ledger_id)
        .expect("Error fetching Ledger")
        .expect("Ledger not found");

    let found = ledger_account_repository::find_optional_by_ledger_and_name(&mut conn, ledger_id, "1.0.0")
        .expect("Error fetching LedgerAccount by ledger and name");
    assert!(found.is_some(), "Expected to find a LedgerAccount with name '1.0.0'");
}
