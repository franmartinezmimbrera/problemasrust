// archivo: costecoche.rs
// Este programa calcula el precio total de un coche

use std::io;
use std::process::ExitCode;

const GANANCIA: f64 = 1.15;
const IMPUESTO: f64 = 1.21;

fn main() -> ExitCode {
    let mut entrada = String::new();
    let (costecoche, pvantesimpuestos, preciototal): (f64, f64, f64);

    println!("Introduce el coste del vehiculo:");
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("Error: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }

    costecoche = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El valor debe de ser un número.");
            return ExitCode::FAILURE;
        }
    };

    if costecoche < 0.0 {
        println!("Error: El valor debe ser un número positivo");
        return ExitCode::FAILURE;
    }

    pvantesimpuestos = costecoche * GANANCIA;
    preciototal = pvantesimpuestos * IMPUESTO;

    println!(
        "El precio total del automovil nuevo para el comprador es: {:.2}",
        preciototal
    );

    ExitCode::SUCCESS
}
