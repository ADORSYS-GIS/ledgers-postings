-- Seed data for chart_of_account (required by ledger foreign key)
INSERT INTO chart_of_account (id, created, user_details, name) VALUES
('coa_ldg_test_001', '2024-01-01 10:00:00', 'test_user', 'Test Chart for Ledger Tests');

-- Seed data for ledger table
INSERT INTO ledger (id, created, user_details, name, coa_id) VALUES
('ledger_test_001', '2024-01-01 11:00:00', 'test_user', 'Test Ledger Alpha', 'coa_ldg_test_001'),
('ledger_test_002', '2024-01-01 11:05:00', 'test_user', 'Test Ledger Beta', 'coa_ldg_test_001'),
('ledger_test_003', '2024-01-01 11:10:00', 'test_user', 'Another Test Ledger', 'coa_ldg_test_001'); 