use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use moon::*;
use sqlx::{query, Executor, PgPool};


async fn frontend() -> Frontend {
    Frontend::new().title("New Project")
}

async fn up_msg_handler(req: UpMsgRequest<()>) {
        let UpMsgRequest {
            up_msg,
            ..
        } = req;

        #[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
        #[serde(crate = "serde")]
        struct U{}

        let _down_msg = match up_msg {
            // ------- Auth --------
            UpMsg::Login {
            
            }
        };
}


#[moon::main]
async fn main() -> std::io::Result<()> {
    // Ortam değişkenlerini yükle
    dotenvy::dotenv().ok();
    let conn_str = std::env::var("DATABASE_URL")
        .expect("Env var DATABASE_URL is required for this example.");
    let pool = PgPool::connect(&conn_str).await.expect("Failed to connect to the database");


    // Moon uygulamasını başlat
    start(frontend, up_msg_handler, |_| {}).await
}
