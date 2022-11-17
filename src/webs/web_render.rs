use std::fmt::Error;
use actix_web::web;
use handlebars::Handlebars;
use crate::db::db_service::DbPool;

fn page_index_render(hb: web::Data<Handlebars<'_>>, pool: web::Data<DbPool>) -> Result<(),Error> {

    Ok(())
}