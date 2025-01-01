use tokio_postgres::{Client, NoTls, Error};
use std::process::{Command, Stdio};

pub async fn backup(client: &Client, db_name: &str, host: &str, backup_path: &str, password: &str) -> Result<(), io::Error> {
    // Configurar la variable de entorno PGPASSWORD con la contraseña del usuario
    env::set_var("PGPASSWORD", password);

    // Construir el comando pg_dump para hacer el backup
    let dump_command = format!(
        "pg_dump -h {} -U postgres -d {} -F c -b -v -f {}",
        host, db_name, backup_path
    );

    // Ejecutar pg_dump como un proceso externo
    let status = Command::new("sh")
        .arg("-c")
        .arg(dump_command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .status()?;

    // Limpiar la variable de entorno para no dejar información sensible en el ambiente
    env::remove_var("PGPASSWORD");

    if status.success() {
        println!("Backup completed successfully");
    } else {
        eprintln!("Backup failed with status: {}", status);
    }

    Ok(())
}