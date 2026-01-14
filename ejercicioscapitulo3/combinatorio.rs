// archivo: combinatorio.rs
// Calcula el número combinatorio C(n, r) o "n sobre r", usando factoriales.

use std::io;
use std::process::ExitCode;

fn factorial(n: u64) -> Option<u64> {
    if n == 0 || n == 1 {
        return Some(1);
    }
    let mut resultado: u64 = 1;
    for i in 2..=n {
        resultado = resultado.checked_mul(i)?;
    }
    Some(resultado)
}

fn ncr(n: u64, r: u64) -> i64 {
    if r > n {
        return 0;
    }

    let fact_n = factorial(n);
    let fact_r = factorial(r);
    let fact_n_r = factorial(n - r);

    match (fact_n, fact_r, fact_n_r) {
        (Some(f1), Some(f2), Some(f3)) => {
            match f2.checked_mul(f3) {
                Some(den) => (f1 / den) as i64,
                None => -1,
            }
        }
        _ => -1,
    }
}

fn main() -> ExitCode {
    let mut entrada = String::new();
    let (n, m): (u64, u64);

    println!("Introduce un número entero no negativo para N (max 20 para precisión):");
    if io::stdin().read_line(&mut entrada).is_err() {
        eprintln!("Error: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    n = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Entrada no válida para N.");
            return ExitCode::FAILURE;
        }
    };
    if n > 20 {
        println!("Advertencia: El factorial de N={} podría causar desbordamiento.", n);
    }

    entrada.clear();
    println!("Introduce un número entero no negativo para M (max 20 para precisión):");
    if io::stdin().read_line(&mut entrada).is_err() {
        eprintln!("Error: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    m = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Entrada no válida para M.");
            return ExitCode::FAILURE;
        }
    };
    if m > 20 {
        println!("Advertencia: El factorial de M={} podría causar desbordamiento.", m);
    }

    let combinaciones = ncr(n, m);

    if combinaciones > 0 {
        println!("El número combinatorio de {} sobre {} es: {}", n, m, combinaciones);
    } else if combinaciones == 0 {
        println!("El número combinatorio de {} sobre {} es 0 (M fuera de rango 0 <= M <= N).", n, m);
    } else {
        println!("No se pudo calcular el número combinatorio de {} sobre {} debido a desbordamiento.", n, m);
    }

    ExitCode::SUCCESS
}
