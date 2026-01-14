// archivo: areatriangulo.rs
// Calculamos el área de un triángulo mediante la fórmula de Herón

use std::io;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut entrada = String::new();
    let (l1, l2, l3): (f64, f64, f64);

    println!("Introduce lo que mide el primer lado del triángulo: ");
    entrada.clear();
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("Error: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    l1 = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El valor debe de ser un número.");
            return ExitCode::FAILURE;
        }
    };
    if l1 < 0.0 {
        println!("Error: El valor debe ser un número positivo");
        return ExitCode::FAILURE;
    }

    println!("Introduce lo que mide el segundo lado del triángulo: ");
    entrada.clear();
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("Error: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    l2 = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El valor debe de ser un número.");
            return ExitCode::FAILURE;
        }
    };
    if l2 < 0.0 {
        println!("Error: El valor debe ser un número positivo");
        return ExitCode::FAILURE;
    }

    println!("Introduce lo que mide el tercer lado del triángulo: ");
    entrada.clear();
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("Error: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    l3 = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El valor debe de ser un número.");
            return ExitCode::FAILURE;
        }
    };
    if l3 < 0.0 {
        println!("Error: El valor debe ser un número positivo");
        return ExitCode::FAILURE;
    }

    if l1 + l2 <= l3 || l1 + l3 <= l2 || l2 + l3 <= l1 {
        println!(
            "\nError: Las longitudes ({}, {}, {}) no forman un triángulo válido (desigualdad triangular).",
            l1, l2, l3
        );
        return ExitCode::FAILURE;
    }

    let sp = (l1 + l2 + l3) / 2.0;
    let area = (sp * (sp - l1) * (sp - l2) * (sp - l3)).sqrt();

    println!("El área del triángulo es: {:.4}", area);

    ExitCode::SUCCESS
}
