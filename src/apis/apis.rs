use std::collections::btree_map::BTreeMap;
use actix_web::{HttpResponse, web};
use handlebars::Handlebars;

pub async fn home(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let mut data :BTreeMap<String,String> = BTreeMap::new();
    let body = hb.render("login-basic", &data).unwrap();
    HttpResponse::Ok().body(body)
}