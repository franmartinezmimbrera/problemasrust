// archivo: bisiesto.rs
// Este programa dice si un año es bisiesto o no

use std::io;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut entrada = String::new();
    let anio: i32;

    println!("Introduzca un año:");

    if io::stdin().read_line(&mut entrada).is_err() {
        println!("ERROR: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }

    anio = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("ERROR: La entrada no es un número entero.");
            return ExitCode::FAILURE;
        }
    };
    if (anio % 4 == 0 && anio % 100 != 0) || (anio % 400 == 0) {
        println!("ES BISIESTO");
    } else {
        println!("NO ES BISIESTO");
    }
    ExitCode::SUCCESS
}
