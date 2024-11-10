mod config;
mod db;
mod sitemap;

use std::env;
use actix_cors::Cors;
use actix_web::{App, get, http, HttpServer, Responder};
use crate::sitemap::generate_sitemap;

#[get("/blog/api/posts")]
async fn posts() -> actix_web::Result<impl Responder> {
    Ok(actix_web::web::Json(db::get_posts().await))
}

#[get("/blog/api/latest_posts")]
async fn latest_posts() -> actix_web::Result<impl Responder> {
    Ok(actix_web::web::Json(db::get_latest_posts().await))
}

async fn serve()  {
    println!("Starting server...");

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:63342")
                    .allowed_methods(vec!["GET"])
                    .allowed_headers(vec![http::header::CONTENT_TYPE])
                    .supports_credentials()
                    .max_age(3600)
            )
            .service(posts)
            .service(latest_posts)
    })
        .bind(("127.0.0.1", config::read_config().server.port))
        .expect("Failed to bind port")
        .run()
        .await
        .expect("Failed to start server");

    println!("Server started on port {}!", config::read_config().server.port);
}

fn help() {
    println!("blogback --help: Shows this page");
    println!("blogback test: Tests the database");
    println!("blogback serve: Runs the server using the configuration");
    println!("blogback post <title> <description> <webpage_path> <image_path>: Creates a post and updates the sitemap");
    println!("blogback sitemap: Updates the sitemap");
}

async fn post(args: Vec<String>) {
    if args.len() < 6 {
        println!("Too few arguments");
        println!("blogback post <title> <description> <webpage_path> <image_path>: Creates a post and updates the sitemap");
        return;
    }

    let post = db::Post {
        title: args[2].clone(),
        description: args[3].clone(),
        path: args[4].clone(),
        image_path: args[5].clone(),
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
        "serve" => serve().await,
        "test" => db::test().await.expect("Failed to connect to DB"),
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
