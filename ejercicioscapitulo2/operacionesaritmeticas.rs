// archivo: operacionesaritmeticas.rs
// Este programa realiza operaciones aritméticas dados 2 números

use std::io;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut entrada = String::new();
    let (valor1, valor2): (f64, f64);
    println!("Introduce el valor primero:");
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("ERROR: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    valor1 = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("ERROR: El valor introducido no es un número.");
            return ExitCode::FAILURE;
        }
    };
    entrada.clear();
    println!("Introduce el valor segundo:");
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("ERROR: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    valor2 = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("ERROR: El valor introducido no es un número.");
            return ExitCode::FAILURE;
        }
    };
    println!(
        "El resultado de la suma ({} + {}) es: {}",
        valor1,
        valor2,
        valor1 + valor2
    );
    println!(
        "El resultado de la resta ({} - {}) es: {}",
        valor1,
        valor2,
        valor1 - valor2
    );
    println!(
        "El resultado de la multiplicación ({} * {}) es: {}",
        valor1,
        valor2,
        valor1 * valor2
    );
    if valor2 == 0.0 {
        println!(
            "El resultado de la división ({} / {}) es: INDEFINIDO (División por Cero)",
            valor1, valor2
        );
    } else {
        println!(
            "El resultado de la división ({} / {}) es: {}",
            valor1,
            valor2,
            valor1 / valor2
        );
    }
    ExitCode::SUCCESS
}
