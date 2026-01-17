// Archivo: cesar_cifrar.rs
// Descripción: Implementación del cifrado César desplazando caracteres.

use std::process::ExitCode;

fn cifrar_cesar(texto: &str, desplazamiento: u8) -> String {
    texto
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                // Convertimos a u8 (byte) para operar, 0-25
                let c_u8 = c as u8 - base;
                // Aplicamos la fórmula: (x + n) % 26
                let nuevo_c = (c_u8 + desplazamiento) % 26;
                // Reconstruimos el carácter
                (base + nuevo_c) as char
            } else {
                // Si no es letra (espacio, signo), lo dejamos igual
                c
            }
        })
        .collect()
}

fn main() -> ExitCode {
    println!("--- Cifrado César ---");
    let mensaje = "Hola Rust";
    let desplazamiento = 3;
    
    let cifrado = cifrar_cesar(mensaje, desplazamiento);
    
    println!("Original: {}", mensaje);
    println!("Cifrado (n={}): {}", desplazamiento, cifrado);
    ExitCode::SUCCESS
}