use actix_web::{HttpResponse, web};
use handlebars::Handlebars;
use log::debug;
use crate::db::db_service::DbPool;

pub async fn login_handler(hb: web::Data<Handlebars<'_>>, pool: web::Data<DbPool>) -> HttpResponse {
    debug!("{}","post to login_form");
    HttpResponse::Ok().finish()
}