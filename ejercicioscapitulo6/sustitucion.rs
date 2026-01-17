// Archivo: sustitucion.rs
// Descripción: Sustitución monoalfabética usando un alfabeto desordenado.
use std::collections::HashMap;
use std::process::ExitCode;
fn main() -> ExitCode {
    let alfabeto_normal = "abcdefghijklmnopqrstuvwxyz";
    let alfabeto_clave  = "qwertyuiopasdfghjklzxcvbnm"; // Teclado QWERTY
    let mensaje = "hola mundo";
    
    // Crear el mapa de sustitución
    let mut mapa = HashMap::new();
    for (k, v) in alfabeto_normal.chars().zip(alfabeto_clave.chars()) {
        mapa.insert(k, v);
    }
    // Cifrar
    let cifrado: String = mensaje.chars().map(|c| {
        // Buscamos en el mapa, si no está (ej. espacio), devolvemos c tal cual
        match mapa.get(&c) {
            Some(&sustituto) => sustituto,
            None => c
        }
    }).collect();
    println!("Original: {}", mensaje);
    println!("Cifrado (QWERTY): {}", cifrado);
    ExitCode::SUCCESS
}
