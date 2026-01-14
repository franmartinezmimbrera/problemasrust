// fichero: sumaparesimpares.rs
// Calcula la suma de 10 números pasados por teclado, distinguiendo entre pares e impares

use std::io::{self, Write};

fn main() {
    const TAMANO_ARRAY: usize = 10;
    let mut numeros: Vec<i64> = Vec::with_capacity(TAMANO_ARRAY);
    let mut suma_pares: i64 = 0;
    let mut suma_impares: i64 = 0;

    println!("Por favor, introduce {TAMANO_ARRAY} números enteros:");

    for i in 0..TAMANO_ARRAY {
        loop {
            print!("Número {}: ", i + 1);
            io::stdout().flush().unwrap();
            let mut entrada = String::new();
            if io::stdin().read_line(&mut entrada).is_err() {
                println!("Error al leer entrada. Intenta de nuevo.");
                continue;
            }

            match entrada.trim().parse::<i64>() {
                Ok(numero) => {
                    numeros.push(numero);
                    if numero % 2 == 0 {
                        suma_pares += numero;
                        println!("Añadiendo {} a la suma par.", numero);
                    } else {
                        suma_impares += numero;
                        println!("Añadiendo {} a la suma impar.", numero);
                    }
                    break;
                }
                Err(_) => {
                    println!("Entrada no válida. Introduce un número entero.");
                    return ExitCode::FAILURE;
                }
            }
        }
    }

    print!("Números ingresados: [");
    for (i, num) in numeros.iter().enumerate() {
        print!("{}", num);
        if i < numeros.len() - 1 {
            print!(", ");
        }
    }
    println!("]");

    println!("Suma total de los números PARES: {}", suma_pares);
    println!("Suma total de los números IMPARES: {}", suma_impares);
    ExitCode::SUCCESS

}
