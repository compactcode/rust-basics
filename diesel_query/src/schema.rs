table! {
    addresses (id) {
        id -> Int4,
        street_number -> Varchar,
        street_name -> Varchar,
        user_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
    }
}

joinable!(addresses -> users (user_id));

allow_tables_to_appear_in_same_query!(
    addresses,
    users,
);
