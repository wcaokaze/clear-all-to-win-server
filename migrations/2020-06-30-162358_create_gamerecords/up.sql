CREATE TABLE gamerecords (
    id SERIAL PRIMARY KEY,
    player_name CHARACTER(32),
    start_time TIMESTAMP NOT NULL,
    initial_field BOOLEAN[][] NOT NULL,
    rule BOOLEAN[3][3] NOT NULL,
    steps TEXT[]
);
