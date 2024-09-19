// @generated automatically by Diesel CLI.

diesel::table! {
    files (fileid, chunk_index) {
        fileid -> Uuid,
        chunk -> Bytea,
        chunk_index -> Int4,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    files,
    posts,
);
