// Archivo: passgen.rs
// Descripción: Generador de contraseñas seguras usando el crate 'rand'.
// Dependencias: Añadir rand = "*" en Cargo.toml

use rand::{Rng, rng};
use rand::distr::Alphanumeric;
use std::process::ExitCode;

fn generar_password(longitud: usize) -> String {
    let rng = rng();
    
    rng
        .sample_iter(&Alphanumeric)
        .take(longitud)
        .map(char::from)
        .collect()
}

// Versión alternativa con caracteres especiales (más robusta)
fn generar_password_compleja(longitud: usize) -> String {
    let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                   abcdefghijklmnopqrstuvwxyz\
                   0123456789\
                   )(*&^%$#@!~";
    let mut rng = rng();
    
    (0..longitud)
        .map(|_| {
            let idx = rng.random_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect()
}

fn main() -> ExitCode {
    let len = 12;
    println!("Generando contraseñas seguras (longitud {}):", len);
    
    println!("Alfanumérica: {}", generar_password(len));
    println!("Compleja:     {}", generar_password_compleja(len));
    ExitCode::SUCCESS
}
