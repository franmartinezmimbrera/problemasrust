// archivo: raizenesima.rs
// Este programa calcula la raíz n-ésima de un número

use std::io;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut entrada = String::new();
    let (numero, resultado): (f64, f64);
    let exponente: i32;

    println!("Introduce el número a calcular la raíz:");
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("Error: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }

    numero = match entrada.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Error: El valor debe de ser un número.");
            return ExitCode::FAILURE;
        }
    };

    entrada.clear();
    println!("Introduce exponente de raíz (un entero, ej: 2, 3..):");
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("Error: No se pudo leer el exponente.");
        return ExitCode::FAILURE;
    }

    exponente = match entrada.trim().parse() {
        Ok(e) => e,
        Err(_) => {
            println!("Error: El valor debe de ser un número entero.");
            return ExitCode::FAILURE;
        }
    };

    if exponente == 0 {
        println!("Error: El exponente de la raíz no puede ser 0");
        return ExitCode::FAILURE;
    }

    if numero < 0.0 && exponente % 2 == 0 {
        println!("Error: Imposible cálculo de raíz par de un número < 0 en R");
        return ExitCode::FAILURE;
    }

    let mut resultado = (numero.abs()).powf(1.0 / exponente as f64);

    if numero < 0.0 && exponente % 2 != 0 {
        resultado = -resultado;
    }

    println!(
        "La raíz {} ésima de {} es: {}",
        exponente, numero, resultado
    );

    ExitCode::SUCCESS
}
