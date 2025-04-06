// tests/posting_trace_repository_test.rs
//
// Copyright (c) 2018-2024 adorsys GmbH and Co. KG
// All rights are reserved.

mod common;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use postings_repository::repository::posting_trace_repository;
use common::{establish_connection, seed_database, TestDatabaseGuard};
use serial_test::serial;

#[test]
#[serial]
fn test_find_by_id() {
    // Establish a connection using our common test setup
    let mut conn = establish_connection();
    
    // Seed fixture data
    seed_database(&mut conn, "tests/fixtures/posting_trace_dataset.sql");
    
    // Create a guard that will clean up after the test
    let _guard = TestDatabaseGuard::new();
    
    // Test finding an existing posting trace
    let result = posting_trace_repository::find_by_id(&mut conn, "post_trace_001");
    assert!(result.is_ok(), "Query failed: {:?}", result.err());
    
    let posting_trace = result.unwrap();
    assert!(posting_trace.is_some(), "Expected to find post_trace_001");
    
    let trace = posting_trace.unwrap();
    assert_eq!(trace.id, "post_trace_001");
    assert_eq!(trace.tgt_pst_id, "target_pst_001");
    assert_eq!(trace.src_pst_id, "source_pst_001");
    assert_eq!(trace.src_opr_id, "source_opr_001");
    assert_eq!(trace.account_id, "account_pt_001");
    
    // Check numeric values
    // Using a string comparison to avoid floating point precision issues
    assert_eq!(trace.debit_amount.to_string(), "100.00");
    assert_eq!(trace.credit_amount.to_string(), "0.00");
    
    // Check timestamp field
    let expected_pst_time = NaiveDateTime::parse_from_str("2023-01-15 10:00:00", "%Y-%m-%d %H:%M:%S")
        .expect("Failed to parse expected src_pst_time");
    assert_eq!(trace.src_pst_time, Some(expected_pst_time));
    
    // Check hash field
    assert_eq!(trace.src_pst_hash.as_deref(), Some("hash001"));
    
    // Test finding a different posting trace with credit amount
    let result = posting_trace_repository::find_by_id(&mut conn, "post_trace_002");
    assert!(result.is_ok());
    
    let trace = result.unwrap().unwrap();
    assert_eq!(trace.id, "post_trace_002");
    assert_eq!(trace.debit_amount.to_string(), "0.00");
    assert_eq!(trace.credit_amount.to_string(), "150.00");
    
    // Test finding a posting trace with null optional fields
    let result = posting_trace_repository::find_by_id(&mut conn, "post_trace_003");
    assert!(result.is_ok());
    
    let trace = result.unwrap().unwrap();
    assert_eq!(trace.id, "post_trace_003");
    assert!(trace.src_pst_time.is_none(), "src_pst_time should be None");
    assert!(trace.src_pst_hash.is_none(), "src_pst_hash should be None");
    
    // Test finding a non-existent posting trace
    let result = posting_trace_repository::find_by_id(&mut conn, "non_existent_id");
    assert!(result.is_ok());
    assert!(result.unwrap().is_none(), "Should not find a non-existent posting trace");
} 