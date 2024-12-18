mod config;
mod trader_spi;
mod types;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let app_config = config::get_config();

    trader_spi::run_td(app_config.ctp, app_config.libs);
}
