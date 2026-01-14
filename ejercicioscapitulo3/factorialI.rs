// archivo: factorial_iterativo.rs
// Calcula el factorial de un número entero no negativo de forma iterativa

use std::io;
use std::process::ExitCode;

fn factorial_iterativo(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return 1;
    }

    let mut resultado: u64 = 1;
    for i in 2..=n {
        resultado *= i;
    }

    resultado
}

fn main() -> ExitCode {
    let mut entrada = String::new();
    println!("Introduce un número entero no negativo (máx 20 para precisión):");

    if let Err(_) = io::stdin().read_line(&mut entrada) {
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
        println!("Advertencia: El factorial de {} puede causar overflow (resultado incorrecto).", numero);
    }

    let resultado = factorial_iterativo(numero as u64);
    println!("El factorial de {} es: {}", numero, resultado);

    ExitCode::SUCCESS
}
