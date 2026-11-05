use db::crud::{create_table, insert_app};
use db::utils::get_db;

use dotenv;
use errors::AppError;
use tokio;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv::dotenv().ok();

    let db = get_db().await?;
    let conn = db.connect()?;

    let _ = create_table(&conn).await?;
    let response = insert_app("MyApp".to_string(), &conn).await?;

    println!("{:?}", response);

    Ok(())
}
