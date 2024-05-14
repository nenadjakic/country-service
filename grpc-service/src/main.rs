use std::{net::SocketAddr};

use dotenv::dotenv;
use envconfig::Envconfig;
use log::info;
use shared::util::{establish_connection, run_pending_migrations};
use tonic::transport::Server;

use crate::{config::app_settings::AppSettings, controller::country_controller::{countries::countries_server::CountriesServer, CountryController}};

mod controller;
mod config;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let app_settings = AppSettings::init_from_env().unwrap();


    info!("🚀🚀🚀 Starting gRPC country-service... 🚀🚀🚀");

    let bind_address: SocketAddr = (String::from("0.0.0.0") + ":" + &app_settings.server_port().to_string()).parse().unwrap();
    
    let db_pool = establish_connection(app_settings.database_url().to_string());
    
    run_pending_migrations(db_pool.get().unwrap());
    info!("🆗 Database migrations run.");

    let grpc_country_service = CountryController::new(db_pool);

    info!("🆗 Listening on port {}.", &app_settings.server_port());
    
    Server::builder()
        .add_service(CountriesServer::new(grpc_country_service))
        .serve(bind_address)
        .await
        .unwrap();           

    info!("🆗 Server stopped.");
}