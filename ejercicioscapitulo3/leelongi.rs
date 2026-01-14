// fichero: leelongi.rs
// Este ejercicio lee una cadena desde teclado, muestra su longitud y la concatena con ".txt"
use std::io::{self, Write};
use std::process::ExitCode;
fn main() -> ExitCode {
    loop {
        let mut cadena = String::new();
        print!("Introduzca cadena (o solo Enter para terminar): \n");
        io::stdout().flush().unwrap();
        if io::stdin().read_line(&mut cadena).is_err() {
            eprintln!("Error al leer la entrada.");
            return ExitCode::FAILURE;
        }
        let cadena = cadena.trim();
        if cadena.is_empty() {
            break;
        }
        let longitud = cadena.chars().count();
        println!("La cadena contiene {} caracteres.", longitud);
        let concatenada = format!("{}.txt", cadena);
        println!("Concatenación: {}", concatenada);
    }
    ExitCode::SUCCESS
}
