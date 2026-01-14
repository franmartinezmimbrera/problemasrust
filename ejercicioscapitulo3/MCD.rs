// fichero: mcd.rs
// Este programa calcula el MCD dados 2 números enteros

use std::io::{self, Write};
use std::process::ExitCode;

fn leer_entero(prompt: &str) -> i64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut entrada = String::new();
        if io::stdin().read_line(&mut entrada).is_err() {
            println!("ERROR: No se pudo leer la entrada.");
            continue;
        }

        match entrada.trim().parse::<i64>() {
            Ok(valor) => return valor,
            Err(_) => {
                println!("ERROR: La entrada no es un número entero válido.");
                return ExitCode::FAILURE;

            }
        }
    }
}

fn calcular_mcd(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();

    if a == 0 || b == 0 {
        return if a == 0 { b } else { a };
    }

    while a != 0 {
        let temp = a;
        a = b % a;
        b = temp;
    }

    b
}

fn main() -> ExitCode {
    let a_original = leer_entero("\n   Introduzca valor de a: ");
    let b_original = leer_entero("\n   Introduzca valor de b: ");

    let resultado = calcular_mcd(a_original, b_original);

    println!(
        "\n EL M.C.D de {} y {} es: {}",
        a_original, b_original, resultado
    );

    ExitCode::SUCCESS
}
