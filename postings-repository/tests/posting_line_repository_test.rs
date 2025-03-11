// tests/posting_line_repository_test.rs
//
// Copyright (c) 2018-2024 adorsys GmbH and Co. KG
// All rights are reserved.

mod common;

use chrono::NaiveDateTime;
use diesel::RunQueryDsl;
use postings_repository::repository::posting_line_repository;
use common::{establish_connection, seed_database, TestDatabaseGuard};
use serial_test::serial;

#[test]
#[serial]
fn test_find_first_by_id_and_account() {
    // Establish a connection using our common test setup
    let mut conn = establish_connection();
    
    // Seed fixture data
    seed_database(&mut conn, "tests/fixtures/posting_line_dataset.sql");
    
    // Create a guard that will clean up after the test
    let _guard = TestDatabaseGuard::new();
    
    // Test finding a posting line by id and account
    let posting_line = posting_line_repository::find_first_by_id_and_account(
        &mut conn, 
        "pline_001", 
        "account_pl_001"
    ).unwrap();
    
    // Verify the result
    assert!(posting_line.is_some(), "Expected to find a posting line");
    let posting_line = posting_line.unwrap();
    assert_eq!(posting_line.id, "pline_001");
    assert_eq!(posting_line.account_id, "account_pl_001");
    assert_eq!(posting_line.opr_id, "opr_pl_001");
}

#[test]
#[serial]
fn test_find_postings_by_account_and_dates() {
    // Establish a connection
    let mut conn = establish_connection();
    
    // Seed fixture data
    seed_database(&mut conn, "tests/fixtures/posting_line_dataset.sql");
    
    // Create a guard that will clean up after the test
    let _guard = TestDatabaseGuard::new();
    
    // Create test date range
    let from_dt = NaiveDateTime::parse_from_str("2024-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")
        .expect("Failed to parse from_dt");
    let to_dt = NaiveDateTime::parse_from_str("2024-01-16 00:00:00", "%Y-%m-%d %H:%M:%S")
        .expect("Failed to parse to_dt");
    
    // Test finding posting lines within a date range
    let posting_lines = posting_line_repository::find_postings_by_account_and_dates(
        &mut conn, 
        "account_pl_001", 
        from_dt, 
        to_dt
    ).unwrap();
    
    // Verify the results
    assert_eq!(posting_lines.len(), 2, "Expected to find 2 posting lines in the date range");
    
    // Posting lines should be ordered by pst_time desc (most recent first)
    assert_eq!(posting_lines[0].id, "pline_002");
    assert_eq!(posting_lines[1].id, "pline_001");
    
    // Verify the discarded posting lines are filtered out
    let has_discarded = posting_lines.iter().any(|pl| pl.id == "pline_005");
    assert!(!has_discarded, "Discarded posting lines should be filtered out");
}

#[test]
#[serial]
fn test_find_by_account_and_pst_time_lte_and_discarded_is_null_order_by_record_time_desc() {
    // Establish a connection
    let mut conn = establish_connection();
    
    // Seed fixture data
    seed_database(&mut conn, "tests/fixtures/posting_line_dataset.sql");
    
    // Create a guard that will clean up after the test
    let _guard = TestDatabaseGuard::new();
    
    // Reference time for the test
    let ref_time = NaiveDateTime::parse_from_str("2024-01-20 00:00:00", "%Y-%m-%d %H:%M:%S")
        .expect("Failed to parse reference time");
    
    // Test finding posting lines by account with posting time less than or equal to ref_time
    let posting_lines = posting_line_repository::find_by_account_and_pst_time_lte_and_discarded_is_null_order_by_record_time_desc(
        &mut conn, 
        "account_pl_001", 
        ref_time
    ).unwrap();
    
    // Verify the results
    assert_eq!(posting_lines.len(), 2, "Expected to find 2 non-discarded posting lines");
    
    // Verify that the correct posting lines are included in the results
    let has_pline_001 = posting_lines.iter().any(|pl| pl.id == "pline_001");
    let has_pline_002 = posting_lines.iter().any(|pl| pl.id == "pline_002");
    
    assert!(has_pline_001 || has_pline_002, "Results should include at least one of the expected posting lines");
    
    // Verify that discarded records are not included
    let has_discarded_record = posting_lines.iter().any(|pl| pl.id == "pline_005");
    assert!(!has_discarded_record, "Results should not include discarded posting lines");
}

#[test]
#[serial]
fn test_find_by_base_line_and_pst_time_lte_and_discarded_is_null_order_by_record_time_desc() {
    // Establish a connection
    let mut conn = establish_connection();
    
    // Seed fixture data
    seed_database(&mut conn, "tests/fixtures/posting_line_dataset.sql");
    
    // Create a guard that will clean up after the test
    let _guard = TestDatabaseGuard::new();
    
    // Reference time for the test
    let ref_time = NaiveDateTime::parse_from_str("2024-01-30 00:00:00", "%Y-%m-%d %H:%M:%S")
        .expect("Failed to parse reference time");
    
    // Test finding posting lines by base_line
    let posting_lines = posting_line_repository::find_by_base_line_and_pst_time_lte_and_discarded_is_null_order_by_record_time_desc(
        &mut conn, 
        "pline_001", 
        ref_time
    ).unwrap();
    
    // Verify the results
    assert_eq!(posting_lines.len(), 1, "Expected to find 1 posting line with base_line='pline_001'");
    assert_eq!(posting_lines[0].id, "pline_002");
    
    // Test with another base_line
    let posting_lines = posting_line_repository::find_by_base_line_and_pst_time_lte_and_discarded_is_null_order_by_record_time_desc(
        &mut conn, 
        "pline_002", 
        ref_time
    ).unwrap();
    
    // Verify the results
    assert_eq!(posting_lines.len(), 1, "Expected to find 1 posting line with base_line='pline_002'");
    assert_eq!(posting_lines[0].id, "pline_004");
    
    // Let's add a posting line that should be filtered out because it has a discarded_time
    // and verify it's not returned even though it matches the base_line criteria
    
    // First, let's add a test entry to the database with base_line='pline_001' but with discarded_time
    diesel::sql_query("
        INSERT INTO posting_line (
            id, account_id, debit_amount, credit_amount, 
            record_time, opr_id, opr_src, pst_time, 
            pst_type, pst_status, hash, base_line, discarded_time
        ) VALUES (
            'pline_discarded', 'account_pl_001', 30.00, 0.00,
            '2024-01-25 10:00:00', 'opr_pl_005', 'payment_005', '2024-01-25 09:45:00',
            'BUSI_TX', 'POSTED', 'hash099', 'pline_001', '2024-01-26 08:00:00'
        )
    ").execute(&mut conn).expect("Failed to insert test discarded posting line");
    
    // Now search again with the same criteria, the discarded record should not be returned
    let posting_lines = posting_line_repository::find_by_base_line_and_pst_time_lte_and_discarded_is_null_order_by_record_time_desc(
        &mut conn, 
        "pline_001", 
        ref_time
    ).unwrap();
    
    // Verify the discarded record is not included
    assert_eq!(posting_lines.len(), 1, "Should still find only 1 posting line with base_line='pline_001'");
    assert_eq!(posting_lines[0].id, "pline_002");
    
    // Verify that the discarded posting line is not in the results
    let has_discarded = posting_lines.iter().any(|pl| pl.id == "pline_discarded");
    assert!(!has_discarded, "Results should not include discarded posting lines");
}