// fichero: newton_iterativo.rs
// Expansión del binomio de Newton (a + b)^n con funciones iterativas

use std::io::{self, Write};
use std::process::ExitCode;

fn factorial(n: u64) -> u64 {
    let mut resultado = 1;
    for i in 2..=n {
        resultado *= i;
    }
    resultado
}

fn ncr(n: u64, r: u64) -> Option<u64> {
    if r > n {
        return None;
    }

    let fact_n = factorial(n);
    let fact_r = factorial(r);
    let fact_n_r = factorial(n - r);

    fact_r
        .checked_mul(fact_n_r)
        .and_then(|den| fact_n.checked_div(den))
}

fn potencia(mut base: u64, mut exponente: u32) -> u64 {
    let mut resultado = 1;
    while exponente > 0 {
        if exponente % 2 == 1 {
            resultado *= base;
        }
        base *= base;
        exponente /= 2;
    }
    resultado
}

fn binomio_de_newton(a: u64, b: u64, n: u32) {
    println!("\nExpansión de ({} + {})^{}:", a, b, n);
    let mut suma: u64 = 0;

    for k in 0..=n {
        let coef = match ncr(n as u64, k as u64) {
            Some(val) => val,
            None => {
                eprintln!("Error: desbordamiento en cálculo de nCr({}, {})", n, k);
                return ExitCode::FAILURE;
            }
        };

        let pot_a = potencia(a, n - k);
        let pot_b = potencia(b, k);
        let termino = coef * pot_a * pot_b;
        suma += termino;

        println!(
            "  + ({} * {}^{} * {}^{}) = {}",
            coef,
            a,
            n - k,
            b,
            k,
            termino
        );
    }

    println!("\nResultado total: ({} + {})^{} = {}", a, b, n, suma);
}

fn leer_u64(mensaje: &str) -> Option<u64> {
    print!("{}", mensaje);
    let _ = io::stdout().flush();
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).ok()?;
    entrada.trim().parse::<u64>().ok()
}

fn leer_u32(mensaje: &str) -> Option<u32> {
    print!("{}", mensaje);
    let _ = io::stdout().flush();
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).ok()?;
    entrada.trim().parse::<u32>().ok()
}

fn main() -> ExitCode {
    let a = match leer_u64("Introduce un número entero no negativo para a: ") {
        Some(val) => val,
        None => {
            eprintln!("Error: Entrada no válida para a.");
            return ExitCode::FAILURE;
        }
    };

    let b = match leer_u64("Introduce un número entero no negativo para b: ") {
        Some(val) => val,
        None => {
            eprintln!("Error: Entrada no válida para b.");
            return ExitCode::FAILURE;
        }
    };

    let n = match leer_u32("Introduce un número entero no negativo para n: ") {
        Some(val) => val,
        None => {
            eprintln!("Error: Entrada no válida para n.");
            return ExitCode::FAILURE;
        }
    };

    if n > 20 || (a > 1 && n > 15) || (b > 1 && n > 15) {
        println!("Advertencia: La expansión puede causar desbordamiento.");
    }

    binomio_de_newton(a, b, n);
    ExitCode::SUCCESS
}
