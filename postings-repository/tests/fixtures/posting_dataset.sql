-- tests/fixtures/posting_dataset.sql
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
    'coa_001',                   -- id
    '2024-01-01 10:00:00',      -- created
    'test_user',                -- user_details
    'Test Chart of Account'     -- name
);

-- 2. Then create the ledger record (required by posting)
INSERT INTO ledger (
    id,
    created,
    user_details,
    name,
    coa_id
) VALUES (
    'ledger_001',                -- id
    '2024-01-01 10:00:00',      -- created
    'test_user',                -- user_details
    'Test Ledger',              -- name
    'coa_001'                   -- coa_id (references chart_of_account.id)
);

-- 3. Create multiple posting records with different record times
INSERT INTO posting (
    id,
    record_user,
    record_time,
    opr_id,
    pst_time,
    pst_type,
    pst_status,
    ledger_id
) VALUES 
    -- This should be the one returned (most recent record_time)
    ('pst_001',                    -- id
    'test_user',                   -- record_user
    '2024-01-01 23:59:59',        -- record_time (latest)
    'opr_001',                    -- opr_id
    '2024-01-01 10:00:00',        -- pst_time
    'BUSI_TX',                    -- pst_type
    'POSTED',                     -- pst_status
    'ledger_001'),                -- ledger_id
    
    ('pst_002',                    -- id
    'test_user',                   -- record_user
    '2024-01-01 12:00:00',        -- record_time (earlier)
    'opr_002',                    -- opr_id
    '2024-01-01 10:00:00',        -- pst_time
    'BUSI_TX',                    -- pst_type
    'POSTED',                     -- pst_status
    'ledger_001'),                -- ledger_id
    
    ('pst_003',                    -- id
    'test_user',                   -- record_user
    '2024-01-01 08:00:00',        -- record_time (earliest)
    'opr_003',                    -- opr_id
    '2024-01-01 10:00:00',        -- pst_time
    'BUSI_TX',                    -- pst_type
    'POSTED',                     -- pst_status
    'ledger_001');                -- ledger_id 