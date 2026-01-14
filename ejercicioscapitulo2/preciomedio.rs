// archivo: preciomedio.rs
// Este programa calcula el precio medio de un producto

use std::io;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut entrada = String::new();
    let (precio1, precio2, precio3, media): (f64, f64, f64, f64);

    println!("Introduzca el precio en establecimiento 1, en euros:");
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("Error: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    precio1 = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El valor debe de ser un número.");
            return ExitCode::FAILURE;
        }
    };
    if precio1 < 0.0 {
        println!("Ningún precio puede ser negativo");
        return ExitCode::FAILURE;
    }

    entrada.clear();
    println!("Introduzca el precio en establecimiento 2, en euros:");
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("Error: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    precio2 = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El valor debe de ser un número.");
            return ExitCode::FAILURE;
        }
    };
    if precio2 < 0.0 {
        println!("Ningún precio puede ser negativo");
        return ExitCode::FAILURE;
    }

    entrada.clear();
    println!("Introduzca el precio en establecimiento 3, en euros:");
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("Error: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    precio3 = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El valor debe de ser un número.");
            return ExitCode::FAILURE;
        }
    };
    if precio3 < 0.0 {
        println!("Ningún precio puede ser negativo");
        return ExitCode::FAILURE;
    }

    media = (precio1 + precio2 + precio3) / 3.0;

    println!("El precio medio del producto es de {} euros", media);

    ExitCode::SUCCESS
}
