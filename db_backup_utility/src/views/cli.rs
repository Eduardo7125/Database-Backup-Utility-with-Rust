pub fn print_success(message: &str) {
    println!("\x1b[32m[SUCCESS]\x1b[0m {}", message); // Verde
}

pub fn print_error(message: &str) {
    eprintln!("\x1b[31m[ERROR]\x1b[0m {}", message); // Rojo
}
