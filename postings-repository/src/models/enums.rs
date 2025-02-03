// src/models/enums.rs
/* 
 * Copyright (c) 2018-2024 adorsys GmbH and Co. KG
 * All rights are reserved.
 */

use serde::{Serialize, Deserialize};
use diesel_derive_enum::DbEnum;


/// Each account belongs to an account category. We distinguish the following:
/// - Revenue
/// - Expense
/// - Asset
/// - Liability
/// - Equity
/// This matches the `CREATE TYPE account_category AS ENUM (...)` in up.sql:
/// ('RE','EX','AS','LI','EQ','NOOP','NORE','NOEX')
#[derive(DbEnum, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// Tells Diesel the SQL type is `schema::sql_types::AccountCategory`
#[ExistingTypePath = "crate::schema::sql_types::AccountCategory"]  
pub enum AccountCategory {
    #[db_rename = "RE"]
    RE,    // "RE", "Revenue",BalanceSide.Cr
    #[db_rename = "EX"]
    EX,    // "EX", "Expense",BalanceSide.Dr
    #[db_rename = "AS"]
    AS,    // "AS", "Asset",BalanceSide.Dr
    #[db_rename = "LI"]
    LI,    // "LI", "Liability",BalanceSide.Cr
    #[db_rename = "EQ"]
    EQ,    // "EQ", "Equity",BalanceSide.Cr
    #[db_rename = "NOOP"]
    NOOP,  // "NOOP", "Non-Operating Income or Expenses",BalanceSide.DrCr
    #[db_rename = "NORE"]
    NORE,  // "NORE", "Non-Operating Revenue",BalanceSide.Cr
    #[db_rename = "NOEX"]
    NOEX,  // "NOEX", "Non-Operating Expenses",BalanceSide.Dr
}

/// The balance side describes the side of the balance where the account balance
/// increases.
/// Matches `CREATE TYPE balance_side AS ENUM ('Dr','Cr','DrCr')`
#[derive(DbEnum, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// Tells Diesel the SQL type is `schema::sql_types::BalanceSide`
#[ExistingTypePath = "crate::schema::sql_types::BalanceSide"]  
pub enum BalanceSide {
    #[db_rename = "Dr"]
    Dr,   // "Dr"
    #[db_rename = "Cr"]
    Cr,   // "Cr"
    #[db_rename = "DrCr"]
    DrCr, // "DrCr"
}

/// Matches `CREATE TYPE posting_status AS ENUM (...)`:
/// ('DEFERRED','POSTED','PROPOSED','SIMULATED','TAX','UNPOSTED','CANCELLED','OTHER')
#[derive(DbEnum, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::PostingStatus"]  
pub enum PostingStatus {
    #[db_rename = "DEFERRED"]
    DEFERRED,
    #[db_rename = "POSTED"]
    POSTED,
    #[db_rename = "PROPOSED"]
    PROPOSED,
    #[db_rename = "SIMULATED"]
    SIMULATED,
    #[db_rename = "TAX"]
    TAX,
    #[db_rename = "UNPOSTED"]
    UNPOSTED,
    #[db_rename = "CANCELLED"]
    CANCELLED,
    #[db_rename = "OTHER"]
    OTHER,
}

/// Matches `CREATE TYPE posting_type AS ENUM (...)`:
/// ('BUSI_TX','ADJ_TX','BAL_STMT','PnL_STMT','BS_STMT','LDG_CLSNG')
#[derive(DbEnum, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::PostingType"]  
pub enum PostingType {
    /// Describes a business transaction involving different accounts and affecting account balances.
    #[db_rename = "BUSI_TX"]
    BusiTx,
    /// Describes an adjustment transaction involving different accounts and affecting account balances.
    #[db_rename = "ADJ_TX"]
    AdjTx,
    /// Documents the balance of a ledger account.
    #[db_rename = "BAL_STMT"]
    BalStmt,
    #[db_rename = "PnL_STMT"]
    PnLStmt,
    #[db_rename = "BS_STMT"]
    BsStmt,
    /// Document the closing of a ledger.
    #[db_rename = "LDG_CLSNG"]
    LdgClsng,
}

/// Matches `CREATE TYPE stmt_status AS ENUM ('SIMULATED','CLOSED')`
#[derive(DbEnum, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::StmtStatus"]  
pub enum StmtStatus {
    #[db_rename = "SIMULATED"]
    SIMULATED,
    #[db_rename = "CLOSED"]
    CLOSED,
}
