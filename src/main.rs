mod system_info;

use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use system_info::system_info::SystemInfo;
use clap::Parser;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tokio::time;
use log::info;
use chrono::Utc;
use actix_files::Files;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Port to bind the server to
    #[arg(short, long, default_value_t = 9999)]
    port: u16,
}

type SharedInfo = Arc<RwLock<SystemInfo>>;

#[get("/all")]
async fn full(data: web::Data<SharedInfo>) -> impl Responder {
    let start = Utc::now();
    info!("{} - / called, about to read SystemInfo", start);
    let info = data.read().unwrap();
    let end = Utc::now();
    info!("{} - / called, SystemInfo read (duration: {} ms)", end, (end - start).num_milliseconds());
    HttpResponse::Ok().json(&*info)
}

#[get("/network")]
async fn network(data: web::Data<SharedInfo>) -> impl Responder {
    let start = Utc::now();
    info!("{} - /network called, about to read SystemInfo", start);
    let info = data.read().unwrap();
    let end = Utc::now();
    info!("{} - /network called, SystemInfo read (duration: {} ms)", end, (end - start).num_milliseconds());
    HttpResponse::Ok().json(&info.networks)
}

#[get("/cpu")]
async fn cpu(data: web::Data<SharedInfo>) -> impl Responder {
    let start = Utc::now();
    info!("{} - /cpu called, about to read SystemInfo", start);
    let info = data.read().unwrap();
    let end = Utc::now();
    info!("{} - /cpu called, SystemInfo read (duration: {} ms)", end, (end - start).num_milliseconds());
    HttpResponse::Ok().json(&info.cpu)
}

#[get("/memory")]
async fn memory(data: web::Data<SharedInfo>) -> impl Responder {
    let start = Utc::now();
    info!("{} - /memory called, about to read SystemInfo", start);
    let info = data.read().unwrap();
    let end = Utc::now();
    info!("{} - /memory called, SystemInfo read (duration: {} ms)", end, (end - start).num_milliseconds());
    HttpResponse::Ok().json(&info.memory)
}

#[get("/swap")]
async fn swap(data: web::Data<SharedInfo>) -> impl Responder {
    let start = Utc::now();
    info!("{} - /swap called, about to read SystemInfo", start);
    let info = data.read().unwrap();
    let end = Utc::now();
    info!("{} - /swap called, SystemInfo read (duration: {} ms)", end, (end - start).num_milliseconds());
    HttpResponse::Ok().json(&info.swap)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let args = Args::parse();

    let shared_info = Arc::new(RwLock::new(SystemInfo::new()));

    // Spawn background task to refresh SystemInfo every second
    {
        let shared_info = Arc::clone(&shared_info);
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(1));
            loop {
                interval.tick().await;
                let start = Utc::now();
                info!("{} - Refreshing SystemInfo...", start);

                // Build new SystemInfo outside the lock
                let new_info = SystemInfo::new();

                // Lock only to swap in the new value
                let mut info = shared_info.write().unwrap();
                *info = new_info;

                let end = Utc::now();
                info!("{} - SystemInfo refreshed (duration: {} ms)", end, (end - start).num_milliseconds());
            }
        });
    }

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(shared_info.clone()))
            .service(full)
            .service(network)
            .service(cpu)
            .service(memory)
            .service(swap)
            .service(Files::new("/", "./static").index_file("index.html")) // <-- add this line
    })
    .bind(("0.0.0.0", args.port))?
    .run()
    .await
}
