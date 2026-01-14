// archivo: areatriangulo2.rs
// Este programa calcula el Área de un triángulo equilátero a partir de uno de sus lados

use std::io;
use std::process::ExitCode;
use std::f64;

fn main() -> ExitCode {
    let mut entrada = String::new();
    let (l, area): (f64, f64);

    println!("Introduce un lado del triángulo equilátero:");

    if io::stdin().read_line(&mut entrada).is_err() {
        println!("Error: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }

    l = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El valor debe ser un número.");
            return ExitCode::FAILURE;
        }
    };

    if l <= 0.0 {
        println!("Error: El valor debe ser un número mayor que 0.");
        return ExitCode::FAILURE;
    }

    area = (f64::sqrt(3.0) / 4.0) * l * l;

    println!(
        "El Área del triángulo equilátero de lado {} es: {}",
        l, area
    );

    ExitCode::SUCCESS
}
