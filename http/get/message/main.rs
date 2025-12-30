use actix_web::{get, web, App, HttpServer, HttpResponse};
use std::env;
use std::net::TcpListener;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

struct State {
    text: String,
    debug: bool,
    request_count: AtomicU64,
    start_time: Instant,
}

#[get("/")]
async fn index(state: web::Data<State>) -> HttpResponse {
    if state.debug {
        let count = state.request_count.fetch_add(1, Ordering::Relaxed) + 1;
        let uptime = state.start_time.elapsed().as_secs();
        println!("Request #{count} | Uptime: {uptime}s");
    }
    HttpResponse::Ok().body(state.text.clone())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    const LOCALHOST: &str = "127.0.0.1";
    const IP: &str = LOCALHOST;
    const DEFAULT_PORT: u16 = 8080;

    let args: Vec<String> = env::args().collect();

    let debug_flags = [
        "-d", "-debug", "--debug", "-m", "-monitor", "--monitor"
    ];
    let debug = args.iter().any(|arg| debug_flags.contains(&arg.as_str()));

    let args_filtered: Vec<String> = args.iter().filter(|arg| !debug_flags.contains(&arg.as_str())).cloned().collect();

    let message = args_filtered.get(1).cloned().unwrap_or_else(|| {
        println!("No message provided, using 'Hello, World!'");
        "Hello, World!".to_string()
    });

    let mut port: u16 = args_filtered
        .get(2)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or_else(|| {
            println!("No port provided, using {DEFAULT_PORT}");
            DEFAULT_PORT
        });

    while TcpListener::bind((IP, port)).is_err() {
        println!("Port {} is taken, trying {}", port, port + 1);
        port += 1;
    }

    if debug {
        println!("Debug mode enabled");
    }
    println!("Running on http://{IP}:{port}");
    println!("Message: {message}");

    let state = web::Data::new(State {
        text: message,
        debug,
        request_count: AtomicU64::new(0),
        start_time: Instant::now(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(index)
    })
    .bind((IP, port))?
    .run()
    .await
}
