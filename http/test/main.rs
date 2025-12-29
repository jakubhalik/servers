use actix_web::{get, post, put, delete, web, App, HttpServer, HttpResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use std::env;
use std::net::TcpListener;

type Identifier = u64;

#[derive(Clone, Serialize)]
struct Item {
    name: String,
    quantity: Identifier,
}

struct Storage {
    items: Mutex<HashMap<Identifier, Item>>,
    counter: Mutex<Identifier>,
}

#[derive(Deserialize)]
struct ItemInput {
    name: String,
    quantity: Identifier,
}

#[post("/items")]
async fn create_item(storage: web::Data<Storage>, body: web::Json<ItemInput>) -> HttpResponse {
    let mut items = storage.items.lock().unwrap();
    let mut counter = storage.counter.lock().unwrap();
    *counter += 1;
    let identifier = *counter;
    items.insert(identifier, Item { name: body.name.clone(), quantity: body.quantity });
    HttpResponse::Created().json(serde_json::json!({"identifier": identifier}))
}

#[get("/")]
async fn get_root(storage: web::Data<Storage>) -> HttpResponse {
    let items = storage.items.lock().unwrap();
    HttpResponse::Ok().json(items.clone())
}

#[get("/items")]
async fn get_all_items(storage: web::Data<Storage>) -> HttpResponse {
    let items = storage.items.lock().unwrap();
    HttpResponse::Ok().json(items.clone())
}

#[get("/items/{identifier}")]
async fn get_item(storage: web::Data<Storage>, path: web::Path<Identifier>) -> HttpResponse {
    let items = storage.items.lock().unwrap();
    match items.get(&path.into_inner()) {
        Some(item) => HttpResponse::Ok().json(item),
        None => HttpResponse::NotFound().json(serde_json::json!({"error": "not found"})),
    }
}

#[put("/items/{identifier}")]
async fn update_item(storage: web::Data<Storage>, path: web::Path<Identifier>, body: web::Json<ItemInput>) -> HttpResponse {
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

#[delete("/items/{identifier}")]
async fn delete_item(storage: web::Data<Storage>, path: web::Path<Identifier>) -> HttpResponse {
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
    const IP: &str = LOCALHOST;
    const DEFAULT_PORT: u16 = 8080;

    let mut port: u16 = match env::args().nth(1) {
        Some(arg) => arg.parse().expect("Invalid port number"),
        None => {
            println!("No port argument provided, running with {DEFAULT_PORT}");
            DEFAULT_PORT
        }
    };

    while TcpListener::bind((IP, port)).is_err() {
        println!("Port {} is taken, trying {}", port, port + 1);
        port += 1;
    }

    println!("Running on http://{IP}:{port}");

    HttpServer::new(move || {
        App::new()
            .app_data(storage.clone())
            .service(create_item)
            .service(get_item)
            .service(update_item)
            .service(delete_item)
            .service(get_all_items)
            .service(get_root)
    })
    .bind((IP, port))?
    .run()
    .await
}
