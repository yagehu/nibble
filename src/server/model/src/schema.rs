// @generated automatically by Diesel CLI.

diesel::table! {
    ingestor (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        name -> Text,
        image_name -> Text,
        image_tag -> Text,
    }
}
