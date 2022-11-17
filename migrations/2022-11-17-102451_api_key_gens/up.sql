-- Your SQL goes here

create table tbl_user
(
    email      varchar(255) not null
        constraint tbl_user_pk
            primary key,
    password   varchar(255) not null,
    first_name varchar(255) not null,
    last_name  varchar(255) not null,
    team_name  varchar(255) not null,
    created_dt timestamp with time zone DEFAULT current_timestamp
);

comment on column tbl_user.email is 'Email is Key';

create table tbl_application_info
(
    app_id           varchar(10)  not null
        constraint tbl_application_info_pk
            primary key,
    application_name varchar(255) not null,
    created_dt       timestamp with time zone DEFAULT current_timestamp
);

create table tbl_api_keys
(
    app_id        varchar(255) not null
        constraint tbl_api_keys_tbl_application_info_null_fk
            references tbl_application_info (app_id),
    env           varchar(10)  not null,
    kv_api_key_id varchar(50)  not null,
    kv_secret_id  varchar(50)  not null,
    created_dt    timestamp with time zone DEFAULT current_timestamp,
    constraint tbl_api_keys_pk
        primary key (app_id, env)
);

comment on table tbl_api_keys is 'Store API for each application id';