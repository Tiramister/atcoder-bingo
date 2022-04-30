CREATE TABLE problems (
    id             SERIAL PRIMARY KEY,
    chosen_date   DATE,
    position       INT,
    problem_id     TEXT,
    contest_id     TEXT,
    title          TEXT,
    difficulty     INT
);

CREATE TABLE user_status (
    user_id          TEXT,
    problem_row_id   INT,
    accepted         BOOLEAN
);
