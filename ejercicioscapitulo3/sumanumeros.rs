// fichero: sumanumeros.rs
// Este programa calcula la suma de los números introducidos por teclado
// mientras no se introduzca el número -50
use std::io::{self, Write};
fn main() {
    let mut suma: f64 = 0.0;
    loop {
        print!("Introduzca número a sumar (o -50 para terminar): ");
        io::stdout().flush().unwrap();
        let mut entrada = String::new();
        if io::stdin().read_line(&mut entrada).is_err() {
            eprintln!("ERROR: No se pudo leer la entrada.");
            continue;
        }
        let numero: f64 = match entrada.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("ERROR: La entrada no es un número válido. Inténtelo de nuevo.");
                continue;
            }
        };
        if numero == -50.0 {
            break;
        }
        suma += numero;
    }
    println!("La suma total de los números introducidos es: {}", suma);
    ExitCode::SUCCESS
}
