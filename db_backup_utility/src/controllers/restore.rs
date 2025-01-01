use std::error::Error;

use crate::{models::database, views::cli};

async fn handle(matches : clap::ArgMatches<'_>) -> Result<(), Box<dyn Error>> {
    let db_type : &str = matches.value_of("db").unwrap();
    let host : &str = marches.value_of("host").unwrap_or("localhost");

    //Connection to the Database
    let cliente: ! = database::connect(db_type, host).await?;
    //Backup
    database::backup(&cliente).await?;

    cli::print_success("Backup completed successfully!");
    Ok(())
}