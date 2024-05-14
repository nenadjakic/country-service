use envconfig::Envconfig;

#[derive(Envconfig, Debug)]
pub struct AppSettings {
    #[envconfig(from = "DATABASE_URL")]
    database_url: String,

    #[envconfig(from = "GRPC_PORT", default = "50051")]
    server_port: u16,
}

impl AppSettings {
    pub fn database_url(&self) -> &str {
        &self.database_url
    }

    pub fn server_port(&self) -> u16 {
        self.server_port
    }
}