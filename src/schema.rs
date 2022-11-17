// @generated automatically by Diesel CLI.

diesel::table! {
    tbl_api_keys (app_id, env) {
        app_id -> Varchar,
        env -> Varchar,
        kv_api_key_id -> Varchar,
        kv_secret_id -> Varchar,
        created_dt -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    tbl_application_info (app_id) {
        app_id -> Varchar,
        application_name -> Varchar,
        created_dt -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    tbl_user (email) {
        email -> Varchar,
        password -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        team_name -> Varchar,
        created_dt -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(tbl_api_keys -> tbl_application_info (app_id));

diesel::allow_tables_to_appear_in_same_query!(
    tbl_api_keys,
    tbl_application_info,
    tbl_user,
);
