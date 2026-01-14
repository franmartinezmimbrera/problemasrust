// fichero: invierte.rs
// Este ejercicio implementa la inversión de una cadena de texto dada

use std::io::{self, Write};
use std::process::ExitCode;

fn main() -> ExitCode {
    const MAX_SIZE: usize = 100;

    print!("Introduce una cadena de texto: ");
    io::stdout().flush().unwrap();

    let mut entrada = String::new();

    if let Err(_) = io::stdin().read_line(&mut entrada) {
        eprintln!("Error al leer la cadena.");
        return ExitCode::FAILURE;
    }

    let cadena = entrada.trim_end();

    if cadena.is_empty() {
        println!("Cadena vacía.");
        return ExitCode::SUCCESS;
    }

    println!("Cadena original: \"{}\"", cadena);
    let mut caracteres: Vec<char> = cadena.chars().collect();
    let len = caracteres.len();

    for i in 0..(len / 2) {
        caracteres.swap(i, len - 1 - i);
    }

    let invertida: String = caracteres.into_iter().collect();
    println!("Cadena invertida: \"{}\"", invertida);

    ExitCode::SUCCESS
}
