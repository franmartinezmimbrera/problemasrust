// archivo: fibo.rs
// Función iterativa para calcular el n-ésimo número de Fibonacci

use std::process::ExitCode;

fn fibonacci_iterativo(n: u32) -> u64 {
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    let mut resultado: u64 = 0;

    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    for _ in 2..=n {
        resultado = a + b;
        a = b;
        b = resultado;
    }

    resultado
}

fn main() -> ExitCode {
    const N: u32 = 45;

    let resultado = fibonacci_iterativo(N);
    println!("El {}-ésimo número de Fibonacci (iterativo) es: {}", N, resultado);

    if N > 90 {
        eprintln!(
            "Advertencia: N > 90. El resultado ({}) probablemente es incorrecto por desbordamiento de 'u64'.",
            resultado
        );
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
