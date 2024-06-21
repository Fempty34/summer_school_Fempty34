use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset};

#[derive(Queryable, Serialize, Deserialize, Debug, Insertable, Clone, AsChangeset)]
#[diesel(table_name = crate::models::schema::products)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Debug, Insertable, Clone)]
#[diesel(table_name = crate::models::schema::products)]
pub struct NewProduct {
    pub name: String,
    pub description: String,
}