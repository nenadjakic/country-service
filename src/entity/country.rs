use crate::schema::countries;

use uuid::Uuid;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Selectable, Insertable)]
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