// src/repository/mod.rs
/* 
 * Copyright (c) 2018-2024 adorsys GmbH and Co. KG
 * All rights are reserved.
 */

use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{OptionalExtension, QueryDsl, ExpressionMethods, RunQueryDsl};
use rust_decimal::Decimal;

use crate::models::{
    AccountStmt, Ledger, LedgerAccount, LedgerStmt, Posting, PostingLine, PostingTrace, OpNote,
    ChartOfAccount,
};
use crate::models::enums::{StmtStatus};

// AccountStmtRepository-like
//
pub mod account_stmt_repository {
    use super::*;

    use diesel::prelude::*;
use crate::models::{AccountStmt, NewAccountStmt};
use crate::schema::account_stmt::dsl::*;

    pub fn save(conn: &mut PgConnection, new_stmt: NewAccountStmt) -> QueryResult<AccountStmt> {
        diesel::insert_into(account_stmt)
            .values(&new_stmt)
            .get_result(conn)
    }

    /// findFirstByAccountAndStmtStatusAndPstTimeLessThanOrderByPstTimeDescStmtSeqNbrDesc(...)
    pub fn find_first_by_account_and_stmt_status_and_pst_time_less_than_order_by_pst_time_desc_stmt_seq_nbr_desc(
        conn: &mut PgConnection,
        account_id_val: &str,
        stmt_status_val: StmtStatus,
        ref_time_val: NaiveDateTime,
    ) -> QueryResult<Option<AccountStmt>> {
        use crate::schema::account_stmt::dsl::*;
        account_stmt
            .filter(account_id.eq(account_id_val))
            .filter(stmt_status.eq(stmt_status_val))
            .filter(pst_time.lt(ref_time_val))
            .order_by((pst_time.desc(), stmt_seq_nbr.desc()))
            .first::<AccountStmt>(conn)
            .optional()
    }

    /// findFirstByAccountAndStmtStatusAndPstTimeGreaterThanEqual(...)
    pub fn find_first_by_account_and_stmt_status_and_pst_time_gte(
        conn: &mut PgConnection,
        account_id_val: &str,
        stmt_status_val: StmtStatus,
        ref_time_val: NaiveDateTime,
    ) -> QueryResult<Option<AccountStmt>> {
        use crate::schema::account_stmt::dsl::*;
        account_stmt
            .filter(account_id.eq(account_id_val))
            .filter(stmt_status.eq(stmt_status_val))
            .filter(pst_time.ge(ref_time_val))
            .order_by((pst_time.asc(), stmt_seq_nbr.asc()))
            .first::<AccountStmt>(conn)
            .optional()
    }
}

//
// LedgerAccountRepository-like
//
pub mod ledger_account_repository {
    use super::*;

    /// findById(...) => Return an Option<LedgerAccount> by primary key
    pub fn find_by_id(
        conn: &mut PgConnection,
        account_id_val: &str,
    ) -> QueryResult<Option<LedgerAccount>> {
        use crate::schema::ledger_account::dsl::*;
        ledger_account
            .find(account_id_val)
            .first::<LedgerAccount>(conn)
            .optional()
    }

    /// findOptionalByLedgerAndName(...)
    pub fn find_optional_by_ledger_and_name(
        conn: &mut PgConnection,
        ledger_id_val: &str,
        account_name: &str,
    ) -> QueryResult<Option<LedgerAccount>> {
        use crate::schema::ledger_account::dsl::*;
        ledger_account
            .filter(ledger_id.eq(ledger_id_val))
            .filter(name.eq(account_name))
            .first::<LedgerAccount>(conn)
            .optional()
    }
}

//
// LedgerRepository-like
//
pub mod ledger_repository {
    use super::*;

    /// findById(...)
    pub fn find_by_id(conn: &mut PgConnection, ledger_id_val: &str) -> QueryResult<Option<Ledger>> {
        use crate::schema::ledger::dsl::*;
        ledger
            .find(ledger_id_val)
            .first::<Ledger>(conn)
            .optional()
    }

    /// findOptionalByName(...)
    pub fn find_optional_by_name(
        conn: &mut PgConnection,
        ledger_name_val: &str,
    ) -> QueryResult<Option<Ledger>> {
        use crate::schema::ledger::dsl::*;
        ledger
            .filter(name.eq(ledger_name_val))
            .first::<Ledger>(conn)
            .optional()
    }
}

//
// PostingRepository-like
//
pub mod posting_repository {
    use super::*;

    /// findById(...) if you need a direct "findById" for posting
    pub fn find_by_id(conn: &mut PgConnection, pst_id: &str) -> QueryResult<Option<Posting>> {
        use crate::schema::posting::dsl::*;
        posting
            .find(pst_id)
            .first::<Posting>(conn)
            .optional()
    }

    /// findByOprId(...)
    pub fn find_by_opr_id(conn: &mut PgConnection, opr_id_val: &str) -> QueryResult<Vec<Posting>> {
        use crate::schema::posting::dsl::*;
        posting
            .filter(opr_id.eq(opr_id_val))
            .load::<Posting>(conn)
    }

    /// findByOprIdAndDiscardingIdIsNull(...)
    pub fn find_by_opr_id_and_discarding_id_is_null(
        conn: &mut PgConnection,
        opr_id_val: &str,
    ) -> QueryResult<Option<Posting>> {
        use crate::schema::posting::dsl::*;
        posting
            .filter(opr_id.eq(opr_id_val))
            // TODO: Find another way to select the last record.
            //.filter(discarding_id.is_null())
            .first::<Posting>(conn)
            .optional()
    }

    /// findFirstByLedgerOrderByRecordTimeDesc(...)
    pub fn find_first_by_ledger_order_by_record_time_desc(
        conn: &mut PgConnection,
        ledger_id_val: &str,
    ) -> QueryResult<Option<Posting>> {
        use crate::schema::posting::dsl::*;
        posting
            .filter(ledger_id.eq(ledger_id_val))
            .order_by(record_time.desc())
            .first::<Posting>(conn)
            .optional()
    }
}

/// PostingLineRepository-like functions
pub mod posting_line_repository {
    use super::*;
    use crate::schema::posting_line::dsl::*;

    /// findPostingsByAccountAndDates(...) ignoring pagination
    pub fn find_postings_by_account_and_dates(
        conn: &mut PgConnection,
        account_id_val: &str,
        from_dt: NaiveDateTime,
        to_dt: NaiveDateTime,
    ) -> QueryResult<Vec<PostingLine>> {
        posting_line
            .filter(account_id.eq(account_id_val))
            .filter(pst_time.gt(from_dt))
            .filter(pst_time.le(to_dt))
            .filter(discarded_time.is_null())
            .order_by(pst_time.desc())
            .load::<PostingLine>(conn)
    }

    /// findFirstByIdAndAccount(...)
    pub fn find_first_by_id_and_account(
        conn: &mut PgConnection,
        transaction_id: &str,
        account_id_val: &str,
    ) -> QueryResult<Option<PostingLine>> {
        posting_line
            .filter(id.eq(transaction_id))
            .filter(account_id.eq(account_id_val))
            .first::<PostingLine>(conn)
            .optional()
    }

    // ------------------------------------------------------------
    // ADDITIONAL FINDER #1
    //   postingLineRepository.findByAccountAndPstTimeLessThanEqualAndDiscardedTimeIsNullOrderByRecordTimeDesc(account, refTime)
    // ------------------------------------------------------------
    pub fn find_by_account_and_pst_time_lte_and_discarded_is_null_order_by_record_time_desc(
        conn: &mut PgConnection,
        account_id_val: &str,
        ref_time_val: NaiveDateTime,
    ) -> QueryResult<Vec<PostingLine>> {
        posting_line
            .filter(account_id.eq(account_id_val))
            .filter(pst_time.le(ref_time_val))
            .filter(discarded_time.is_null())
            .order_by(record_time.desc())
            .load::<PostingLine>(conn)
    }

    // ------------------------------------------------------------
    // ADDITIONAL FINDER #2
    //   postingLineRepository.findByBaseLineAndPstTimeLessThanEqualAndDiscardedTimeIsNullOrderByRecordTimeDesc(accStmt.getId(), refTime);
    // ------------------------------------------------------------
    pub fn find_by_base_line_and_pst_time_lte_and_discarded_is_null_order_by_record_time_desc(
        conn: &mut PgConnection,
        base_line_val: &str,
        ref_time_val: NaiveDateTime,
    ) -> QueryResult<Vec<PostingLine>> {
        posting_line
            .filter(base_line.eq(base_line_val))
            .filter(pst_time.le(ref_time_val))
            .filter(discarded_time.is_null())
            .order_by(record_time.desc())
            .load::<PostingLine>(conn)
    }
}

//
// LedgerStmtRepository-like (for reference)
//
pub mod ledger_stmt_repository {
    use super::*;

    /// Example from original code, if needed
    pub fn find_first_by_ledger_and_stmt_status_and_pst_time_lte_order_by_pst_time_desc_stmt_seq_nbr_desc(
        conn: &mut PgConnection,
        ledger_id_val: &str,
        stmt_status_val: StmtStatus,
        ref_time_val: NaiveDateTime,
    ) -> QueryResult<Option<LedgerStmt>> {
        use crate::schema::ledger_stmt::dsl::*;
        ledger_stmt
            .filter(ledger_id.eq(ledger_id_val))
            .filter(stmt_status.eq(stmt_status_val))
            .filter(pst_time.le(ref_time_val))
            .order_by((pst_time.desc(), stmt_seq_nbr.desc()))
            .first::<LedgerStmt>(conn)
            .optional()
    }
}

//
// Example: OpNoteRepository-like (if needed)
//
pub mod op_note_repository {
    use super::*;

    /// findById(...) for OpNote
    pub fn find_by_id(conn: &mut PgConnection, note_id: &str) -> QueryResult<Option<OpNote>> {
        use crate::schema::op_note::dsl::*;
        op_note
            .find(note_id)
            .first::<OpNote>(conn)
            .optional()
    }
}

//
// PostingTraceRepository-like (if needed)
//
pub mod posting_trace_repository {
    use super::*;

    /// findById(...)
    pub fn find_by_id(conn: &mut PgConnection, trace_id: &str) -> QueryResult<Option<PostingTrace>> {
        use crate::schema::posting_trace::dsl::*;
        posting_trace
            .find(trace_id)
            .first::<PostingTrace>(conn)
            .optional()
    }
}
