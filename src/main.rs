mod models;
mod apis;

use actix_web::{web, App, HttpServer, Responder};
use env_logger::Env;

async fn index() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let instrumentation_key = std::env::var("INSTRUMENTATION_KEY").unwrap_or("".to_string());
    if !instrumentation_key.is_empty() {
        let instrumentation_endpoint =
            std::env::var("INSTRUMENTATION_ENDPOINT").unwrap_or("https://southeastasia-1.in.applicationinsights.azure.com".to_string());

         let tracer = opentelemetry_application_insights::new_pipeline(instrumentation_key)
            .with_client(reqwest::Client::new())
            .with_endpoint(instrumentation_endpoint.as_str()).unwrap()
            .install_simple();


    }





    HttpServer::new(|| {
        App::new().service(
            // prefixes all resources and routes attached to it...
            web::scope("/app")
                // ...so this handles requests for `GET /app/index.html`
                .route("/index.html", web::get().to(index)),
        )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}