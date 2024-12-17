use blog_rs::{
    config::get_config, domain::blog::service::Service, inbound::http::http_server::HttpServer,
    logger, outbound::db::postgres::Pg,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = get_config()?;
    logger::init(&config.logger);
    let pg = Pg::new(config.database).await?;
    let blog_service = Service::new(pg);
    let http_server = HttpServer::new(blog_service, config.application).await?;
    http_server.run().await
}
