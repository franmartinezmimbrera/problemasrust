// Archivo: cesar_bruta.rs
// Descripción: Ataque de fuerza bruta probando las 25 combinaciones.
use std::process::ExitCode;

// Reutilizamos la lógica de descifrado (podría estar en un módulo aparte)
fn descifrar_rot(texto: &str, n: i32) -> String {
    texto
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' } as i32;
                let val = (c as i32 - base - n).rem_euclid(26);
                (base + val) as u8 as char
            } else {
                c
            }
        })
        .collect()
}

fn main() -> ExitCode {
    let mensaje_interceptado = "Qeb nrfzh yoltk clu"; // "The quick brown fox" (n=-3 o rot23)

    println!("Interceptado: {}", mensaje_interceptado);
    println!("Probando todas las claves posibles:\n");

    for key in 1..26 {
        let candidato = descifrar_rot(mensaje_interceptado, key);
        println!("Clave {:02}: {}", key, candidato);
    }

    println!("\nBusca la frase que tenga sentido en la lista.");
    ExitCode::SUCCESS
}
