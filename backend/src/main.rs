use std::collections::HashMap;

use moon::*;
use sqlx::{query, Executor, PgPool};

async fn frontend() -> Frontend {
    Frontend::new().title("New Project")
}

async fn up_msg_handler(_: UpMsgRequest<()>) {}

struct Author {
    _id: i32,
    name: String,
    country: String,
}

#[moon::main]
async fn main() -> std::io::Result<()> {
    // Ortam değişkenlerini yükle
    dotenvy::dotenv().ok();
    let conn_str = std::env::var("DATABASE_URL")
        .expect("Env var DATABASE_URL is required for this example.");
    let pool = PgPool::connect(&conn_str).await.expect("Failed to connect to the database");

    // Veritabanı tablolarını oluştur
    pool.execute(
        "
        CREATE TABLE IF NOT EXISTS author (
            id      SERIAL PRIMARY KEY,
            name    VARCHAR NOT NULL,
            country VARCHAR NOT NULL
        );
    ",
    )
    .await
    .expect("Failed to create author table");

    pool.execute(
        "
        CREATE TABLE IF NOT EXISTS book (
            id        SERIAL PRIMARY KEY,
            title     VARCHAR NOT NULL,
            author_id INTEGER NOT NULL REFERENCES author
        );
    ",
    )
    .await
    .expect("Failed to create book table");

    pool.execute(
        "
        CREATE TABLE IF NOT EXISTS defter (
            id        SERIAL PRIMARY KEY,
            title     VARCHAR NOT NULL,
            author_id INTEGER NOT NULL REFERENCES author
        );
    ",
    )
    .await
    .expect("Failed to create defter table");

    // Yazarları ekleme
    let mut authors = HashMap::new();
    authors.insert(String::from("Sadagat Asgarov"), "Azerbaijan");
    authors.insert("Sakhavat Asgarov".to_string(), "Nakhchivan");

    for (name, country) in &authors {
        let author = Author {
            _id: 0,
            name: name.to_string(),
            country: country.to_string(),
        };

        // SQL sorgusunu parametrelerle çalıştır
        query!(
            "INSERT INTO author (name, country) VALUES ($1, $2)",
            author.name,
            author.country
        )
        .execute(&pool)
        .await
        .expect("Failed to insert author");
    }

    // Moon uygulamasını başlat
    start(frontend, up_msg_handler, |_| {}).await
}
