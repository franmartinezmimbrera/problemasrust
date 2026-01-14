// fichero: mcd1.rs
// Este programa calcula el MCD dados 2 números mediante una función

use std::io;
use std::process::ExitCode;

fn mcd(mut a: i64, mut b: i64) -> i64 {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }

    let mut temporal: i64;
    while a > 0 {
        temporal = a;
        a = b % a;
        b = temporal;
    }
    b
}

fn main() -> ExitCode {
    let mut entrada = String::new();

    println!("Introduzca valor de a:");
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("ERROR: La entrada no es un número entero");
        return ExitCode::FAILURE;
    }
    let mut a: i64 = match entrada.trim().parse() {
        Ok(v) => v,
        Err(_) => {
            println!("ERROR: La entrada no es un número entero");
            return ExitCode::FAILURE;
        }
    };

    entrada.clear();
    println!("Introduzca valor de b:");
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("ERROR: La entrada no es un número entero");
        return ExitCode::FAILURE;
    }
    let mut b: i64 = match entrada.trim().parse() {
        Ok(v) => v,
        Err(_) => {
            println!("ERROR: La entrada no es un número entero");
            return ExitCode::FAILURE;
        }
    };

    a = a.abs();
    b = b.abs();

    println!("EL M.C.D de {} y {} es: {}", a, b, mcd(a, b));

    ExitCode::SUCCESS
}
