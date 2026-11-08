use db::crud::insert_app;
use db::utils::get_db;

use dotenv;
use errors::AppError;
use tokio;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv::dotenv().ok();

    let db = get_db().await?;

    let response = insert_app("MyApp1".to_string(), &db).await?;

    println!("{:?}", response);

    Ok(())
}
