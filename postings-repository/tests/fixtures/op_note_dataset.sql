-- Create test operation notes
INSERT INTO op_note (
    id, rec_id, type, content, rec_time, 
    exec_time, premature_exc, repeated_exec, exec_status
) VALUES (
    'op_note_001', 'record_001', 'REMINDER', 'Test reminder note', 
    '2023-01-10 10:00:00', NULL, FALSE, FALSE, NULL
);

-- Note with execution time and status
INSERT INTO op_note (
    id, rec_id, type, content, rec_time, 
    exec_time, premature_exc, repeated_exec, exec_status
) VALUES (
    'op_note_002', 'record_002', 'EXECUTION', 'Test execution note', 
    '2023-02-15 14:30:00', '2023-02-15 15:00:00', FALSE, FALSE, 'COMPLETED'
);

-- Note with premature execution flag
INSERT INTO op_note (
    id, rec_id, type, content, rec_time, 
    exec_time, premature_exc, repeated_exec, exec_status
) VALUES (
    'op_note_003', 'record_003', 'WARNING', 'Test warning note', 
    '2023-03-20 09:15:00', '2023-03-20 09:20:00', TRUE, FALSE, 'COMPLETED'
);

-- Note with repeated execution flag
INSERT INTO op_note (
    id, rec_id, type, content, rec_time, 
    exec_time, premature_exc, repeated_exec, exec_status
) VALUES (
    'op_note_004', 'record_004', 'INFO', 'Test info note', 
    '2023-04-25 16:45:00', '2023-04-25 17:00:00', FALSE, TRUE, 'COMPLETED'
); 