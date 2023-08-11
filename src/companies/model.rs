use crate::{db, error_handler::CustomError};
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = companies)]
pub struct Company {
    pub name: String,
}

#[derive(Serialize, Deserialize, Insertable, Queryable)]
#[diesel(table_name = companies)]
pub struct Companies {
    pub id: i32,
    pub name: String,
}

impl Companies {
    pub fn create(name: String) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let company = Company { name: name };
        let company = diesel::insert_into(companies::table)
            .values(company)
            .get_result(&conn)?;
        Ok(company)
    }
}
