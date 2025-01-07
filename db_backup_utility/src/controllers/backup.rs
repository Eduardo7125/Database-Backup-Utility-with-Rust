use crate::controllers::tests::{check_mysql_connection, check_postgres_connection, check_sqlite_connection, check_directory ,extract_credentials};

use super::tests::exec_command;

pub async fn create_backup(db_type: &str, uri: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    match db_type {
        "MySql" => backup_mysql(uri, output).await?,
        "Postgres" => backup_postgres(uri, output).await?,
        "Sqlite" => backup_sqlite(uri, output).await?,
        _ => return Err(format!("Base de datos no soportada: {}", db_type).into()),
    }
    Ok(())
}

pub async fn backup_mysql(uri: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    check_mysql_connection(uri).await?;

    let backup_dir: &str = "backups";
    check_directory(backup_dir).await?;

    // Command for creating the backup file
    let backup_path = format!("{},{}", backup_dir, output);

    let (user, password) = extract_credentials(uri)?;
    println!("{}",user);
    println!("{}",password);
    let dump_command : String = format!("mysqldumb --uri={} --result-file={}", uri, backup_path);

    // EXEC
    exec_command(dump_command).await?;
    println!("Backup creado exitosamente: {}", backup_path);

    Ok(())
}

pub async fn backup_postgres(uri: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    check_postgres_connection(uri).await?;

    let backup_dir: &str = "backups";
    check_directory(backup_dir).await?;

    // Command for creating the backup file
    let backup_path = format!("{},{}", backup_dir, output);
    let dump_command: String = format!("pg_dump --no-password --file={} --dbname={}", backup_path, uri);

    // EXEC
    exec_command(dump_command).await?;

    println!("Backup creado exitosamente: {}", backup_path);

    Ok(())
}

pub async fn backup_sqlite(uri: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    check_sqlite_connection(uri).await?;

    let backup_dir: &str = "backups";
    check_directory(backup_dir).await?;

    // Command for creating the backup file
    let backup_path = format!("{},{}", backup_dir, output);
    let dump_command: String = format!("sqlite3 {} .dump > {}", uri, backup_path);

    // EXEC
    exec_command(dump_command).await?;

    println!("Backup creado exitosamente: {}", backup_path);

    Ok(())
}

pub async fn compress_backup() -> Result<(), Box<dyn std::error::Error>> {
    println!("Compress!!");
    Ok(())
}