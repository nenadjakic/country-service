use diesel::{r2d2::ConnectionManager, PgConnection};
use diesel::prelude::*;
use diesel::prelude::QueryDsl;
use r2d2::Pool;
use uuid::Uuid;

use crate::entity::country::Country;
use crate::schema::countries::dsl::*;
use crate::schema::countries::{alpha2_code, name};
pub struct CountryRepository { 
    db_pool: Pool<ConnectionManager<PgConnection>>,
}

impl CountryRepository {
    pub fn new(db_pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { db_pool }
    }

    pub fn find_by_id(&self, other_id: Uuid) -> Option<Country> {
        let db_conn: &mut r2d2::PooledConnection<ConnectionManager<PgConnection>> = &mut self.db_pool.get().unwrap();
        
        countries
            .find(other_id)
            .first::<Country>(db_conn)
            .ok()
            
    }
    pub fn find_by_alpha2_code(&self, other_alpha2_code: String) -> Option<Country> {
        let db_conn = &mut self.db_pool.get().unwrap();

        countries
            .filter(alpha2_code.eq(other_alpha2_code.to_uppercase()))
            .first::<Country>(db_conn)
            .ok()
    }

    pub fn find_by_name(&self, other_name: String) -> Vec<Country> {
        let db_conn = &mut self.db_pool.get().unwrap();

        countries
            .filter(name.ilike(other_name + "%"))
            .load::<Country>(db_conn)
            .unwrap()
    }

    pub fn find_all(&self) -> Vec<Country> {
        let db_conn = &mut self.db_pool.get().unwrap();

        countries.load::<Country>(db_conn).unwrap()
    }
}