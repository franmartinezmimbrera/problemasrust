// fichero creafiche.rs
// Ejemplo para crear/escribir ficheros
use std::fs::File;
use std::io::{Write, BufWriter};
use std::process::ExitCode;
fn main() -> ExitCode {
    let archivo = File::create("datos.txt");
    let archivo = match archivo {
        Ok(f) => BufWriter::new(f),
        Err(_) => {
            eprintln!("Error al abrir/crear el archivo datos.txt");
            return ExitCode::FAILURE;
        }
    };
    let mut writer = archivo;
    if writeln!(writer, "Esta es la primera línea.").is_err() {
        eprintln!("Error al escribir en el archivo.");
        return ExitCode::FAILURE;
    }
    if writeln!(writer, "El número PI es aproximadamente {:.4}", 3.14159).is_err() {
        eprintln!("Error al escribir en el archivo.");
        return ExitCode::FAILURE;
    }
    if writeln!(writer, "Tercera línea de ejemplo.").is_err() {
        eprintln!("Error al escribir en el archivo.");
        return ExitCode::FAILURE;
    }
    println!("El archivo 'datos.txt' fue creado y escrito correctamente.");
    ExitCode::SUCCESS
}
