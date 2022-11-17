use std::collections::btree_map::BTreeMap;
use actix_web::{HttpResponse, web};
use handlebars::Handlebars;
use log::debug;
use crate::db::db_service::DbPool;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    #[serde(rename = "inputEmailAddress")]
    pub input_email_address: String,
    #[serde(rename = "inputPassword")]
    pub input_password: String
}

pub async fn home_handler(hb: web::Data<Handlebars<'_>>,pool: web::Data<DbPool>) -> HttpResponse {
    debug!("{}","home");
    let mut data :BTreeMap<String,String> = BTreeMap::new();
    data.insert("APP_COPY_RIGHT".to_string(),"API Key Generator 2023".to_string());
    let body = hb.render("login-basic", &data).unwrap();
    HttpResponse::Ok().body(body)
}

pub async fn login_handler(hb: web::Data<Handlebars<'_>>, pool: web::Data<DbPool>,form: web::Form<LoginRequest>) -> HttpResponse {
    debug!("{}","post to login_form");
    debug!("User Name : {}",form.input_email_address);



    HttpResponse::Ok().finish()
}
