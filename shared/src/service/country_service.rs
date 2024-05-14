use uuid::Uuid;

use crate::{entity::country::Country, repository::country_repository::CountryRepository};

pub struct CountryService {
    repository: CountryRepository
}

impl CountryService {
    pub fn new(repository: CountryRepository) -> Self {
        Self { repository }
    }

    pub fn find_by_id(&self, id: Uuid) -> Option<Country> {
        self.repository
            .find_by_id(id)
    }

    pub fn find_by_alpha2_code(&self, alpha2_code: String) -> Option<Country> {
        self.repository
            .find_by_alpha2_code(alpha2_code)
    }

    pub fn find_by_name(&self, other_name: String) -> Vec<Country> {
        self.repository
            .find_by_name(other_name)
    }

    pub fn find_all(&self) -> Vec<Country> {
        self.repository.find_all()
    }
}