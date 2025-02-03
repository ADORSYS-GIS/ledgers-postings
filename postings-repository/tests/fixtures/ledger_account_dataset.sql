-- tests/fixtures/ledger_account_dataset.sql
-- 
-- Copyright (c) 2018-2024 adorsys GmbH and Co. KG
-- All rights are reserved.
--

-- Insert into chart_of_account
INSERT INTO chart_of_account (id, created, user_details, name, short_desc)
VALUES 
  ('ci8k8PDcTrCsi-F3sT3i-g', '2018-08-07 20:58:24.232', 'Francis', 'IFRS', 'Sample chart of account');

-- Insert into ledger
INSERT INTO ledger (id, created, user_details, name, coa_id)
VALUES 
  ('Zd0ND5YwSzGwIfZilhumPg', '2018-08-07 20:58:24.232', 'Sample User', 'GL', 'ci8k8PDcTrCsi-F3sT3i-g');

-- Insert into ledger_account (multiple rows)
INSERT INTO ledger_account (id, created, user_details, ledger_id, coa_id, balance_side, category, name, short_desc)
VALUES 
  ('xVgaTPMcRty9ik3BTQDh1Q_BS_1_0_0', '2018-08-07 23:50:41.231', 'Sample User', 'Zd0ND5YwSzGwIfZilhumPg', 'ci8k8PDcTrCsi-F3sT3i-g', 'Dr', 'AS', '1.0.0', 'Asset Accounts'),
  ('xVgaTPMcRty9ik3BTQDh1Q_BS_2_0_0', '2018-08-07 23:50:41.231', 'Sample User', 'Zd0ND5YwSzGwIfZilhumPg', 'ci8k8PDcTrCsi-F3sT3i-g', 'Cr', 'EQ', '2.0.0', 'Equity Accounts'),
  ('xVgaTPMcRty9ik3BTQDh1Q_BS_3_0_0', '2018-08-07 23:50:41.231', 'Sample User', 'Zd0ND5YwSzGwIfZilhumPg', 'ci8k8PDcTrCsi-F3sT3i-g', 'Cr', 'LI', '3.0.0', 'Liability Accounts'),
  ('xVgaTPMcRty9ik3BTQDh1Q_PL_4_0_0', '2018-08-07 23:50:41.231', 'Sample User', 'Zd0ND5YwSzGwIfZilhumPg', 'ci8k8PDcTrCsi-F3sT3i-g', 'Cr', 'RE', '4.0.0', 'Revenue Accounts'),
  ('xVgaTPMcRty9ik3BTQDh1Q_PL_5_0_0', '2018-08-07 23:50:41.231', 'Sample User', 'Zd0ND5YwSzGwIfZilhumPg', 'ci8k8PDcTrCsi-F3sT3i-g', 'Dr', 'EX', '5.0.0', 'Operating Expense Accounts'),
  ('xVgaTPMcRty9ik3BTQDh1Q_PL_6_0_0', '2018-08-07 23:50:41.231', 'Sample User', 'Zd0ND5YwSzGwIfZilhumPg', 'ci8k8PDcTrCsi-F3sT3i-g', 'Dr', 'EX', '6.0.0', 'Non-operating income and expense Accounts'),
  ('xVgaTPMcRty9ik3BTQDh1Q_PL_7_0_0', '2018-08-07 23:50:41.231', 'Sample User', 'Zd0ND5YwSzGwIfZilhumPg', 'ci8k8PDcTrCsi-F3sT3i-g', 'Cr', 'LI', '7.0.0', 'Income Tax Accounts');
