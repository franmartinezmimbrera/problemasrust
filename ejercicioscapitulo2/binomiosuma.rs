// archivo: binomiosuma.rs
// Este programa calcula el binomio de suma de (a + b) al cuadrado

use std::io;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut entrada = String::new();
    let (a, b, resultado): (f64, f64, f64);

    println!("Introduce el valor de a:");
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("Error: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    a = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El valor debe ser un número.");
            return ExitCode::FAILURE;
        }
    };

    entrada.clear();
    println!("Introduce el valor de b:");
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("Error: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    b = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El valor debe ser un número.");
            return ExitCode::FAILURE;
        }
    };

    resultado = (a * a) + (b * b) + (2.0 * a * b);

    println!("El resultado del binomio de suma de (a + b)^2 es: {}", resultado);

    ExitCode::SUCCESS
}
