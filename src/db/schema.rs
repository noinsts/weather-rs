diesel::table! {
    users (id) {
        id -> Int8,              // BIGINT PRIMARY KEY
        city -> Text,            // TEXT NOT NULL
        language -> Text,        // TEXT NOT NULL
        temperature_unit -> Text, // TEXT NOT NULL
        speed_unit -> Text,       // TEXT NOT NULL
        created_at -> Timestamp, // TIMESTAMP NOT NULL
        updated_at -> Timestamp, // TIMESTAMP NOT NULL
    }
}