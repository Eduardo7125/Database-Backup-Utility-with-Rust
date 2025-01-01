// Timestamp: 2025-01-01 00:00:00
mod models;
mod views;
mod controllers;

use std::error::Error;

use clap::{App, Arg};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Database Backup Utility")
        .version("1.0")
        .author("Eduardo Cristea")
        .about("Backup and restore databases")
        .subcommand(App::new("backup")
            .about("Create a Backup of a database")
            .arg(Arg::new("db").required(true).about("Database type"))
            .arg(Arg::new("host").about("Database host").takes_value(true))
            .arg(Arg::new("user").required(true).about("Database User").takes_value(true))
            .arg(Arg::new("password").required(true).about("Database Password").takes_value(true))
            .arg(Arg::new("dbname").required(true).about("Database Name").takes_value(true))
        )
        .subcommand(App::new("restore")
            .about("Restore a Backup of a database")
            .arg(Arg::new("file").required(true).about("Backup file")))
        .get_matches();

    if let Some(subcommand) = matches.subcommand_name(){
        match subcommand {
            "backup" => controllers::backup::handle(matches).await?,
            "restore" => controllers::restore::handle(matches).await?,
            _ => {
                views::cli::print_error("Unknown command");
                return Err("Unknown command".into());
            }
        }
    }

    Ok(())
}