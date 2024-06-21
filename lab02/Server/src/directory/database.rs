use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

use crate::models::product::{Product, NewProduct};
use crate::models::schema::products::dsl::*;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Database {
    pub pool: DBPool,
}

impl Database {
    pub fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let result = r2d2::Pool::builder().build(manager).expect("Failed to create pool");

        Database {pool: result}
    }


    pub fn get_products(&self) -> Vec<Product> {
        products
            .load::<Product>(&mut self.pool.get().unwrap())
            .expect("Failed to get products")
    }


    pub fn get_product(&self, find_id: i32) -> Option<Product> {
        products
            .find(find_id)
            .first::<Product>(&mut self.pool.get().unwrap())
            .ok()
    }

    pub fn create_product(&self, product: NewProduct) -> Result<Product, diesel::result::Error> {
        diesel::insert_into(products)
           .values(&product)
           .get_result(&mut self.pool.get().unwrap())
    }

    pub fn delete_product(&self, find_id: i32) -> Result<usize, diesel::result::Error> {
        diesel::delete(products.find(find_id)).execute(&mut self.pool.get().unwrap())
    }

    pub fn update_product(&self, find_id: i32, product: Product) -> Result<Product, diesel::result::Error> {
        diesel::update(products.find(find_id))
           .set(&product)
           .get_result(&mut self.pool.get().unwrap())
    }
}