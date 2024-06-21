// @generated automatically by Diesel CLI.

diesel::table! {
    products (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
    }
}
