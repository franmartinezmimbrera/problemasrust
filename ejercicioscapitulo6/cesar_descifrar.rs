// Archivo: cesar_descifrar.rs
// Descripción: Descifrado César manejando correctamente los números negativos.
use std::process::ExitCode;

fn descifrar_cesar(texto: &str, desplazamiento: i32) -> String {
    // Usamos i32 para permitir restas que den negativo antes del módulo
    texto
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' } as i32;
                let c_val = c as i32 - base;
                
                // FÓRMULA CLAVE: (x - n).rem_euclid(26)
                // En C++ el operador % puede dar negativo (ej: -3 % 26 = -3).
                // En Rust, rem_euclid garantiza un resultado positivo (0..25).
                let nuevo_val = (c_val - desplazamiento).rem_euclid(26);
                
                (base + nuevo_val) as u8 as char
            } else {
                c
            }
        })
        .collect()
}

fn main() -> ExitCode {
    let cifrado = "Krod Uxvw"; // "Hola Rust" con n=3
    let n = 3;
    
    let descifrado = descifrar_cesar(cifrado, n);
    println!("Cifrado: {}", cifrado);
    println!("Descifrado (n={}): {}", n, descifrado);
    ExitCode::SUCCESS
}
