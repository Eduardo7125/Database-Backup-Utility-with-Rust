use sqlx::{PgPool, Pool, SqlitePool, mysql::MySqlPool};
use std::{fs, process::Command, error::Error, path::Path};
use regex::Regex;

pub async fn check_mysql_connection(uri: &str) -> Result<(), Box<dyn Error>> {
    let _pool: Pool<sqlx::MySql> = match MySqlPool::connect(uri).await {
        Ok(pool) => pool,
        Err(e) => return Err(format!("{}", e).into()),
    };

    println!("Conectado a la base de datos MySQL");

    Ok(())
}

pub async fn check_postgres_connection(uri: &str) -> Result<(), Box<dyn Error>> {
    let _pool: PgPool = match PgPool::connect(uri).await {
        Ok(pool) => pool,
        Err(e) => return Err(format!("{}", e).into()),
    };

    println!("Conectado a la base de datos PostgreSQL");

    Ok(())
}

pub async fn check_sqlite_connection(uri: &str) -> Result<(), Box<dyn Error>> {
    let _pool: SqlitePool = match SqlitePool::connect(uri).await {
        Ok(pool) => pool,
        Err(e) => return Err(format!("{}", e).into()),
    };

    println!("Conectado a la base de datos SQLite");

    Ok(())
}

pub async fn check_directory(backup_dir: &str) -> Result<(), Box<dyn Error>> {
    if !Path::new(backup_dir).exists() {
        fs::create_dir_all(backup_dir)?;
        println!("Directorio 'Backups' creado.")
    }
    Ok(())
}

pub async fn exec_command(dump_command: String) -> Result<(), Box<dyn Error>> {
    let output: std::process::Output = Command::new("bash")
        .arg("-c")
        .arg(dump_command)
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        return Err(format!("Error al ejecutar el comando de backup: {:?}", output).into());
    }

    Ok(())
}

pub fn extract_credentials(uri: &str) -> Result<(String, String), Box<dyn Error>> {
    // Expresión regular para extraer usuario y contraseña de la URI
    let re = Regex::new(r"//([^:]+):([^@]+)@").unwrap();

    if let Some(caps) = re.captures(uri) {
        // Extraer usuario y contraseña
        let user = caps[1].to_string();
        let password = caps[2].to_string();

        Ok((user, password))
    } else {
        Err("No se pudo extraer las credenciales de la URI".into())
    }
}