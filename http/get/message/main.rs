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
        fn format_request_number(count: u64) -> String {
            let with_spaces = count.to_string()
                .chars()
                .rev()
                .collect::<Vec<_>>()
                .chunks(3)
                .map(|chunk| chunk.iter().collect::<String>())
                .collect::<Vec<_>>()
                .join(" ")
                .chars()
                .rev()
                .collect::<String>();
            let scientific = { 
                let exponent = (count as f64).log10().floor() as i32;
                let coefficient = (count as f64) / 10f64.powi(exponent);
                format!("{:.0}×10^{}", coefficient, exponent)
            };

            format!("{} ({})", with_spaces, scientific)
        }

        fn time_val(metric: &mut u128, prevMetric: &mut u128, divideBy: u128) {
            if *metric == 0 {
                *metric = *prevMetric / divideBy;
            } else {
                *metric += 1;
            }
            *prevMetric = *prevMetric % divideBy;
        }

        let elapsed = state.start_time.elapsed();
        let total_nanos = elapsed.as_nanos();
        let mut secs: u128 = (total_nanos / 1_000_000_000);
        let ms: u128 = ((total_nanos % 1_000_000_000) / 1_000_000);
        let us: u128 = ((total_nanos % 1_000_000) / 1_000);
        let ns: u128 = (total_nanos % 1_000);
        let (mut years, mut months, mut days, mut hours, mut mins): 
            (u128, u128, u128, u128, u128) = 
                 (0, 0, 0, 0, 0);
        if secs >= 60 {
            time_val(&mut mins, &mut secs, 60);
            if mins >= 60 {
                time_val(&mut hours, &mut mins, 60);
                if hours >= 24 {
                    time_val(&mut days, &mut hours, 24);
                    if days >= 30 {
                        time_val(&mut months, &mut days, 30);
                        if months >= 12 {
                            time_val(&mut years, &mut months, 12);
                        }
                    }
                }
            }
        }
        let units = [
            (years, "y"),
            (months, "mo"),
            (days, "d"),
            (hours, "h"),
            (mins, "m"),
            (secs, "s"),
            (ms, "ms"),
            (us, "µs"),
            (ns, "ns"),
        ];
        let uptime_str = units
            .iter()
            .filter_map(|(number, unit)| (*number > 0).then(|| format!("{}{}", number, unit)))
            .collect::<Vec<_>>()
            .join(" ");
        let display = if uptime_str.is_empty() { "" } else { &uptime_str };


        println!("Request #{} | Uptime: {}", format_request_number(count), display);
    }

    HttpResponse::Ok().body(state.text.clone())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    const LOCALHOST: &str = "127.0.0.1";
    const PUBLIC: &str = "0.0.0.0";
    const DEFAULT_PORT: u16 = 8080;

    let args: Vec<String> = env::args().collect();

    let mut port: u16 = args.iter()
        .find(|arg| arg.parse::<u16>().is_ok())
        .and_then(|arg| arg.parse().ok())
        .unwrap_or_else(|| {
            println!("No port provided, using {DEFAULT_PORT}");
            DEFAULT_PORT
        });
    let index_of_port: usize = args.iter()
        .position(|arg| arg == &port.to_string())
        .unwrap_or(usize::MAX);

    let debug_flags = [
        "-d", "-debug", "--debug", "-m", "-monitor", "--monitor"
    ];
    let debug: bool = args.iter().any(|arg| debug_flags.contains(&arg.as_str()));

    let public_flags = [
        "-p", "--public"
    ];
    let public: bool = args.iter().any(|arg| debug_flags.contains(&arg.as_str()));

    let IP: &str = if public { PUBLIC } else { LOCALHOST };

    let message = args.iter()
        .enumerate()
        .skip(1)
        .find(
            |(indx, arg)| *indx != index_of_port 
            && !debug_flags.contains(&arg.as_str())
            && !public_flags.contains(&arg.as_str())
        )
        .map(|(_, arg)| arg.clone())
        .unwrap_or_else(|| {
            println!("No message provided, using 'Hello, World!'");
            "Hello, World!".to_string()
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

