// fichero: sumanumerosnaturales.rs
// Este programa calcula la suma de los "n" primeros números naturales con for

use std::io::{self, Write};

fn main() {
    let numero: u64;

    loop {
        print!("Introduzca un número n > 0 de términos a sumar: ");
        io::stdout().flush().unwrap();
        let mut entrada = String::new();
        if io::stdin().read_line(&mut entrada).is_err() {
            println!("ERROR: No se pudo leer la entrada.");
            continue;
        }

        match entrada.trim().parse::<u64>() {
            Ok(n) if n > 0 => {
                numero = n;
                break;
            }
            Ok(_) => {
                println!("ERROR: El número de términos debe ser > 0.");
                return ExitCode::FAILURE;
            }
            Err(_) => {
                println!("ERROR: La entrada no es un número entero válido.");
                return ExitCode::FAILURE;
            }
        }
    }

    let mut suma: u64 = 0;
    for i in 1..=numero {
        suma += i;
    }

    println!("La suma de los {} primeros números naturales es: {}", numero, suma);
    ExitCode::SUCCESS

}
