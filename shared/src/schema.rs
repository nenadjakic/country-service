// @generated automatically by Diesel CLI.

diesel::table! {
    countries (id) {
        id -> Uuid,
        alpha2_code -> Bpchar,
        name -> Varchar,
    }
}
