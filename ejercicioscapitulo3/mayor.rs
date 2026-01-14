// fichero: mayor.rs
// Este programa calcula el número mayor de 10 introducidos por teclado

use std::io::{self, Write};
use std::process::ExitCode;
use std::i64;

const NUM_ELEMENTOS: usize = 10;

fn mayor(numeros: &[i64]) -> i64 {
    if numeros.is_empty() {
        return 0;
    }

    let mut max = numeros[0];
    for &num in numeros.iter() {
        if num > max {
            max = num;
        }
    }
    max
}

fn main() -> ExitCode {
    let mut numeros: Vec<i64> = Vec::with_capacity(NUM_ELEMENTOS);

    for i in 0..NUM_ELEMENTOS {
        loop {
            print!("Introduzca número {} de {}: ", i + 1, NUM_ELEMENTOS);
            io::stdout().flush().unwrap(); // asegura impresión antes de leer

            let mut entrada = String::new();
            if let Err(_) = io::stdin().read_line(&mut entrada) {
                println!("ERROR: No se pudo leer la entrada. Inténtelo de nuevo.");
                continue;
            }

            match entrada.trim().parse::<i64>() {
                Ok(valor) => {
                    numeros.push(valor);
                    break;
                }
                Err(_) => {
                    println!("ERROR: La entrada no es un número entero válido. Inténtelo de nuevo.");
                return ExitCode::FAILURE;

                }
            }
        }
    }

    println!(
        "\nEl número mayor de todos los introducidos es: {}",
        mayor(&numeros)
    );

    ExitCode::SUCCESS
}
