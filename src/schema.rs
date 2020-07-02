table! {
    gamerecords (id) {
        id -> Int4,
        player_name -> Nullable<Bpchar>,
        start_time -> Timestamp,
        initial_field -> Array<Bool>,
        rule -> Array<Bool>,
        steps -> Nullable<Array<Text>>,
    }
}
