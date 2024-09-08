use dotenvy::dotenv;
use sqlx::{PgConnection, PgExecutor, PgPool, Postgres, Transaction};

mod db;

async fn query(conn: impl PgExecutor<'_>) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
        select true as boolean
        "#
    )
    .fetch_one(conn)
    .await
    .map(|_| ())
}

// async fn call_two_queries(conn: impl PgExecutor<'_>) {
//     query(conn).await.unwrap();

//     //query(conn).await.unwrap();
// }

async fn call_two_queries(conn: &PgPool) {
    query(conn).await.unwrap();

    query(conn).await.unwrap();
}

// async fn call_two_queries(conn: &mut PgConnection) {
//     query(&mut *conn).await.unwrap();

//     query(&mut *conn).await.unwrap();
// }

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = db::prepare_db_and_get_connection().await.unwrap();

    call_two_queries(&*pool).await;
}

#[cfg(test)]
pub mod test {

    use crate::{call_two_queries, db::test};

    #[tokio::test]
    pub async fn test() {
        let mut tx = test::get_test_connection().await.unwrap();

        call_two_queries(&mut *tx).await;
    }
}
