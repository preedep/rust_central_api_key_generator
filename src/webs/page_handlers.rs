use std::collections::btree_map::BTreeMap;
use actix_web::{HttpResponse, web};
use handlebars::Handlebars;
use log::debug;
use crate::db::db_service::DbPool;
use serde::{Deserialize};
use crate::webs::web_render::page_index_render;

#[derive(Deserialize)]
pub struct LoginRequest {
    #[serde(rename = "inputEmailAddress")]
    pub input_email_address: String,
    #[serde(rename = "inputPassword")]
    pub input_password: String
}


#[derive(Deserialize)]
pub struct RegisterRequest {
    #[serde(rename = "inputFirstName")]
    pub input_first_name: String,
    #[serde(rename = "inputLastName")]
    pub input_last_name: String,
    #[serde(rename = "inputEmailAddress")]
    pub input_email_address: String,
    #[serde(rename = "inputPassword")]
    pub input_password: String,
    #[serde(rename = "inputConfirmPassword")]
    pub input_confirm_password: String
}



pub async fn home_handler(hb: web::Data<Handlebars<'_>>,pool: web::Data<DbPool>) -> HttpResponse {
    debug!("{}","home");
    let mut data :BTreeMap<String,String> = BTreeMap::new();
    data.insert("APP_COPY_RIGHT".to_string(),"API Key Generator 2023".to_string());
    let body = hb.render("login-basic", &data).unwrap();
    HttpResponse::Ok().body(body)
}

pub async fn register_handler(hb: web::Data<Handlebars<'_>>,pool: web::Data<DbPool>) -> HttpResponse {
    let mut data :BTreeMap<String,String> = BTreeMap::new();
    data.insert("APP_COPY_RIGHT".to_string(),"API Key Generator 2023".to_string());
    let body = hb.render("register-basic", &data).unwrap();
    HttpResponse::Ok().body(body)
}

pub async fn login_handler(hb: web::Data<Handlebars<'_>>,pool: web::Data<DbPool>) -> HttpResponse {
    debug!("{}","home");
    let mut data :BTreeMap<String,String> = BTreeMap::new();
    data.insert("APP_COPY_RIGHT".to_string(),"API Key Generator 2023".to_string());
    let body = hb.render("login-basic", &data).unwrap();
    HttpResponse::Ok().body(body)
}

pub async fn do_login_handler(hb: web::Data<Handlebars<'_>>, pool: web::Data<DbPool>,form: web::Form<LoginRequest>) -> HttpResponse {
    debug!("{}","post to do_login_handler");
    debug!("User Name : {}",form.input_email_address);

    let body = page_index_render(hb,pool).unwrap();
    HttpResponse::Ok().body(body)
}

pub async fn do_register_handler(hb: web::Data<Handlebars<'_>>, pool: web::Data<DbPool>,form: web::Form<RegisterRequest>) -> HttpResponse {
    debug!("{}","post to do_register_handler");

    let mut data :BTreeMap<String,String> = BTreeMap::new();
    data.insert("APP_COPY_RIGHT".to_string(),"API Key Generator 2023".to_string());
    let body = hb.render("login-basic", &data).unwrap();
    HttpResponse::Ok().body(body)
}