// archivo: raizcuadrada.rs
// Este programa calcula la raíz cuadrada de un número
use std::io;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut entrada = String::new();
    let (numero, resultado): (f64, f64);

    println!("Introduce el número a calcular la raíz cuadrada:");
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("Error: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    numero = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El valor debe de ser un número.");
            return ExitCode::FAILURE;
        }
    };
    if numero < 0.0 {
        println!("Error: El valor debe ser un número positivo");
        return ExitCode::FAILURE;
    }

    resultado = numero.sqrt();

    println!("La raíz cuadrada de {} es: {}", numero, resultado);

    ExitCode::SUCCESS
}
