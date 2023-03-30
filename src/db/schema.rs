// @generated automatically by Diesel CLI.

diesel::table! {
    contacts (name) {
        name -> Text,
        phone_no -> Int8,
        email -> Text,
    }
}
