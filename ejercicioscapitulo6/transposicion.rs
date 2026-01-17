// Archivo: transposicion.rs
// Descripción: Cifrado por transposición columnar (Escítala).
use std::process::ExitCode;

fn cifrar_escitala(texto: &str, columnas: usize) -> String {
    let chars: Vec<char> = texto.chars().collect();
    let n = chars.len();
    // Calculamos las filas necesarias (redondeo hacia arriba)
    let filas = (n as f64 / columnas as f64).ceil() as usize;
    let mut resultado = String::with_capacity(n);
    // Leemos por columnas en lugar de por filas
    for c in 0..columnas {
        for f in 0..filas {
            let index = f * columnas + c;
            if index < n {
                resultado.push(chars[index]);
            }
        }
    }
    resultado
}
fn main() -> ExitCode {
    let texto = "ESTO ES UN SECRETO";
    let columnas = 4;
    let cifrado = cifrar_escitala(texto, columnas);
    println!("Original: {}", texto);
    println!("Cifrado (col={}): {}", columnas, cifrado);
    ExitCode::SUCCESS
}
