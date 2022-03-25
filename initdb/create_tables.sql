CREATE TABLE bingos (
    id             SERIAL PRIMARY KEY,
    created_date   DATE,
    position       INT,
    problem_id     TEXT,
    contest_id     TEXT,
    title          TEXT,
    difficulty     INT
);

CREATE TABLE user_status (
    user_id          TEXT,
    problem_row_id   INT,
    accepted         BOOLEAN DEFAULT FALSE
);
