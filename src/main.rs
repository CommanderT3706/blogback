mod config;
mod db;
mod sitemap;

use std::env;
use std::process::exit;
use actix_web::{App, get, HttpResponse, HttpServer, Responder};
use crate::sitemap::generate_sitemap;

#[get("/blog/api/posts")]
async fn posts() -> actix_web::Result<impl Responder> {
    Ok(actix_web::web::Json(db::get_posts().await))
}

async fn serve() -> std::io::Result<()> {
    println!("Starting server...");

    HttpServer::new(|| {
        App::new()
            .service(posts)
    })
        .bind(("127.0.0.1", config::read_config().server.port))?
        .run()
        .await
}

fn help() {
    println!("blogback --help: Shows this page");
    println!("blogback serve: Runs the server using the configuration");
    println!("blogback post <title> <webpage_path> <image_path>: Creates a post and updates the sitemap");
    println!("blogback sitemap: Updates the sitemap");
}

async fn post(args: Vec<String>) {
    if args.len() < 5 {
        println!("Too few arguments");
        println!("blogback post <title> <webpage_path> <image_path>: Creates a post and updates the sitemap");
        return;
    }

    let post = db::Post {
        title: args[2].clone(),
        path: args[3].clone(),
        image_path: args[4].clone(),
        date: None
    };

    db::create_post(post).await;
    generate_sitemap().await;
}

async fn sitemap() {
    generate_sitemap().await;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("No command specified");
        println!("Run blogback --help for all commands");
        return Ok(());
    }

    let command = &args[1];

    match command.as_str() {
        "serve" => serve().await?,
        "db_test" => db::test().await.expect("Failed to connect to DB"),
        "post" => post(args).await,
        "sitemap" => sitemap().await,
        "--help" => help(),
        _ => {
            println!("Invalid command");
            println!("Run blogback --help for all commands");
            return Ok(());
        },
    };

    Ok(())
}
