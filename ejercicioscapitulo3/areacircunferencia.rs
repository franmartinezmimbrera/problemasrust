// archivo: areacircunferencia.rs
// Este programa calcula el Área de una circunferencia

use std::io;
use std::process::ExitCode;

const PI_VALOR: f64 = 3.14159265358979323846;

fn area_circunferencia(r: f64) -> f64 {
    PI_VALOR * r.powi(2)
}

fn main() -> ExitCode {
    let mut entrada = String::new();
    let radio: f64;

    loop {
        println!("Introduce el radio de la circunferencia >0:");
        entrada.clear();
        if io::stdin().read_line(&mut entrada).is_err() {
            println!("ERROR: No se pudo leer la entrada.");
            continue;
        }

        match entrada.trim().parse::<f64>() {
            Ok(valor) => {
                if valor <= 0.0 {
                    println!("ERROR: El radio debe ser mayor que cero");
                    continue;
                } else {
                    radio = valor;
                    break;
                }
            }
            Err(_) => {
                println!("ERROR: La entrada no es un número válido.");
                continue;
            }
        }
    }

    println!(
        "El área de la circunferencia es: {}",
        area_circunferencia(radio)
    );

    ExitCode::SUCCESS
}
