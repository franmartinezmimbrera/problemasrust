// archivo: pesetaseuros.rs
// Este programa realiza la conversión de pesetas a euros
use std::io;
use std::process::ExitCode;
const TASA_CONVERSION: f64 = 166.386;
fn main() -> ExitCode {
    let mut entrada = String::new();
    let (pesetas, euros): (f64, f64);
    println!("Introduzca su valor en pesetas:");
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("ERROR: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    pesetas = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("ERROR: La entrada no es un número válido.");
            return ExitCode::FAILURE;
        }
    };
    if pesetas < 0.0 {
        println!("ERROR: El valor en pesetas no puede ser negativo.");
        return ExitCode::FAILURE;
    }
    euros = pesetas / TASA_CONVERSION;
    println!(
        "Su valor de {:.2} pesetas equivale a: {:.2} euros",
        pesetas, euros
    );
    ExitCode::SUCCESS
}
