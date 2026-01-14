// archivo: tipotriangulo.cpp 
// Este programa calcula el tipo de triángulo en función de los lados

use std::io::{self, Write};

fn leer_lado(nombre: &str) -> Result<f64, ()> {
    print!("Introduce lo que mide el {} lado del triángulo: ", nombre);
    io::stdout().flush().unwrap();
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Error al leer entrada");
    match entrada.trim().parse::<f64>() {
        Ok(valor) if valor > 0.0 => Ok(valor),
        _ => {
            println!("Error: la entrada no es un número válido o es <= 0");
            Err(())
        }
    }
}

fn main() {
    let l1 = match leer_lado("primer") {
        Ok(val) => val,
        Err(_) => return ExitCode::FAILURE,
    };

    let l2 = match leer_lado("segundo") {
        Ok(val) => val,
        Err(_) => return ExitCode::FAILURE,
    };

    let l3 = match leer_lado("tercer") {
        Ok(val) => val,
        Err(_) => return,
    };

    if l1 + l2 <= l3 || l1 + l3 <= l2 || l2 + l3 <= l1 {
        println!("\nError Geométrico: Los lados NO forman un triángulo válido.");
        return;
    }

    if (l1 == l2) && (l2 == l3) {
        println!("El Triángulo es Equilátero");
    } else if (l1 == l2) || (l1 == l3) || (l2 == l3) {
        println!("El Triángulo es Isósceles");
    } else {
        println!("El Triángulo es Escaleno");
    }
    ExitCode::SUCCESS

}
