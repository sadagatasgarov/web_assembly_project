use sqlx::postgres::{PgPool, Postgres};
use sqlx::Pool;
use moon::Lazy;
use crate::tokio::sync::RwLock;


pub static DB: Lazy<RwLock<PgPool>> = 
    Lazy::new(|| {
        RwLock::new(
            Pool::<Postgres>::connect_lazy(
                &dotenvy::var("DATABASE_URL")
                        .expect("Databasae url must be set")
            ).expect("msg")
        )
    });