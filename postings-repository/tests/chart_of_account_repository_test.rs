// tests/chart_of_account_repository_test.rs
//
// Copyright (c) 2018-2024 adorsys GmbH and Co. KG
// All rights are reserved.

mod common;

use common::{establish_connection, seed_database, TestDatabaseGuard};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use postings_repository::models::{ChartOfAccount, NewChartOfAccount};
use postings_repository::repository::chart_of_account_repository;
use serial_test::serial; // import the serial attribute

#[test]
#[serial]
fn test_create_coa_ok() {
    // Establish a connection and seed fixture data.
    let mut conn = establish_connection();
    seed_database(&mut conn, "tests/fixtures/chart_of_account_dataset.sql");

    // Create a guard that will perform cleanup after the test.
    let _guard = TestDatabaseGuard::new();

    // Create a new ChartOfAccount with a unique name.
    let new_coa = NewChartOfAccount {
        id: "generated-coa-id-001".to_string(),
        // Using a fixed timestamp for testing purposes
        created: NaiveDateTime::from_timestamp(1627833600, 0),
        user_details: "francis".to_string(),
        short_desc: Some("Sample chart of account".to_string()),
        long_desc: None,
        name: "UniqueCoAName".to_string(),
    };

    // Save the new ChartOfAccount via the repository function.
    let saved_coa = chart_of_account_repository::save(&mut conn, new_coa)
        .expect("Failed to create ChartOfAccount");

    // Assert that the saved ChartOfAccount has a non-empty id.
    assert!(!saved_coa.id.is_empty(), "Saved ChartOfAccount should have a non-empty id");
}

#[test]
#[serial]
fn test_load_coa() {
    let mut conn = establish_connection();
    seed_database(&mut conn, "tests/fixtures/chart_of_account_dataset.sql");
    let _guard = TestDatabaseGuard::new();

    // Attempt to load the ChartOfAccount with id "ci8k8bcdTrCsi-F3sT3i-g".
    let coa = chart_of_account_repository::find_by_id(&mut conn, "ci8k8bcdTrCsi-F3sT3i-g")
        .expect("Error fetching ChartOfAccount")
        .expect("ChartOfAccount with id ci8k8bcdTrCsi-F3sT3i-g not found");

    // Assert that the loaded ChartOfAccount has a non-empty id.
    assert!(!coa.id.is_empty(), "Loaded ChartOfAccount should have a non-empty id");
}

#[test]
#[serial]
fn test_create_coa_unique_constrain_violation_name() {
    let mut conn = establish_connection();
    seed_database(&mut conn, "tests/fixtures/chart_of_account_dataset.sql");
    let _guard = TestDatabaseGuard::new();

    // Load an existing ChartOfAccount with id "ci8k8bcdTrCsi-F3sT3i-g".
    let existing_coa = chart_of_account_repository::find_by_id(&mut conn, "ci8k8bcdTrCsi-F3sT3i-g")
        .expect("Error fetching ChartOfAccount")
        .expect("ChartOfAccount with id ci8k8bcdTrCsi-F3sT3i-g should exist");

    // Create a new ChartOfAccount with a different id but the same name as the existing one.
    let new_coa = NewChartOfAccount {
        id: "generated-coa-id-002".to_string(),
        created: NaiveDateTime::from_timestamp(1627833600, 0),
        user_details: "francis".to_string(),
        short_desc: None,
        long_desc: None,
        name: existing_coa.name.clone(),
    };

    // Expect a unique constraint violation error when saving.
    let result = chart_of_account_repository::save(&mut conn, new_coa);
    assert!(
        result.is_err(),
        "Expected a unique constraint violation error when saving duplicate ChartOfAccount name"
    );
}
