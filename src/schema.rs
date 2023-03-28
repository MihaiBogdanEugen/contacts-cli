// @generated automatically by Diesel CLI.

diesel::table! {
    contacts (id) {
        id -> Int4,
        name -> Text,
        phone_no -> Int8,
        email -> Text,
    }
}
