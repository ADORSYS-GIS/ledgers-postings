-- Create the chart of account entry
INSERT INTO chart_of_account (id, name, created, user_details)
VALUES ('coa_test_001', 'Test CoA', NOW(), 'test_user');

-- Create the ledger (check column names in schema)
INSERT INTO ledger (id, name, coa_id, created, user_details)
VALUES ('ldgr_test_001', 'Test Ledger', 'coa_test_001', NOW(), 'test_user');

-- Create ledger statements
INSERT INTO ledger_stmt (
    id, ledger_id, pst_time, stmt_seq_nbr, stmt_status, 
    created, user_details
) VALUES (
    'ldgr_stmt_001', 'ldgr_test_001', '2023-01-15 12:00:00', 
    1, 'SIMULATED', NOW(), 'test_user'
);

-- Add another ledger statement with same time but higher seq_nbr
INSERT INTO ledger_stmt (
    id, ledger_id, pst_time, stmt_seq_nbr, stmt_status, 
    created, user_details
) VALUES (
    'ldgr_stmt_002', 'ldgr_test_001', '2023-01-15 12:00:00', 
    2, 'SIMULATED', NOW(), 'test_user'
);

-- Add an earlier ledger statement
INSERT INTO ledger_stmt (
    id, ledger_id, pst_time, stmt_seq_nbr, stmt_status, 
    created, user_details
) VALUES (
    'ldgr_stmt_003', 'ldgr_test_001', '2023-01-10 10:00:00', 
    1, 'SIMULATED', NOW(), 'test_user'
);

-- Add a statement with CLOSED status
INSERT INTO ledger_stmt (
    id, ledger_id, pst_time, stmt_seq_nbr, stmt_status, 
    created, user_details
) VALUES (
    'ldgr_stmt_004', 'ldgr_test_001', '2023-01-05 09:00:00', 
    1, 'CLOSED', NOW(), 'test_user'
);

-- Add a later statement 
INSERT INTO ledger_stmt (
    id, ledger_id, pst_time, stmt_seq_nbr, stmt_status, 
    created, user_details
) VALUES (
    'ldgr_stmt_005', 'ldgr_test_001', '2023-01-20 09:00:00', 
    1, 'SIMULATED', NOW(), 'test_user'
); 