-- Enums
CREATE TYPE account_category AS ENUM (
    'RE',         -- Revenue
    'EX',         -- Expense
    'AS',         -- Asset
    'LI',         -- Liability
    'EQ',         -- Equity
    'NOOP',       -- Non-Operating Income or Expenses
    'NORE',       -- Non-Operating Revenue
    'NOEX'        -- Non-Operating Expenses
);

CREATE TYPE balance_side AS ENUM (
    'Dr',
    'Cr',
    'DrCr'
);

CREATE TYPE posting_status AS ENUM (
    'DEFERRED',
    'POSTED',
    'PROPOSED',
    'SIMULATED',
    'TAX',
    'UNPOSTED',
    'CANCELLED',
    'OTHER'
);

CREATE TYPE posting_type AS ENUM (
    'BUSI_TX',
    'ADJ_TX',
    'BAL_STMT',
    'PnL_STMT',
    'BS_STMT',
    'LDG_CLSNG'
);

CREATE TYPE stmt_status AS ENUM (
    'SIMULATED',
    'CLOSED'
);

-- ===============================================
--  CHART_OF_ACCOUNT
--  (NamedEntity -> BaseEntity)
-- ===============================================
CREATE TABLE chart_of_account (
    id             VARCHAR PRIMARY KEY,
    created        TIMESTAMP NOT NULL,
    user_details   VARCHAR NOT NULL,
    short_desc     VARCHAR,
    long_desc      VARCHAR,
    name           VARCHAR NOT NULL,

    CONSTRAINT chart_of_account_name_unique UNIQUE (name)
);

-- ===============================================
--  LEDGER
--  (NamedEntity -> BaseEntity) references chart_of_account
-- ===============================================
CREATE TABLE ledger (
    id             VARCHAR PRIMARY KEY,
    created        TIMESTAMP NOT NULL,
    user_details   VARCHAR NOT NULL,
    short_desc     VARCHAR,
    long_desc      VARCHAR,
    name           VARCHAR NOT NULL,

    coa_id         VARCHAR NOT NULL,  -- references chart_of_account

    CONSTRAINT ledger_name_unique UNIQUE (name),
    CONSTRAINT fk_ledger_coa
        FOREIGN KEY (coa_id)
        REFERENCES chart_of_account (id)
);

-- ===============================================
--  LEDGER_ACCOUNT
--  (NamedEntity -> BaseEntity) references ledger, chart_of_account, parent (self)
--  stores balance_side and account_category as ENUM
-- ===============================================
CREATE TABLE ledger_account (
    id             VARCHAR PRIMARY KEY,
    created        TIMESTAMP NOT NULL,
    user_details   VARCHAR NOT NULL,
    short_desc     VARCHAR,
    long_desc      VARCHAR,
    name           VARCHAR NOT NULL,

    ledger_id      VARCHAR NOT NULL,
    parent_id      VARCHAR,
    coa_id         VARCHAR NOT NULL,

    balance_side   balance_side NOT NULL,
    category       account_category NOT NULL,

    CONSTRAINT LedgerAccount_ledger_id_name_unique
        UNIQUE (ledger_id, name),
    CONSTRAINT fk_ledger_account_ledger
        FOREIGN KEY (ledger_id)
        REFERENCES ledger (id),
    CONSTRAINT fk_ledger_account_parent
        FOREIGN KEY (parent_id)
        REFERENCES ledger_account (id),
    CONSTRAINT fk_ledger_account_coa
        FOREIGN KEY (coa_id)
        REFERENCES chart_of_account (id)
);

-- ===============================================
--  OPERATION_DETAILS
-- ===============================================
CREATE TABLE operation_details (
    id         VARCHAR PRIMARY KEY,
    op_details TEXT
);

-- ===============================================
--  POSTING
--  (HashRecord + additional fields) references ledger, optional operation_details
--  storing posting_status, posting_type as ENUM
-- ===============================================
CREATE TABLE posting (
    id                VARCHAR PRIMARY KEY,

    -- HashRecord fields:
    antecedent_id     VARCHAR,
    antecedent_hash   VARCHAR,
    hash              VARCHAR,
    hash_alg          VARCHAR,

    record_user       VARCHAR NOT NULL,
    record_time       TIMESTAMP NOT NULL,
    opr_id            VARCHAR NOT NULL,
    opr_time          TIMESTAMP,
    opr_type          VARCHAR,
    opr_src           VARCHAR,
    pst_time          TIMESTAMP NOT NULL,
    pst_type          posting_type NOT NULL,
    pst_status        posting_status NOT NULL,
    ledger_id         VARCHAR NOT NULL,
    val_time          TIMESTAMP,
    discarded_id      VARCHAR,
    discarded_time    TIMESTAMP,
    discarding_id     VARCHAR,

    opr_details_id    VARCHAR,

    CONSTRAINT posting_opr_id_discarding_id_unique
        UNIQUE (opr_id, discarding_id),
    CONSTRAINT fk_posting_ledger
        FOREIGN KEY (ledger_id)
        REFERENCES ledger (id),
    CONSTRAINT fk_posting_opr_details
        FOREIGN KEY (opr_details_id)
        REFERENCES operation_details (id)
);

-- ===============================================
--  POSTING_TRACE
--  references ledger_account
-- ===============================================
CREATE TABLE posting_trace (
    id              VARCHAR PRIMARY KEY,
    tgt_pst_id      VARCHAR NOT NULL,
    src_pst_id      VARCHAR NOT NULL,
    src_pst_time    TIMESTAMP,
    src_opr_id      VARCHAR NOT NULL,
    account_id      VARCHAR NOT NULL,
    debit_amount    NUMERIC NOT NULL,
    credit_amount   NUMERIC NOT NULL,
    src_pst_hash    VARCHAR,

    CONSTRAINT fk_posting_trace_account
        FOREIGN KEY (account_id)
        REFERENCES ledger_account (id)
);

-- ===============================================
--  LEDGER_STMT
--  (FinancialStmt + BaseEntity fields) references posting, posting_trace, ledger
--  uses enum stmt_status
-- ===============================================
CREATE TABLE ledger_stmt (
    id             VARCHAR PRIMARY KEY,
    posting_id     VARCHAR,
    pst_time       TIMESTAMP NOT NULL,
    stmt_status    stmt_status NOT NULL,
    latest_pst_id  VARCHAR,
    stmt_seq_nbr   INT NOT NULL,

    -- BaseEntity
    created        TIMESTAMP,
    user_details   VARCHAR,
    short_desc     VARCHAR,
    long_desc      VARCHAR,

    ledger_id      VARCHAR NOT NULL,

    CONSTRAINT fk_ledger_stmt_posting
        FOREIGN KEY (posting_id)
        REFERENCES posting (id),
    CONSTRAINT fk_ledger_stmt_latest_pst
        FOREIGN KEY (latest_pst_id)
        REFERENCES posting_trace (id),
    CONSTRAINT fk_ledger_stmt_ledger
        FOREIGN KEY (ledger_id)
        REFERENCES ledger (id)
);

-- ===============================================
--  ACCOUNT_STMT
--  (FinancialStmt + BaseEntity) references ledger_account, posting, posting_trace
--  uses enum stmt_status
-- ===============================================
CREATE TABLE account_stmt (
    id              VARCHAR PRIMARY KEY,
    posting_id      VARCHAR,
    pst_time        TIMESTAMP NOT NULL,
    stmt_status     stmt_status NOT NULL,
    latest_pst_id   VARCHAR,
    stmt_seq_nbr    INT NOT NULL,

    -- BaseEntity
    created         TIMESTAMP,
    user_details    VARCHAR,
    short_desc      VARCHAR,
    long_desc       VARCHAR,

    account_id      VARCHAR NOT NULL,
    youngest_pst_id VARCHAR,
    total_debit     NUMERIC NOT NULL,
    total_credit    NUMERIC NOT NULL,

    CONSTRAINT fk_account_stmt_posting
        FOREIGN KEY (posting_id)
        REFERENCES posting (id),
    CONSTRAINT fk_account_stmt_latest_pst
        FOREIGN KEY (latest_pst_id)
        REFERENCES posting_trace (id),
    CONSTRAINT fk_account_stmt_account
        FOREIGN KEY (account_id)
        REFERENCES ledger_account (id),
    CONSTRAINT fk_account_stmt_youngest_pst
        FOREIGN KEY (youngest_pst_id)
        REFERENCES posting_trace (id)
);

-- ===============================================
--  POSTING_LINE
--  references ledger_account, operation_details (optional)
--  also copies some posting fields for denormalization
-- ===============================================
CREATE TABLE posting_line (
    id             VARCHAR PRIMARY KEY,
    account_id     VARCHAR NOT NULL,
    debit_amount   NUMERIC NOT NULL,
    credit_amount  NUMERIC NOT NULL,
    details_id     VARCHAR,
    src_account    VARCHAR,
    base_line      VARCHAR,
    sub_opr_src_id VARCHAR,

    record_time    TIMESTAMP NOT NULL,
    opr_id         VARCHAR NOT NULL,
    opr_src        VARCHAR,
    pst_time       TIMESTAMP NOT NULL,
    pst_type       posting_type NOT NULL,
    pst_status     posting_status NOT NULL,
    hash           VARCHAR NOT NULL,
    discarded_time TIMESTAMP,

    CONSTRAINT fk_posting_line_account
        FOREIGN KEY (account_id)
        REFERENCES ledger_account (id),
    CONSTRAINT fk_posting_line_details
        FOREIGN KEY (details_id)
        REFERENCES operation_details (id)
);

-- ===============================================
--  OP_NOTE
-- ===============================================
CREATE TABLE op_note (
    id            VARCHAR PRIMARY KEY,
    rec_id        VARCHAR NOT NULL,
    type          VARCHAR,
    content       VARCHAR,
    rec_time      TIMESTAMP NOT NULL,
    exec_time     TIMESTAMP,
    premature_exc BOOLEAN,
    repeated_exec BOOLEAN,
    exec_status   VARCHAR
);
