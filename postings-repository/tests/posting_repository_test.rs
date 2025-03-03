mod common;

use common::{establish_connection, seed_database, TestDatabaseGuard};
use postings_repository::repository::posting_repository;
use serial_test::serial;

#[test]
#[serial]
fn test_find_by_id() {
    // establish a connection
    let mut conn = establish_connection();
    //seed fixture data
    seed_database(&mut conn, "tests/fixtures/posting_dataset.sql");
    // create a guard that will perform cleanup after the test
    let _guard = TestDatabaseGuard::new();  

    // Test retrieving a posting by ID
    let posting = posting_repository::find_by_id(&mut conn, "pst_001").unwrap();
    assert_eq!(posting.unwrap().id, "pst_001");
}

#[test]
#[serial]
fn test_find_by_opr_id() {  
    // establish a connection
    let mut conn = establish_connection();
    //seed fixture data
    seed_database(&mut conn, "tests/fixtures/posting_dataset.sql");
    // create a guard that will perform cleanup after the test
    let _guard = TestDatabaseGuard::new();  
    // Test retrieving postings by operation ID
    let postings = posting_repository::find_by_opr_id(&mut conn, "opr_001").unwrap();
    assert_eq!(postings.len(), 1);
    assert_eq!(postings[0].id, "pst_001");
}

#[test]
#[serial]
fn test_find_by_opr_id_and_discarding_id_is_null() {    
    // establish a connection
    let mut conn = establish_connection();
    //seed fixture data
    seed_database(&mut conn, "tests/fixtures/posting_dataset.sql");
    // create a guard that will perform cleanup after the test
    let _guard = TestDatabaseGuard::new();  
    // Test finding non-discarded postings
    let posting = posting_repository::find_by_opr_id_and_discarding_id_is_null(&mut conn, "opr_001").unwrap();
    assert_eq!(posting.unwrap().id, "pst_001");
}

#[test]
#[serial]
fn test_find_first_by_ledger_order_by_record_time_desc() {
    // establish a connection
    let mut conn = establish_connection();
    //seed fixture data
    seed_database(&mut conn, "tests/fixtures/posting_dataset.sql");
    // create a guard that will perform cleanup after the test
    let _guard = TestDatabaseGuard::new();  
    // Test finding the most recent posting for a ledger
    let posting = posting_repository::find_first_by_ledger_order_by_record_time_desc(&mut conn, "ledger_001").unwrap();
    assert_eq!(posting.unwrap().id, "pst_001");
}