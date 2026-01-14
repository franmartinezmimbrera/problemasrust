// archivo: saludo.rs
// Este programa hace un saludo personalizado
use std::io;
use std::process::ExitCode;
fn main() -> ExitCode {
    let mut nombre = String::new();
    println!("¡Hola! ¿Cómo te llamas?");
    if io::stdin().read_line(&mut nombre).is_err() {
        println!("Error al leer el nombre.");
        return ExitCode::FAILURE;
    }
    let nombre = nombre.trim();
    println!("¿Qué tal estás {}?", nombre);
    ExitCode::SUCCESS
}
