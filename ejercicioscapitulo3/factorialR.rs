// archivo: factorialR.rs
// Calcula el factorial de un número entero no negativo de forma recursiva

use std::io;
use std::process::ExitCode;

fn factorial_recursivo(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * factorial_recursivo(n - 1)
    }
}

fn main() -> ExitCode {
    let mut entrada = String::new();

    println!("Introduce un número entero no negativo (máx 20 para precisión):");

    if io::stdin().read_line(&mut entrada).is_err() {
        eprintln!("Error: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }

    let numero: i64 = match entrada.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: Entrada no válida.");
            return ExitCode::FAILURE;
        }
    };

    if numero < 0 {
        eprintln!("Error: El factorial solo está definido para números no negativos.");
        return ExitCode::FAILURE;
    }

    if numero > 20 {
        println!(
            "Advertencia: El factorial de {} es muy grande y probablemente cause un resultado incorrecto por desbordamiento (overflow).",
            numero
        );
    }

    let resultado = factorial_recursivo(numero as u64);

    println!("El factorial de {} es: {}", numero, resultado);

    ExitCode::SUCCESS
}
