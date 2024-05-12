use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::Pool;

use crate::{
    repository::country_repository::CountryRepository, service::country_service::CountryService,
};

pub struct AppState {
    pub country_service: CountryService,
}

impl AppState {
    pub fn new(db_pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        let country_repository = CountryRepository::new(db_pool);

        Self {
            country_service: CountryService::new(country_repository),
        }
    }
}
