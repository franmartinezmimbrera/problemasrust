// archivo: conteo.rs
// Este ejercicio convierte cadenas a mayúsculas y cuenta sus vocales

use std::io::{self, Write};

const NUM_FRASES: usize = 4;

fn convertir_a_mayusculas(s: &str) -> String {
    s.to_uppercase()
}

fn contar_vocales(s: &str) -> usize {
    s.chars()
        .filter(|c| matches!(c, 'A' | 'E' | 'I' | 'O' | 'U'))
        .count()
}

fn main() {
    let mut frases: Vec<String> = Vec::with_capacity(NUM_FRASES);
    let mut total_vocales = 0;

    println!("Introduce {} frases/líneas de texto:", NUM_FRASES);

    for i in 0..NUM_FRASES {
        print!("Frase {}: ", i + 1);
        io::stdout().flush().unwrap();

        let mut entrada = String::new();
        if let Err(_) = io::stdin().read_line(&mut entrada) {
            eprintln!("Error en la lectura. Terminando...");
            std::process::exit(1);
        }

        let linea = entrada.trim_end().to_string();
        frases.push(linea);
    }

    println!("\n--- Procesamiento y Conteo ---");
    for (i, frase) in frases.iter().enumerate() {
        let mayus = convertir_a_mayusculas(frase);
        let vocales = contar_vocales(&mayus);
        total_vocales += vocales;
        println!("Frase {} (MAYÚS): '{}' -> Vocales contadas: {}", i + 1, mayus, vocales);
    }

    println!("\n--- Resumen Final ---");
    println!("El número total de vocales en todas las frases es: {}", total_vocales);
}
