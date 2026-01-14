// archivo: ladotriangulo.rs
// Este programa calcula el valor del lado a de un triángulo rectángulo usando el valor del lado b y la hipotenusa h

use std::io;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut entrada = String::new();
    let (a, b, h): (f64, f64, f64);

    println!("Introduzca el valor del lado \"b\" del triángulo rectángulo:");
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("Error: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    b = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El valor debe ser un número.");
            return ExitCode::FAILURE;
        }
    };
    if b <= 0.0 {
        println!("Error: El lado 'b' debe ser un número mayor que 0.");
        return ExitCode::FAILURE;
    }

    entrada.clear();
    println!("Introduzca el valor de la hipotenusa del triángulo rectángulo:");
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("Error: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    h = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El valor debe ser un número.");
            return ExitCode::FAILURE;
        }
    };
    if h <= 0.0 {
        println!("Error: La hipotenusa debe ser un número mayor que 0.");
        return ExitCode::FAILURE;
    }
    if b >= h {
        println!("Error: La hipotenusa ({}) debe ser mayor que el cateto 'b' ({}).", h, b);
        return ExitCode::FAILURE;
    }

    a = ((h * h) - (b * b)).sqrt();

    println!("El valor del lado \"a\" del triángulo rectángulo es: {}", a);

    ExitCode::SUCCESS
}
