// tests/ledger_stmt_repository_test.rs
//
// Copyright (c) 2018-2024 adorsys GmbH and Co. KG
// All rights are reserved.

mod common;

use chrono::NaiveDateTime;
use postings_repository::models::enums::StmtStatus;
use postings_repository::repository::ledger_stmt_repository;
use common::{establish_connection, seed_database, TestDatabaseGuard};
use serial_test::serial;

#[test]
#[serial]
fn test_find_first_by_ledger_and_stmt_status_and_pst_time_lte_order_by_pst_time_desc_stmt_seq_nbr_desc() {
    // Establish a connection using our common test setup
    let mut conn = establish_connection();
    
    // Seed fixture data
    seed_database(&mut conn, "tests/fixtures/ledger_stmt_dataset.sql");
    
    // Create a guard that will clean up after the test
    let _guard = TestDatabaseGuard::new();
    
    // Test parameters
    let ledger_id_val = "ldgr_test_001";
    let target_status = StmtStatus::SIMULATED;
    
    // Case 1: Reference time between statement dates
    // Should return the latest statement before/at the reference time
    let ref_time_mid = NaiveDateTime::parse_from_str("2023-01-15 15:00:00", "%Y-%m-%d %H:%M:%S")
        .expect("Failed to parse reference time");
    
    let result_mid = ledger_stmt_repository::find_first_by_ledger_and_stmt_status_and_pst_time_lte_order_by_pst_time_desc_stmt_seq_nbr_desc(
        &mut conn,
        ledger_id_val,
        target_status,
        ref_time_mid
    ).unwrap();
    
    // Should return the statement with highest sequence number at the latest time
    assert!(result_mid.is_some(), "Expected to find a statement for mid reference time");
    let stmt_mid = result_mid.unwrap();
    assert_eq!(stmt_mid.id, "ldgr_stmt_002", "Wrong statement ID returned for mid reference time");
    assert_eq!(stmt_mid.stmt_seq_nbr, 2, "Wrong sequence number for mid reference time");
    
    // Case 2: Reference time before any statements
    // Should return None as no statements exist before this time
    let ref_time_early = NaiveDateTime::parse_from_str("2023-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")
        .expect("Failed to parse early reference time");
    
    let result_early = ledger_stmt_repository::find_first_by_ledger_and_stmt_status_and_pst_time_lte_order_by_pst_time_desc_stmt_seq_nbr_desc(
        &mut conn,
        ledger_id_val,
        target_status,
        ref_time_early
    ).unwrap();
    
    assert!(result_early.is_none(), "Should not find any statements for early reference time");
    
    // Case 3: Different status (should find CLOSED statement)
    let wrong_status = StmtStatus::CLOSED;
    
    let result_wrong_status = ledger_stmt_repository::find_first_by_ledger_and_stmt_status_and_pst_time_lte_order_by_pst_time_desc_stmt_seq_nbr_desc(
        &mut conn,
        ledger_id_val,
        wrong_status,
        ref_time_mid
    ).unwrap();
    
    // Should return the CLOSED statement
    assert!(result_wrong_status.is_some(), "Expected to find a CLOSED statement");
    assert_eq!(result_wrong_status.unwrap().id, "ldgr_stmt_004", "Wrong statement ID returned for CLOSED status");
    
    // Case 4: Different ledger ID (non-existent)
    let non_existent_ledger = "non_existent_ledger";
    
    let result_wrong_ledger = ledger_stmt_repository::find_first_by_ledger_and_stmt_status_and_pst_time_lte_order_by_pst_time_desc_stmt_seq_nbr_desc(
        &mut conn,
        non_existent_ledger,
        target_status,
        ref_time_mid
    ).unwrap();
    
    assert!(result_wrong_ledger.is_none(), "Should not find any statements for non-existent ledger");
    
    // Case 5: Early reference time with SIMULATED status
    // Should return the earliest SIMULATED statement
    let ref_time_early_with_data = NaiveDateTime::parse_from_str("2023-01-10 11:00:00", "%Y-%m-%d %H:%M:%S")
        .expect("Failed to parse early reference time with data");
    
    let result_early_with_data = ledger_stmt_repository::find_first_by_ledger_and_stmt_status_and_pst_time_lte_order_by_pst_time_desc_stmt_seq_nbr_desc(
        &mut conn,
        ledger_id_val,
        target_status,
        ref_time_early_with_data
    ).unwrap();
    
    assert!(result_early_with_data.is_some(), "Expected to find a statement for early reference time with data");
    assert_eq!(result_early_with_data.unwrap().id, "ldgr_stmt_003", "Wrong statement ID returned for early reference time with data");
} 