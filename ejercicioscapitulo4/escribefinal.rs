// fichero escribefinal.rs
// Ejemplo para escribir al final de un fichero de texto

use std::fs::OpenOptions;
use std::io::Write;
use std::process::ExitCode;

fn main() -> ExitCode {
    let archivo_resultado = OpenOptions::new()
        .append(true)
        .create(true)   
        .open("datos.txt");

    let mut archivo = match archivo_resultado {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Error al abrir el archivo datos.txt en modo append");
            return ExitCode::FAILURE;
        }
    };

    if writeln!(archivo, "Esta es la línea AÑADIDA al final (Append)").is_err() {
        eprintln!("Error al escribir en el archivo.");
        return ExitCode::FAILURE;
    }

    println!("\nSe ha añadido una línea al final de 'datos.txt'.");
    ExitCode::SUCCESS
}
