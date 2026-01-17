// Archivo: xor_cifra_descifra.rs
// Descripción: Cifrado simétrico usando la propiedad involutiva de XOR.
use std::process::ExitCode;
fn procesar_xor(entrada: &str, clave: char) -> String {
    let clave_byte = clave as u8;    
    // Convertimos a bytes, aplicamos XOR y (intentamos) reconstruir el string.
    let bytes_procesados: Vec<u8> = entrada.bytes()
        .map(|b| b ^ clave_byte)
        .collect();
    // String::from_utf8_lossy reemplaza caracteres inválidos con 
    String::from_utf8_lossy(&bytes_procesados).to_string()
}
fn main() -> ExitCode {
    let original = "Secreto 123";
    let clave = 'K'; // Clave de un solo carácter
    // Cifrar
    let cifrado = procesar_xor(original, clave);
    // Nota: Al imprimir 'cifrado' es posible que veas caracteres raros
    println!("Original:  {}", original);
    println!("Cifrado:   {:?}", cifrado); // Usamos debug {:?} para ver caracteres especiales
    // Descifrar (Aplicar XOR de nuevo con la misma clave)
    let descifrado = procesar_xor(&cifrado, clave);
    println!("Recuperado: {}", descifrado);
    ExitCode::SUCCESS
}
