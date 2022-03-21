CREATE TABLE bingo (
    id             SERIAL PRIMARY KEY,
    created_date   TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    level          INT,
    position       INT,
    problem_id     TEXT,
    contest_id     TEXT,
    title          TEXT,
    difficulty     INT
);