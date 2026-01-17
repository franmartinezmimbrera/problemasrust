// Archivo: hash_simple.rs
// Descripción: Implementación de un hash simple (tipo K&R) para verificar integridad.
use std::process::ExitCode;

// Algoritmo simple: hash = hash * 31 + c
fn generar_hash(texto: &str) -> u64 {
    let mut hash: u64 = 0;
    
    for c in texto.bytes() {
        // wrapping_mul y wrapping_add evitan pánico en overflow
        hash = hash.wrapping_mul(31).wrapping_add(c as u64);
    }
    hash
}
fn main() -> ExitCode {
    let mensaje1 = "Contrato final v1";
    let mensaje2 = "Contrato final v2"; // Pequeño cambio

    let h1 = generar_hash(mensaje1);
    let h2 = generar_hash(mensaje2);
    println!("Mensaje: '{}' -> Hash: {:x}", mensaje1, h1);
    println!("Mensaje: '{}' -> Hash: {:x}", mensaje2, h2);
    if h1 != h2 {
        println!("¡Alerta! Los datos han sido modificados.");
    }
    ExitCode::SUCCESS
}
