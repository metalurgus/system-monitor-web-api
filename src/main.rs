mod system_info;

use actix_web::{get, App, HttpServer, Responder, HttpResponse};
use system_info::system_info::SystemInfo;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Port to bind the server to
    #[arg(short, long, default_value_t = 9999)]
    port: u16,
}

#[get("/")]
async fn info() -> impl Responder {
    let info = SystemInfo::new();
    HttpResponse::Ok().json(info)
}

#[get("/network")]
async fn network() -> impl Responder {
    let network_info = SystemInfo::new().networks;
    HttpResponse::Ok().json(network_info)
}

#[get("/cpu")]
async fn cpu() -> impl Responder {
    let cpu_info = SystemInfo::new().cpu;
    HttpResponse::Ok().json(cpu_info)
}
#[get("/memory")]
async fn memory() -> impl Responder {
    let memory_info = SystemInfo::new().memory;
    HttpResponse::Ok().json(memory_info)
}
#[get("/swap")]
async fn swap() -> impl Responder {
    let swap_info = SystemInfo::new().swap;
    HttpResponse::Ok().json(swap_info)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    HttpServer::new(|| {
        App::new()
            .service(info)
            .service(network)
            .service(cpu)
            .service(memory)
            .service(swap)
    })
    .bind(("0.0.0.0", args.port))?
    .run()
    .await
}
