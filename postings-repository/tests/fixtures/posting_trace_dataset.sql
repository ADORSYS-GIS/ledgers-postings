-- First ensure the chart of account exists
INSERT INTO chart_of_account (id, name, created, user_details)
VALUES ('coa_001', 'Test CoA', NOW(), 'test_user');

-- Then ensure the referenced ledger exists
INSERT INTO ledger (id, name, coa_id, created, user_details)
VALUES ('ledger_001', 'Test Ledger', 'coa_001', NOW(), 'test_user');

-- Create the ledger account records next
INSERT INTO ledger_account (
    id, name, ledger_id, coa_id, balance_side, category,
    created, user_details
) VALUES (
    'account_pt_001', 'Test Account 1', 'ledger_001', 'coa_001', 
    'Dr', 'AS', NOW(), 'test_user'
);

INSERT INTO ledger_account (
    id, name, ledger_id, coa_id, balance_side, category,
    created, user_details
) VALUES (
    'account_pt_002', 'Test Account 2', 'ledger_001', 'coa_001', 
    'Cr', 'LI', NOW(), 'test_user'
);

INSERT INTO ledger_account (
    id, name, ledger_id, coa_id, balance_side, category,
    created, user_details
) VALUES (
    'account_pt_003', 'Test Account 3', 'ledger_001', 'coa_001', 
    'Dr', 'AS', NOW(), 'test_user'
);

-- Now create the posting traces
-- Basic trace with minimal values
INSERT INTO posting_trace (
    id, tgt_pst_id, src_pst_id, src_pst_time, src_opr_id,
    account_id, debit_amount, credit_amount, src_pst_hash
) VALUES (
    'post_trace_001', 'target_pst_001', 'source_pst_001', '2023-01-15 10:00:00',
    'source_opr_001', 'account_pt_001', 100.00, 0.00, 'hash001'
);

-- Trace with credit instead of debit
INSERT INTO posting_trace (
    id, tgt_pst_id, src_pst_id, src_pst_time, src_opr_id,
    account_id, debit_amount, credit_amount, src_pst_hash
) VALUES (
    'post_trace_002', 'target_pst_002', 'source_pst_002', '2023-02-20 11:30:00',
    'source_opr_002', 'account_pt_002', 0.00, 150.00, 'hash002'
);

-- Trace with missing optional values
INSERT INTO posting_trace (
    id, tgt_pst_id, src_pst_id, src_pst_time, src_opr_id,
    account_id, debit_amount, credit_amount, src_pst_hash
) VALUES (
    'post_trace_003', 'target_pst_003', 'source_pst_003', NULL,
    'source_opr_003', 'account_pt_003', 75.00, 0.00, NULL
); 