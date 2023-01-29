use actix_web::{App, HttpServer};
use http_server::logging_middleware::LoggingMiddlewareFactory;
use slog::Logger;

mod http_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let logger = configure_logger();
        App::new()
            .configure(http_server::api::api_config)
            .wrap(LoggingMiddlewareFactory::new(&logger))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn configure_logger() -> Logger {
    use slog::{o, Drain};

    let sync_decorator = slog_term::PlainSyncDecorator::new(std::io::stdout());
    //let term_decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(sync_decorator).build().fuse();

    Logger::root(drain, o!())
}
