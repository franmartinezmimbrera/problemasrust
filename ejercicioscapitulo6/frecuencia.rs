// Archivo: frecuencia.rs
// Descripción: Análisis de frecuencia de caracteres para criptoanálisis.

use std::collections::HashMap;
use std::process::ExitCode;

fn analizar_frecuencia(texto: &str) {

    let mut contador = HashMap::new();
    let mut total_letras = 0;

    // Contar apariciones
    
    for c in texto.chars() {
        if c.is_ascii_alphabetic() {
            let char_norm = c.to_ascii_uppercase();
            *contador.entry(char_norm).or_insert(0) += 1;
            total_letras += 1;
        }
    }

    // Convertir a vector para ordenar
    
    let mut lista: Vec<(&char, &i32)> = contador.iter().collect();
    
    // Ordenar de mayor a menor frecuencia
    lista.sort_by(|a, b| b.1.cmp(a.1));

    println!("Análisis de Frecuencia (Total letras: {})", total_letras);
    println!("----------------------------------------");
    for (letra, cuenta) in lista {
        let porcentaje = (*cuenta as f32 / total_letras as f32) * 100.0;
        // Imprimimos una barra visual simple
        let barra = "|".repeat((*cuenta) as usize);
        println!("{} : {:02.2}%  {}", letra, porcentaje, barra);
    }
}

fn main() -> ExitCode {
    
    let texto_cifrado = "HOLA ESTO ES UNA PRUEBA DE FRECUENCIA EN ESPAÑOL";
    analizar_frecuencia(texto_cifrado);
    
    ExitCode::SUCCESS
    
}
