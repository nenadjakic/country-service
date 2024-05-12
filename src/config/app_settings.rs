use envconfig::Envconfig;

#[derive(Envconfig, Debug)]
pub struct AppSettings {
    #[envconfig(from = "DATABASE_URL")]
    database_url: String,

    #[envconfig(from = "SERVER_PORT", default = "8080")]
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
