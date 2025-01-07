// Timestamp: 2025-01-01 00:00:00
mod views;
mod controllers;

use views::cli::list_backups;
use controllers::{backup::{create_backup, compress_backup}, restore::restore_backup};

use std::io::Result;
use tokio::runtime::Runtime;
use clap::{Parser,Subcommand};

#[derive(Parser)]
#[command(name = "Database Backup Utility")]
#[command(version = "0.1.0")]
#[command(about = "Backup and restore databases", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    // It creates a Backup File
    Create {
        #[arg(short, long)]
        db_type: String, // "mysql", "postgres", "sqlite"
        #[arg(short, long)]
        uri: String, // URI de conexión
        #[arg(short, long)]
        output: String, // Archivo de destino
    },
    // It list all the Backup Files
    List,
    // It restores the database
    Restore,
    // It comprime a backupt file
    Compress,
}

fn main() -> Result<()> {
    
    let cli = Cli::parse();

    match cli.command {
        Commands::Create { db_type, uri, output } => {
            let rt: Runtime = Runtime::new().unwrap();
            rt.block_on(create_backup(&db_type, &uri, &output)).unwrap();
        }
        Commands::List => {
            let rt: Runtime = Runtime::new().unwrap();
            rt.block_on(list_backups()).unwrap();
        }
        Commands::Restore => {
            let rt: Runtime = Runtime::new().unwrap();
            rt.block_on(restore_backup()).unwrap();
        }
        Commands::Compress => {
            let rt: Runtime = Runtime::new().unwrap();
            rt.block_on(compress_backup()).unwrap();
        }
    }

    Ok(())
}