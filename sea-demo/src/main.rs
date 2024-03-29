use futures::executor::block_on;
use sea_orm::{Database, DbErr};

const DATABASE_URI: &str = "mysql://root:7oty\\@7#667989@localhost:3306";
const DB_NAME: &str = "bseaorm";

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URI).await?;
    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err)
    }
}
