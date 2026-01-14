// fichero leerfiche.rs
// Ejemplo para leer ficheros de texto

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let ruta = Path::new("datos.txt");
    let archivo = match File::open(&ruta) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Error al abrir el archivo datos.txt");
            return ExitCode::FAILURE;
        }
    };
    println!("\nContenido de 'datos.txt':");
    let lector = io::BufReader::new(archivo);
    for linea in lector.lines() {
        match linea {
            Ok(texto) => println!("{}", texto),
            Err(_) => {
                eprintln!("Error al leer una línea del archivo.");
                return ExitCode::FAILURE;
            }
        }
    }

    ExitCode::SUCCESS
}
