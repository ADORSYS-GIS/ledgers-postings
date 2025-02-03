-- tests/fixtures/account_stmt_dataset.sql
INSERT INTO chart_of_account (id, created, user_details, name, short_desc)
VALUES 
    ('ci8k8PDcTrCsi-F3sT3i-g', '2018-08-07 20:58:24.232', 'Francis', 'IFRS', 'Sample chart of account');

INSERT INTO ledger (id, created, user_details, name, coa_id)
VALUES 
    ('Zd0ND5YwSzGwIfZilhumPg', '2018-08-07 20:58:24.232', 'Sample User', 'GL', 'ci8k8PDcTrCsi-F3sT3i-g');

INSERT INTO ledger_account (id, created, user_details, ledger_id, coa_id, balance_side, category, name, short_desc)
VALUES 
    ('xVgaTPMcRty9ik3BTQDh1Q_BS_1_0_0', '2018-08-07 23:50:41.231', 'Sample User', 'Zd0ND5YwSzGwIfZilhumPg', 'ci8k8PDcTrCsi-F3sT3i-g', 'Dr', 'AS', '1.0.0', 'Asset Accounts'),
    ('xVgaTPMcRty9ik3BTQDh1Q_BS_2_0_0', '2018-08-07 23:50:41.231', 'Sample User', 'Zd0ND5YwSzGwIfZilhumPg', 'ci8k8PDcTrCsi-F3sT3i-g', 'Cr', 'EQ', '2.0.0', 'Equity Accounts');
