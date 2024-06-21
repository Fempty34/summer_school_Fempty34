use actix_web::{web,get,post,delete,put,HttpResponse};
use actix_web::web::Json;
use crate::{models::product::{Product, NewProduct},directory::database::Database};



#[post("/product")]
async fn create_product(db:web::Data<Database>, product: web::Json<NewProduct>) -> HttpResponse {
    let product = db.create_product(product.into_inner());
    match product {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
#[get("/product/{product_id}")]
async fn get_product(db:web::Data<Database>, product_id: web::Path<i32>) -> HttpResponse {
    let product = db.get_product(product_id.into_inner());
    match product {
        Some(product) => HttpResponse::Ok().json(product),
        None => HttpResponse::NotFound().finish(),
    }
}

#[put("/product/{product_id}")]
async fn update_product(db:web::Data<Database>, product: web::Json<Product>, product_id: web::Path<i32>) -> HttpResponse {
    let product = db.update_product(product_id.into_inner(), product.into_inner());
    match product {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/product/{product_id}")]
async fn delete_product(db:web::Data<Database>, product_id: web::Path<i32>) -> HttpResponse {
    let product = db.delete_product(product_id.into_inner());
    match product {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/products")]
async fn get_products(db:web::Data<Database>) -> HttpResponse {
    let products = db.get_products();
    HttpResponse::Ok().json(products)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_product);
    cfg.service(get_product);
    cfg.service(update_product);
    cfg.service(delete_product);
    cfg.service(get_products);
}