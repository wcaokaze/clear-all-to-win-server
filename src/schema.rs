table! {
    gamerecords (id) {
        id -> Int8,
        player_name -> Nullable<Text>,
        start_time -> Text,
        initial_field_width -> Int2,
        initial_field_height -> Int2,
        initial_field -> Array<Bool>,
        rule -> Array<Bool>,
        steps -> Array<Text>,
    }
}
