use database::DB;
use serde::{Serialize, Deserialize};
use moon::*;
use sqlx::PgPool;

use shared::*;

mod database;

async fn frontend() -> Frontend {
    Frontend::new().title("New Project")
}
async fn up_msg_handler(req: UpMsgRequest<UpMsg>) {
    let UpMsgRequest {
        up_msg,
        ..
    } = req;

    #[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
    #[serde(crate = "serde")]
    struct U{id: i32}

    let _down_msg = match up_msg {
        // ------- Auth --------
        UpMsg::Login { email, password} => {
            let user: sqlx::Result<U> = sqlx::query_as(r#"select id from users"#)
                .bind(&email)
                .fetch_one(&*DB.read().await)
                .await;

            if let Ok(u) = user {
                let user = User {
                    id: EntityId::new(),
                    name: email,
                    auth_token: AuthToken::new("i'm auth token")
                };

                DownMsg::LoggedIn(user)
            } else {
                DownMsg::LoginError("sorry, invalid name or password".to_owned())
            }
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
