// archivo: ecuaciones.rs
// Este programa resuelve ecuaciones de segundo grado con soluciones reales

use std::io;
use std::process::ExitCode;

fn main() -> ExitCode {
    let (a, b, c): (f64, f64, f64);

    let a_input = loop {
        println!("Ingrese coeficiente a:");
        let mut entrada = String::new();
        if io::stdin().read_line(&mut entrada).is_err() {
            eprintln!("Error al leer la entrada.");
            return ExitCode::FAILURE;
        }

        match entrada.trim().parse::<f64>() {
            Ok(valor) if valor != 0.0 => break valor,
            Ok(_) => println!("El coeficiente 'a' no puede ser cero."),
            Err(_) => {
                eprintln!("Error: la entrada no es un número válido.");
                return ExitCode::FAILURE;
            }
        }
    };
    a = a_input;

    let mut entrada = String::new();
    println!("Ingrese coeficiente b:");
    if io::stdin().read_line(&mut entrada).is_err() {
        eprintln!("Error al leer la entrada.");
        return ExitCode::FAILURE;
    }
    b = match entrada.trim().parse::<f64>() {
        Ok(valor) => valor,
        Err(_) => {
            eprintln!("Error: la entrada no es un número válido.");
            return ExitCode::FAILURE;
        }
    };

    entrada.clear();
    println!("Ingrese coeficiente c:");
    if io::stdin().read_line(&mut entrada).is_err() {
        eprintln!("Error al leer la entrada.");
        return ExitCode::FAILURE;
    }
    c = match entrada.trim().parse::<f64>() {
        Ok(valor) => valor,
        Err(_) => {
            eprintln!("Error: la entrada no es un número válido.");
            return ExitCode::FAILURE;
        }
    };

    let d = b.powi(2) - 4.0 * a * c;

    if d > 0.0 {
        let x1 = (-b + d.sqrt()) / (2.0 * a);
        let x2 = (-b - d.sqrt()) / (2.0 * a);
        println!("x1 = {}", x1);
        println!("x2 = {}", x2);
    } else if d == 0.0 {
        let x = -b / (2.0 * a);
        println!("x1 = {}", x);
    } else {
        println!("La ecuación no tiene soluciones reales");
    }

    ExitCode::SUCCESS
}
