use actix_web::{web, App, HttpServer, HttpResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Clone, Serialize, Deserialize)]
struct Item {
    name: String,
    quantity: u32,
}

struct Storage {
    items: Mutex<HashMap<u32, Item>>,
    counter: Mutex<u32>,
}

#[derive(Deserialize)]
struct ItemInput {
    name: String,
    quantity: u32,
}

async fn create_item(storage: web::Data<Storage>, body: web::Json<ItemInput>) -> HttpResponse {
    let mut items = storage.items.lock().unwrap();
    let mut counter = storage.counter.lock().unwrap();
    *counter += 1;
    let identifier = *counter;
    items.insert(identifier, Item { name: body.name.clone(), quantity: body.quantity });
    HttpResponse::Created().json(serde_json::json!({"identifier": identifier}))
}

async fn get_item(storage: web::Data<Storage>, path: web::Path<u32>) -> HttpResponse {
    let items = storage.items.lock().unwrap();
    match items.get(&path.into_inner()) {
        Some(item) => HttpResponse::Ok().json(item),
        None => HttpResponse::NotFound().json(serde_json::json!({"error": "not found"})),
    }
}

async fn update_item(storage: web::Data<Storage>, path: web::Path<u32>, body: web::Json<ItemInput>) -> HttpResponse {
    let mut items = storage.items.lock().unwrap();
    let identifier = path.into_inner();
    match items.get_mut(&identifier) {
        Some(item) => {
            item.name = body.name.clone();
            item.quantity = body.quantity;
            HttpResponse::Ok().json(item.clone())
        }
        None => HttpResponse::NotFound().json(serde_json::json!({"error": "not found"})),
    }
}

async fn delete_item(storage: web::Data<Storage>, path: web::Path<u32>) -> HttpResponse {
    let mut items = storage.items.lock().unwrap();
    match items.remove(&path.into_inner()) {
        Some(item) => HttpResponse::Ok().json(item),
        None => HttpResponse::NotFound().json(serde_json::json!({"error": "not found"})),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let storage = web::Data::new(Storage {
        items: Mutex::new(HashMap::new()),
        counter: Mutex::new(0),
    });

    const LOCALHOST: &str = "127.0.0.1";
    const IP: &str = localhost;
    const PORT: u16 = 8080;

    println!("Running on http://{IP}:{PORT}");

    HttpServer::new(move || {
        App::new()
            .app_data(storage.clone())
            .route("/items", web::post().to(create_item))
            .route("/items/{identifier}", web::get().to(get_item))
            .route("/items/{identifier}", web::put().to(update_item))
            .route("/items/{identifier}", web::delete().to(delete_item))
    })
    .bind((IP, PORT))?
    .run()
    .await
}
