// tests/op_note_repository_test.rs
//
// Copyright (c) 2018-2024 adorsys GmbH and Co. KG
// All rights are reserved.

mod common;

use chrono::NaiveDateTime;
use postings_repository::repository::op_note_repository;
use common::{establish_connection, seed_database, TestDatabaseGuard};
use serial_test::serial;

#[test]
#[serial]
fn test_find_by_id() {
    // Establish a connection using our common test setup
    let mut conn = establish_connection();
    
    // Seed fixture data
    seed_database(&mut conn, "tests/fixtures/op_note_dataset.sql");
    
    // Create a guard that will clean up after the test
    let _guard = TestDatabaseGuard::new();
    
    // Test finding an existing op_note
    let result = op_note_repository::find_by_id(&mut conn, "op_note_001");
    assert!(result.is_ok(), "Query failed: {:?}", result.err());
    
    let op_note = result.unwrap();
    assert!(op_note.is_some(), "Expected to find op_note_001");
    
    let op_note = op_note.unwrap();
    assert_eq!(op_note.id, "op_note_001");
    assert_eq!(op_note.rec_id, "record_001");
    assert_eq!(op_note.note_type.as_deref(), Some("REMINDER"));
    assert_eq!(op_note.content.as_deref(), Some("Test reminder note"));
    
    // Verify the record time
    let expected_rec_time = NaiveDateTime::parse_from_str("2023-01-10 10:00:00", "%Y-%m-%d %H:%M:%S")
        .expect("Failed to parse expected rec_time");
    assert_eq!(op_note.rec_time, expected_rec_time);
    
    // Verify null fields
    assert!(op_note.exec_time.is_none());
    assert_eq!(op_note.premature_exc, Some(false));
    assert_eq!(op_note.repeated_exec, Some(false));
    assert!(op_note.exec_status.is_none());
    
    // Test finding a note with execution time and status
    let result = op_note_repository::find_by_id(&mut conn, "op_note_002");
    assert!(result.is_ok());
    
    let op_note = result.unwrap().unwrap();
    assert_eq!(op_note.id, "op_note_002");
    assert_eq!(op_note.exec_status.as_deref(), Some("COMPLETED"));
    
    let expected_exec_time = NaiveDateTime::parse_from_str("2023-02-15 15:00:00", "%Y-%m-%d %H:%M:%S")
        .expect("Failed to parse expected exec_time");
    assert_eq!(op_note.exec_time, Some(expected_exec_time));
    
    // Test finding a non-existent op_note
    let result = op_note_repository::find_by_id(&mut conn, "non_existent_id");
    assert!(result.is_ok());
    assert!(result.unwrap().is_none(), "Should not find a non-existent op_note");
    
    // Test finding a note with premature execution flag
    let result = op_note_repository::find_by_id(&mut conn, "op_note_003");
    assert!(result.is_ok());
    
    let op_note = result.unwrap().unwrap();
    assert_eq!(op_note.id, "op_note_003");
    assert_eq!(op_note.premature_exc, Some(true));
    
    // Test finding a note with repeated execution flag
    let result = op_note_repository::find_by_id(&mut conn, "op_note_004");
    assert!(result.is_ok());
    
    let op_note = result.unwrap().unwrap();
    assert_eq!(op_note.id, "op_note_004");
    assert_eq!(op_note.repeated_exec, Some(true));
} 