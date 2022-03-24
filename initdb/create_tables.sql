CREATE TABLE bingo (
    id             SERIAL PRIMARY KEY,
    created_time   TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    position       INT,
    problem_id     TEXT,
    contest_id     TEXT,
    title          TEXT,
    difficulty     INT
);
