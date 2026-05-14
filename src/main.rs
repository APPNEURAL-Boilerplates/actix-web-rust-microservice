use actix_web::{web, App, HttpServer};
use tracing_actix_web::TracingLogger;

use actix_web_microservice_boilerplate::{
    app::{build_cors, configure, json_config},
    common::request_id::RequestId,
    config::Settings,
    logging::init_logging,
    state::AppState,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = Settings::from_env().expect("failed to load application settings");
    init_logging(&settings.log_level);

    let bind_address = (settings.host.clone(), settings.port);
    let state = web::Data::new(AppState::new(settings.clone()));

    tracing::info!(
        service = %settings.service_name,
        environment = %settings.environment,
        host = %settings.host,
        port = settings.port,
        "starting Actix Web microservice"
    );

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .app_data(json_config())
            .wrap(RequestId)
            .wrap(TracingLogger::default())
            .wrap(build_cors(&settings))
            .configure(configure)
    })
    .bind(bind_address)?
    .run()
    .await
}
