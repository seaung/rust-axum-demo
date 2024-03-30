use futures::executor::block_on;
use sea_orm::{ConnectionTrait, Database, DbErr, DbBackend, Statement};

const DATABASE_URI: &str = "mysql://root:7oty\\@7#667989@localhost:3306";
const DB_NAME: &str = "bseaorm";

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URI).await?;

    let db = &match db.get_database_backend() {
        DbBackend::MySql => {
            db.execute(
                Statement::from_string(db.get_database_backend(), 
                format!("CREATE DATABASE IF NOT EXISTS `{}`;", DB_NAME),
            )).await?;
            let url = format!("{}/{}", DATABASE_URI, DB_NAME);
            Database::connect(&url).await?
        }

        DbBackend::Postgres => {
            db.execute(
                Statement::from_string(db.get_database_backend(), 
                format!("DROP DATABASE IF EXISTS \"{}\";", DB_NAME),
            )).await?;

            db.execute(Statement::from_string(
               db.get_database_backend(),
               format!("CREATE DATABASE \"{}\";", DB_NAME),
           )).await?;

            let url = format!("{}/{}", DATABASE_URI, DB_NAME);
            Database::connect(&url).await?
        }

        DbBackend::Sqlite => db
    };
    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err)
    }
}
