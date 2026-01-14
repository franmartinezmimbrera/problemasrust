// archivo: cambiagrados.rs
// Este programa cambia grados centígrados por Fahrenheit

use std::io;
use std::process::ExitCode;

const FACTOR_C_TO_F: f64 = 9.0 / 5.0;

fn main() -> ExitCode {
    let mut entrada = String::new();
    let (c, f): (f64, f64);
    println!("Introduzca valor en grados Centígrados :");
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("Error: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    c = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El valor debe de ser un número.");
            return ExitCode::FAILURE;
        }
    };
    f = (FACTOR_C_TO_F * c) + 32.0;
    println!("El resultado en grados Fahrenheit es: {}", f);
    ExitCode::SUCCESS
}
