use uuid::Uuid;
use diesel::{Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::schema::countries;

#[derive(Serialize, Deserialize, Queryable, Selectable)]
#[diesel(primary_key(id))]
#[diesel(table_name = countries)]
pub struct Country {
    #[diesel(deserialize_as = Uuid)]
    pub id: Option<Uuid>,
    pub alpha2_code: String,
    pub name: String
}

impl Country {
    pub fn new(id: Option<Uuid>, alpha2_code: String, name: String) -> Self {
        Self {
            id,
            alpha2_code,
            name,
        }
    }
}