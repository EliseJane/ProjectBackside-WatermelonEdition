table! {
    image (id) {
        id -> Uuid,
        file_name -> Text,
        file_size -> Int4,
        data -> Bytea,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        owner -> Text,
    }
}

table! {
    user (id) {
        id -> Uuid,
        user_name -> Text,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    image,
    user,
);
