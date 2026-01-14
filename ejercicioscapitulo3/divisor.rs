// archivo: divisor.rs
// Este programa calcula dados 2 números si a es divisor de b

use std::io;
use std::process::ExitCode;

fn es_divisor(a: i64, b: i64) -> bool {
    if a == 0 {
        return false;
    }

    let abs_a = a.abs();
    let abs_b = b.abs();

    abs_b % abs_a == 0
}

fn main() -> ExitCode {
    let mut entrada = String::new();
    let (a, b): (i64, i64);

    println!("Introduzca valor de a (el divisor):");
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("ERROR: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }

    a = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("ERROR: La entrada para 'a' no es un número entero válido.");
            return ExitCode::FAILURE;
        }
    };

    entrada.clear();

    println!("Introduzca valor de b (el dividendo):");
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("ERROR: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }

    b = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("ERROR: La entrada para 'b' no es un número entero válido.");
            return ExitCode::FAILURE;
        }
    };

    if es_divisor(a, b) {
        println!("{a} ES divisor de {b}");
    } else {
        println!("{a} NO es divisor de {b}");
    }

    ExitCode::SUCCESS
}
