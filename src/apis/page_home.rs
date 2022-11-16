use std::collections::btree_map::BTreeMap;
use actix_web::{HttpResponse, web};
use handlebars::Handlebars;
use log::debug;
use crate::db::db_service::DbPool;

pub async fn home(hb: web::Data<Handlebars<'_>>,pool: web::Data<DbPool>) -> HttpResponse {
    debug!("{}","home");
    let mut data :BTreeMap<String,String> = BTreeMap::new();
    data.insert("APP_COPY_RIGHT".to_string(),"API Key Generator 2023".to_string());
    let body = hb.render("login-basic", &data).unwrap();
    HttpResponse::Ok().body(body)
}