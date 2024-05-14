use uuid::Uuid;
use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::Pool;
use shared::{repository::country_repository::CountryRepository, service::country_service::CountryService};
use tonic::{Request, Response, Status};

use countries::{ StringRequest, UuidRequest, CountryRequest, CountryReply};
use crate::controller::country_controller::countries::countries_server::Countries;

use self::countries::CountriesReply;

pub mod countries {
    tonic::include_proto!("countries");
}
pub struct CountryController {
    pub country_service: CountryService,
}

impl CountryController {
    pub fn new(db_pool: Pool<ConnectionManager<PgConnection>> ) -> Self {
        let country_repository = CountryRepository::new(db_pool);

        Self {
            country_service: CountryService::new(country_repository),
        }
    }
}

#[tonic::async_trait]
impl Countries for CountryController {
    async fn find_all(&self, _:Request<CountryRequest>) -> Result<Response<CountriesReply>, Status> {

        let countries = self.country_service
            .find_all()
            .into_iter()
            .map(|x|CountryReply {
                id: x.id.unwrap().to_string(),
                aplha2_code: x.alpha2_code,
                name: x.name,
            })        
            .collect::<Vec<_>>();

        Ok(Response::new(CountriesReply {
            countries
        }))
    }

    async fn find_by_id(&self, request:Request<UuidRequest>) -> Result<Response<CountryReply>, Status> {
        let uuid = match Uuid::parse_str(request.into_inner().value.as_str()) {
            Ok(x) => x,
            Err(_) => {
                return Err(Status::invalid_argument("INVALID_ARGUMENT")); 
            }
        };

        match self.country_service
        .find_by_id(uuid) {
            Some(x) => Ok(Response::new(
                CountryReply {
                    id: x.id.unwrap().to_string(),
                    aplha2_code: x.alpha2_code.to_string(),
                    name: x.name.to_string()
                })),
            None => Err(Status::not_found("NOT_FOUND")),
        }
    }

    async fn find_by_alpha2_code(&self, request:Request<StringRequest>) -> Result<Response<CountryReply>, Status> {
        match self.country_service
            .find_by_alpha2_code(request.into_inner().value) {
                Some(x) => Ok(Response::new(
                    CountryReply {
                        id: x.id.unwrap().to_string(),
                        aplha2_code: x.alpha2_code.to_string(),
                        name: x.name.to_string()
                    })),
                None => Err(Status::not_found("NOT_FOUND")),
            }
    }

    async fn find_by_name(&self, request:Request<StringRequest>) -> Result<Response<CountriesReply>, Status> {

        let countries = self.country_service
            .find_by_name(request.into_inner().value)
            .into_iter()
            .map(|x|CountryReply {
                id: x.id.unwrap().to_string(),
                aplha2_code: x.alpha2_code,
                name: x.name,
            })        
            .collect::<Vec<_>>();

        Ok(Response::new(CountriesReply {
            countries
        }))
    }
}