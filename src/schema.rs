// @generated automatically by Diesel CLI.

diesel::table! {
    files (fileid) {
        fileid -> Uuid,
        chunk -> Bytea,
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
