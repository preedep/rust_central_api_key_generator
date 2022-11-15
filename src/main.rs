mod models;
mod apis;


use actix_web::{web, App, HttpServer, Responder};
use actix_web::middleware::Logger;
use actix_web_opentelemetry::RequestTracing;
use diesel::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use log::info;
use opentelemetry::global::shutdown_tracer_provider;


type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;


async fn index() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let instrumentation_key = std::env::var("INSTRUMENTATION_KEY").unwrap_or("".to_string());
    if !instrumentation_key.is_empty() {
        info!("Use Application Insight");

        let instrumentation_endpoint =
            std::env::var("INSTRUMENTATION_ENDPOINT").unwrap_or("https://southeastasia-1.in.applicationinsights.azure.com".to_string());

         let _tracer = opentelemetry_application_insights::new_pipeline(instrumentation_key)
            .with_client(reqwest::Client::new())
            .with_endpoint(instrumentation_endpoint.as_str()).unwrap()
            .install_simple();
    }
    //////
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL Invalid");
    let manager = ConnectionManager::<MysqlConnection>::new(conn_spec);
    let pool : DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");


    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(RequestTracing::new())
            .service(
            // prefixes all resources and routes attached to it...
            web::scope("/v1")
                // ...so this handles requests for `GET /app/index.html`
                .route("/index.html", web::get().to(index)),
        )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;
    shutdown_tracer_provider();
    Ok(())
}