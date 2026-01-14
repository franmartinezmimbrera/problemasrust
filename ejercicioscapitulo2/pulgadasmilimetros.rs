// archivo: pulgadasmilimetros.rs
// Este programa cambia pulgadas por milímetros
use std::io;
use std::process::ExitCode;
const FACTOR_CONVERSION: f64 = 25.4;
fn main() -> ExitCode {
    let mut entrada = String::new();
    let (pul, mil): (f64, f64);
    println!("Introduzca valor en pulgadas:");
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("Error: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    pul = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El valor debe de ser un número.");
            return ExitCode::FAILURE;
        }
    };
    if pul < 0.0 {
        println!("Error: El valor debe ser un número positivo");
        return ExitCode::FAILURE;
    }
    mil = FACTOR_CONVERSION * pul;
    println!("El resultado en milímetros es: {}", mil);
    ExitCode::SUCCESS
}
