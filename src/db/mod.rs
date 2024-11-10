use serde::Serialize;
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::Row;
use crate::config;

#[derive(std::fmt::Debug, Serialize)]
pub struct Post {
    pub title: String,
    pub path: String,
    pub image_path: String,
    pub date: Option<chrono::NaiveDateTime>
}

#[derive(Serialize)]
pub struct JsonResponse {
    pub posts: Vec<Post>
}

pub fn connection_str() -> String {
    let config = config::read_config().db;
    format!("postgresql://{}:{}@{}:{}/{}", config.db_user, config.db_passwd, config.address, config.port, config.db_name)
}
pub async fn test() -> Result<(), sqlx::Error> {
    println!("Testing connection...");
    let _pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(connection_str().as_str()).await?;
    println!("Connection succeeded!");

    Ok(())
}

pub async fn get_posts() -> Vec<Post> {
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(connection_str().as_str()).await.expect("Connection failed");

    let db_posts = sqlx::query("SELECT * FROM posts ORDER BY date DESC")
        .fetch_all(&pool)
        .await.expect("Failed to retrieve posts");

    parse_posts(db_posts)
}

pub async fn get_latest_posts() -> Vec<Post> {
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(connection_str().as_str()).await.expect("Connection failed");

    let db_posts = sqlx::query("SELECT * FROM posts ORDER BY date DESC LIMIT 3")
        .fetch_all(&pool)
        .await.expect("Failed to retrieve posts");

    parse_posts(db_posts)
}

pub fn parse_posts(db_posts: Vec<PgRow>) -> Vec<Post> {
    let mut posts: Vec<Post> = Vec::new();

    for db_post in db_posts {
        let post = Post {
            title: db_post.get("title"),
            path: db_post.get("path"),
            image_path: db_post.get("image_path"),
            date: db_post.get("date"),
        };

        posts.push(post);
    }

    posts
}

pub async fn create_post(post: Post) {
    println!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(connection_str().as_str()).await.expect("Connection failed");

    println!("Creating post...");
    sqlx::query("INSERT INTO posts (title, path, image_path) VALUES ($1, $2, $3)")
        .bind(&post.title)
        .bind(&post.path)
        .bind(&post.image_path)
        .execute(&pool)
        .await
        .expect("Failed to insert into database");

    println!("Success!");
}