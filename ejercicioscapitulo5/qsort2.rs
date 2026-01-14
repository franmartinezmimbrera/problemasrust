// fichero qsort2.rs
// Ejemplo simple usando el método sort de Rust

use std::process::ExitCode;

fn main() -> ExitCode {
    let mut numeros = vec![50, 10, 8, 20, 40];

    println!("Vector antes de sort:");
    for x in &numeros {
        print!("{} ", x);
    }
    println!();

    numeros.sort();

    println!("Vector después de sort:");
    for x in &numeros {
        print!("{} ", x);
    }
    println!();

    ExitCode::SUCCESS
}
