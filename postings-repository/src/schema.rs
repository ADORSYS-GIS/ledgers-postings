// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "account_category"))]
    pub struct AccountCategory;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "balance_side"))]
    pub struct BalanceSide;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "posting_status"))]
    pub struct PostingStatus;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "posting_type"))]
    pub struct PostingType;

    #[derive(diesel::sql_types::SqlType, diesel::QueryId)]
    #[diesel(postgres_type(name = "stmt_status"))]
    pub struct StmtStatus;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::StmtStatus;

    account_stmt (id) {
        id -> Varchar,
        posting_id -> Nullable<Varchar>,
        pst_time -> Timestamp,
        stmt_status -> StmtStatus,
        latest_pst_id -> Nullable<Varchar>,
        stmt_seq_nbr -> Int4,
        created -> Nullable<Timestamp>,
        user_details -> Nullable<Varchar>,
        short_desc -> Nullable<Varchar>,
        long_desc -> Nullable<Varchar>,
        account_id -> Varchar,
        youngest_pst_id -> Nullable<Varchar>,
        total_debit -> Numeric,
        total_credit -> Numeric,
    }
}

diesel::table! {
    chart_of_account (id) {
        id -> Varchar,
        created -> Timestamp,
        user_details -> Varchar,
        short_desc -> Nullable<Varchar>,
        long_desc -> Nullable<Varchar>,
        name -> Varchar,
    }
}

diesel::table! {
    ledger (id) {
        id -> Varchar,
        created -> Timestamp,
        user_details -> Varchar,
        short_desc -> Nullable<Varchar>,
        long_desc -> Nullable<Varchar>,
        name -> Varchar,
        coa_id -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::BalanceSide;
    use super::sql_types::AccountCategory;

    ledger_account (id) {
        id -> Varchar,
        created -> Timestamp,
        user_details -> Varchar,
        short_desc -> Nullable<Varchar>,
        long_desc -> Nullable<Varchar>,
        name -> Varchar,
        ledger_id -> Varchar,
        parent_id -> Nullable<Varchar>,
        coa_id -> Varchar,
        balance_side -> BalanceSide,
        category -> AccountCategory,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::StmtStatus;

    ledger_stmt (id) {
        id -> Varchar,
        posting_id -> Nullable<Varchar>,
        pst_time -> Timestamp,
        stmt_status -> StmtStatus,
        latest_pst_id -> Nullable<Varchar>,
        stmt_seq_nbr -> Int4,
        created -> Nullable<Timestamp>,
        user_details -> Nullable<Varchar>,
        short_desc -> Nullable<Varchar>,
        long_desc -> Nullable<Varchar>,
        ledger_id -> Varchar,
    }
}

diesel::table! {
    op_note (id) {
        id -> Varchar,
        rec_id -> Varchar,
        #[sql_name = "type"]
        note_type -> Nullable<Varchar>,
        content -> Nullable<Varchar>,
        rec_time -> Timestamp,
        exec_time -> Nullable<Timestamp>,
        premature_exc -> Nullable<Bool>,
        repeated_exec -> Nullable<Bool>,
        exec_status -> Nullable<Varchar>,
    }
}

diesel::table! {
    operation_details (id) {
        id -> Varchar,
        op_details -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PostingType;
    use super::sql_types::PostingStatus;

    posting (id) {
        id -> Varchar,
        antecedent_id -> Nullable<Varchar>,
        antecedent_hash -> Nullable<Varchar>,
        hash -> Nullable<Varchar>,
        hash_alg -> Nullable<Varchar>,
        record_user -> Varchar,
        record_time -> Timestamp,
        opr_id -> Varchar,
        opr_time -> Nullable<Timestamp>,
        opr_type -> Nullable<Varchar>,
        opr_src -> Nullable<Varchar>,
        pst_time -> Timestamp,
        pst_type -> PostingType,
        pst_status -> PostingStatus,
        ledger_id -> Varchar,
        val_time -> Nullable<Timestamp>,
        discarded_id -> Nullable<Varchar>,
        discarded_time -> Nullable<Timestamp>,
        opr_details_id -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PostingType;
    use super::sql_types::PostingStatus;

    posting_line (id) {
        id -> Varchar,
        account_id -> Varchar,
        debit_amount -> Numeric,
        credit_amount -> Numeric,
        details_id -> Nullable<Varchar>,
        src_account -> Nullable<Varchar>,
        base_line -> Nullable<Varchar>,
        sub_opr_src_id -> Nullable<Varchar>,
        record_time -> Timestamp,
        opr_id -> Varchar,
        opr_src -> Nullable<Varchar>,
        pst_time -> Timestamp,
        pst_type -> PostingType,
        pst_status -> PostingStatus,
        hash -> Varchar,
        discarded_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    posting_trace (id) {
        id -> Varchar,
        tgt_pst_id -> Varchar,
        src_pst_id -> Varchar,
        src_pst_time -> Nullable<Timestamp>,
        src_opr_id -> Varchar,
        account_id -> Varchar,
        debit_amount -> Numeric,
        credit_amount -> Numeric,
        src_pst_hash -> Nullable<Varchar>,
    }
}

diesel::joinable!(account_stmt -> ledger_account (account_id));
diesel::joinable!(account_stmt -> posting (posting_id));
diesel::joinable!(ledger -> chart_of_account (coa_id));
diesel::joinable!(ledger_account -> chart_of_account (coa_id));
diesel::joinable!(ledger_account -> ledger (ledger_id));
diesel::joinable!(ledger_stmt -> ledger (ledger_id));
diesel::joinable!(ledger_stmt -> posting (posting_id));
diesel::joinable!(ledger_stmt -> posting_trace (latest_pst_id));
diesel::joinable!(posting -> ledger (ledger_id));
diesel::joinable!(posting -> operation_details (opr_details_id));
diesel::joinable!(posting_line -> ledger_account (account_id));
diesel::joinable!(posting_line -> operation_details (details_id));
diesel::joinable!(posting_trace -> ledger_account (account_id));

diesel::allow_tables_to_appear_in_same_query!(
    account_stmt,
    chart_of_account,
    ledger,
    ledger_account,
    ledger_stmt,
    op_note,
    operation_details,
    posting,
    posting_line,
    posting_trace,
);
