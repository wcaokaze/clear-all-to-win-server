CREATE TABLE gamerecords (
    id SERIAL8 PRIMARY KEY,
    player_name TEXT,
    start_time TEXT NOT NULL,
    initial_field_width SMALLINT NOT NULL,
    initial_field_height SMALLINT NOT NULL,
    initial_field BOOLEAN[] NOT NULL,
    rule BOOLEAN[9] NOT NULL,
    steps TEXT[] NOT NULL
);
