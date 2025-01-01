use std::error::Error;

use crate::{models::database, views::cli};

async fn handle(matches : clap::ArgMatches<'_>) -> Result<(), Box<dyn Error>> {
    let db_type : &str      = matches.value_of("db").unwrap();
    let host : &str         = matches.value_of("host").unwrap_or("localhost");
    let db_user : &str      = matches.value_of("user").unwrap();
    let db_password : &str  = matches.value_of("password").unwrap();
    let db_name : &str      = matches.value_of("dbname").unwrap();

    //Connection to the Database
    let cliente= database::connect(db_type, host, db_user, db_password, db_name).await?;
    //Backup
    database::backup(&cliente).await?;

    cli::print_success("Backup completed successfully!");
    Ok(())
}