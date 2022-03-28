use std::time::Duration;
use sqlx::Executor;
use sqlx::mysql::MySqlPoolOptions;

#[tokio::main]
pub async fn main()-> Result<(),sqlx::Error> {
    println!("aaa");
    let db_url = "mysql://cf_user:Cf2021%26%23@localhost:3306/cf";

    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .min_connections(1)
        .idle_timeout(Duration::from_secs(60*30))
        .after_connect(|conn|Box::pin(
            async move {
                let sql = format!("set time_zone = '{}'","+8:00");

                conn.execute(sql.as_str()).await?;
                println!("set end.");
                Ok(())
            }
        ))
        .connect(db_url).await?;

    Ok(())

}