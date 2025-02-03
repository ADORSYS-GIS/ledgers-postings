-- tests/fixtures/cleanup.sql
-- Truncate all tables in the correct order by letting CASCADE handle the dependencies.
TRUNCATE TABLE 
    account_stmt,
    ledger_stmt,
    posting_line,
    posting,
    posting_trace,
    op_note,
    ledger_account,
    ledger,
    chart_of_account
RESTART IDENTITY CASCADE;
