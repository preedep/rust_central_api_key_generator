use std::collections::btree_map::BTreeMap;
use std::fmt::Error;
use actix_web::web;
use handlebars::Handlebars;
use crate::db::db_service::DbPool;

pub fn page_index_render(hb: web::Data<Handlebars<'_>>, pool: web::Data<DbPool>) -> Result<String,Error> {
    let mut data :BTreeMap<String,String> = BTreeMap::new();
    let body = hb.render("index", &data).unwrap();
    Ok(body)
}