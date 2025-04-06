// tests/ledger_repository_test.rs
//
// Copyright (c) 2018-2024 adorsys GmbH and Co. KG
// All rights are reserved.

mod common;

use postings_repository::repository::ledger_repository;
use common::{establish_connection, seed_database, TestDatabaseGuard};
use serial_test::serial;

const FIXTURE_FILE: &str = "tests/fixtures/ledger_dataset.sql";

#[test]
#[serial]
fn test_find_by_id_found() {
    let mut conn = establish_connection();
    seed_database(&mut conn, FIXTURE_FILE);
    let _guard = TestDatabaseGuard::new(); // Cleanup after test

    let ledger_id_to_find = "ledger_test_001";
    let result = ledger_repository::find_by_id(&mut conn, ledger_id_to_find);

    assert!(result.is_ok(), "Query failed: {:?}", result.err());
    let maybe_ledger = result.unwrap();
    assert!(maybe_ledger.is_some(), "Ledger with ID '{}' not found", ledger_id_to_find);

    let ledger = maybe_ledger.unwrap();
    assert_eq!(ledger.id, ledger_id_to_find);
    assert_eq!(ledger.name, "Test Ledger Alpha");
    assert_eq!(ledger.coa_id, "coa_ldg_test_001");
}

#[test]
#[serial]
fn test_find_by_id_not_found() {
    let mut conn = establish_connection();
    seed_database(&mut conn, FIXTURE_FILE);
    let _guard = TestDatabaseGuard::new();

    let ledger_id_to_find = "non_existent_ledger_id";
    let result = ledger_repository::find_by_id(&mut conn, ledger_id_to_find);

    assert!(result.is_ok(), "Query failed: {:?}", result.err());
    let maybe_ledger = result.unwrap();
    assert!(maybe_ledger.is_none(), "Expected not to find ledger with ID '{}'", ledger_id_to_find);
}

#[test]
#[serial]
fn test_find_optional_by_name_found() {
    let mut conn = establish_connection();
    seed_database(&mut conn, FIXTURE_FILE);
    let _guard = TestDatabaseGuard::new();

    let ledger_name_to_find = "Test Ledger Beta";
    let result = ledger_repository::find_optional_by_name(&mut conn, ledger_name_to_find);

    assert!(result.is_ok(), "Query failed: {:?}", result.err());
    let maybe_ledger = result.unwrap();
    assert!(maybe_ledger.is_some(), "Ledger with name '{}' not found", ledger_name_to_find);

    let ledger = maybe_ledger.unwrap();
    assert_eq!(ledger.id, "ledger_test_002");
    assert_eq!(ledger.name, ledger_name_to_find);
    assert_eq!(ledger.coa_id, "coa_ldg_test_001");
}

#[test]
#[serial]
fn test_find_optional_by_name_not_found() {
    let mut conn = establish_connection();
    seed_database(&mut conn, FIXTURE_FILE);
    let _guard = TestDatabaseGuard::new();

    let ledger_name_to_find = "Non Existent Ledger Name";
    let result = ledger_repository::find_optional_by_name(&mut conn, ledger_name_to_find);

    assert!(result.is_ok(), "Query failed: {:?}", result.err());
    let maybe_ledger = result.unwrap();
    assert!(maybe_ledger.is_none(), "Expected not to find ledger with name '{}'", ledger_name_to_find);
} 