// archivo: areatriangulo1.rs
// Este programa calcula el área de un triángulo rectángulo a partir de la base y la altura

use std::io;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut entrada = String::new();
    let (a, b, area): (f64, f64, f64);

    println!("Introduce la base del triángulo rectángulo:");
    entrada.clear();
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("Error: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    b = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El valor debe de ser un número.");
            return ExitCode::FAILURE;
        }
    };
    if b <= 0.0 {
        println!("Error: El valor debe ser un número > que 0");
        return ExitCode::FAILURE;
    }

    println!("Introduce la altura del triángulo rectángulo:");
    entrada.clear();
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("Error: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    a = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El valor debe de ser un número.");
            return ExitCode::FAILURE;
        }
    };
    if a <= 0.0 {
        println!("Error: El valor debe ser un número > que 0");
        return ExitCode::FAILURE;
    }

    area = (a * b) / 2.0;

    println!("El área del triángulo rectángulo es: {:.2}", area);

    ExitCode::SUCCESS
}
