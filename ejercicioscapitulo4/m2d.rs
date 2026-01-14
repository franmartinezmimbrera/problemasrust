// fichero m2d.rs
// Ejercicio de ejemplo para crear Matriz 2D Dinámica en Rust

use std::process::ExitCode;

fn main() -> ExitCode {
    let m = 3;
    let n = 4;

    let mut matriz: Vec<Vec<i32>> = Vec::with_capacity(m);

    for i in 0..m {
        let mut fila = Vec::with_capacity(n);
        for j in 0..n {
            fila.push((i * n + j + 1) as i32);
        }
        matriz.push(fila);
    }

    println!("\nMatriz Dinámica {}x{}", m, n);
    for fila in &matriz {
        for valor in fila {
            print!("{:3} ", valor);
        }
        println!();
    }

    println!("Memoria de la matriz liberada correctamente.");

    ExitCode::SUCCESS
}
