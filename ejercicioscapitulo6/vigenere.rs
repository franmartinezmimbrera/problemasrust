// Archivo: vigenere.rs
// Descripción: Cifrado de Vigenere usando iteradores cíclicos para la clave.
use std::process::ExitCode;

fn cifrar_vigenere(texto: &str, clave: &str) -> String {
    // Convertimos la clave a un vector de desplazamientos (0-25)
    let desplazamientos: Vec<u8> = clave
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| (c.to_ascii_uppercase() as u8) - b'A')
        .collect();

    if desplazamientos.is_empty() { 
        return texto.to_string(); 
    }

    let mut resultado = String::new();
    let mut key_iter = desplazamientos.iter().cycle(); // Iterador infinito de la clave

    for c in texto.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let shift = key_iter.next().unwrap(); // Obtenemos el siguiente shift
            
            let c_u8 = c as u8 - base;
            let nuevo_c = (c_u8 + shift) % 26;
            
            resultado.push((base + nuevo_c) as char);
        } else {
            resultado.push(c);
        }
    }
    resultado
}

fn main() -> ExitCode {

    let texto = "ATACAR AL AMANECER";
    let clave = "LIMON";
    
    let cifrado = cifrar_vigenere(texto, clave);

    println!("Texto: {}", texto);
    println!("Clave: {}", clave);
    println!("Cifrado: {}", cifrado);

    ExitCode::SUCCESS
}
