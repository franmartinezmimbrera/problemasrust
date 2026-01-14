use std::fs::File;
use std::io::{self, BufReader, Read};
use std::process::ExitCode;


fn main() -> ExitCode {
    let archivo = match File::open("datos.txt") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error al abrir archivo datos.txt: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let mut lector = BufReader::new(archivo);
    let mut contenido = String::new();

    if let Err(e) = lector.read_to_string(&mut contenido) {
        eprintln!("Error al leer el archivo: {}", e);
        return ExitCode::FAILURE;
    }

    let contador_caracteres = contenido.chars().count();
    let contador_palabras = contenido.split_whitespace().count();

    println!("Total caracteres leídos: {}", contador_caracteres);
    println!("Total palabras: {}", contador_palabras);

    ExitCode::SUCCESS
}
