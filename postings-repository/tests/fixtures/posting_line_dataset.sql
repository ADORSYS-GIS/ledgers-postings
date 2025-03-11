-- tests/fixtures/posting_line_dataset.sql
-- 
-- Copyright (c) 2018-2024 adorsys GmbH and Co. KG
-- All rights are reserved.
--

-- 1. First create the chart of account record (required by ledger)
INSERT INTO chart_of_account (
    id,
    created,
    user_details,
    name
) VALUES (
    'coa_pl_001',                -- id
    '2024-01-01 10:00:00',      -- created
    'test_user',                -- user_details
    'Test Chart of Account'     -- name
);

-- 2. Then create the ledger record (required by ledger_account)
INSERT INTO ledger (
    id,
    created,
    user_details,
    name,
    coa_id
) VALUES (
    'ledger_pl_001',             -- id
    '2024-01-01 10:00:00',      -- created
    'test_user',                -- user_details
    'Test Ledger',              -- name
    'coa_pl_001'                -- coa_id (references chart_of_account.id)
);

-- 3. Create two ledger accounts: one for debit and one for credit
INSERT INTO ledger_account (
    id,
    created,
    user_details,
    ledger_id,
    coa_id,
    balance_side,
    category,
    name,
    short_desc
) VALUES 
    ('account_pl_001',           -- id
    '2024-01-01 10:00:00',      -- created
    'test_user',                -- user_details
    'ledger_pl_001',            -- ledger_id
    'coa_pl_001',               -- coa_id
    'Dr',                       -- balance_side
    'AS',                       -- category (Asset)
    '1.0.0',                    -- name
    'Asset Account'),           -- short_desc
    
    ('account_pl_002',           -- id
    '2024-01-01 10:00:00',      -- created
    'test_user',                -- user_details
    'ledger_pl_001',            -- ledger_id
    'coa_pl_001',               -- coa_id
    'Cr',                       -- balance_side
    'LI',                       -- category (Liability)
    '2.0.0',                    -- name
    'Liability Account');       -- short_desc

-- 4. Create posting line records with different timestamps for testing
INSERT INTO posting_line (
    id,
    account_id,
    debit_amount,
    credit_amount,
    record_time,
    opr_id,
    opr_src,
    pst_time,
    pst_type,
    pst_status,
    hash,
    base_line,
    discarded_time
) VALUES 
    -- Main test posting line
    ('pline_001',                -- id
    'account_pl_001',           -- account_id
    100.00,                     -- debit_amount
    0.00,                       -- credit_amount
    '2024-01-10 12:00:00',      -- record_time
    'opr_pl_001',               -- opr_id
    'payment_001',              -- opr_src
    '2024-01-10 11:59:00',      -- pst_time
    'BUSI_TX',                  -- pst_type
    'POSTED',                   -- pst_status
    'hash001',                  -- hash
    NULL,                       -- base_line
    NULL),                      -- discarded_time
    
    -- Second posting line (different time, same account)
    ('pline_002',                -- id
    'account_pl_001',           -- account_id
    50.00,                      -- debit_amount
    0.00,                       -- credit_amount
    '2024-01-15 15:30:00',      -- record_time
    'opr_pl_002',               -- opr_id
    'payment_002',              -- opr_src
    '2024-01-15 15:25:00',      -- pst_time
    'BUSI_TX',                  -- pst_type
    'POSTED',                   -- pst_status
    'hash002',                  -- hash
    'pline_001',                -- base_line (referencing first posting line)
    NULL),                      -- discarded_time
    
    -- Third posting line (for a different account)
    ('pline_003',                -- id
    'account_pl_002',           -- account_id
    0.00,                       -- debit_amount
    100.00,                     -- credit_amount
    '2024-01-10 12:00:00',      -- record_time
    'opr_pl_001',               -- opr_id
    'payment_001',              -- opr_src
    '2024-01-10 11:59:00',      -- pst_time
    'BUSI_TX',                  -- pst_type
    'POSTED',                   -- pst_status
    'hash003',                  -- hash
    NULL,                       -- base_line
    NULL),                      -- discarded_time
    
    -- Fourth posting line (for testing date ranges)
    ('pline_004',                -- id
    'account_pl_001',           -- account_id
    25.00,                      -- debit_amount
    0.00,                       -- credit_amount
    '2024-01-20 09:45:00',      -- record_time
    'opr_pl_003',               -- opr_id
    'payment_003',              -- opr_src
    '2024-01-20 09:30:00',      -- pst_time
    'BUSI_TX',                  -- pst_type
    'POSTED',                   -- pst_status
    'hash004',                  -- hash
    'pline_002',                -- base_line
    NULL),                      -- discarded_time
    
    -- Fifth posting line (with discarded_time to test filtering)
    ('pline_005',                -- id
    'account_pl_001',           -- account_id
    75.00,                      -- debit_amount
    0.00,                       -- credit_amount
    '2024-01-05 14:00:00',      -- record_time
    'opr_pl_004',               -- opr_id
    'payment_004',              -- opr_src
    '2024-01-05 13:45:00',      -- pst_time
    'BUSI_TX',                  -- pst_type
    'POSTED',                   -- pst_status
    'hash005',                  -- hash
    NULL,                       -- base_line
    '2024-01-06 10:00:00');     -- discarded_time (should be filtered out in tests)