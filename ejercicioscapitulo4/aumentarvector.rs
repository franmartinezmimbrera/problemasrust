// fichero aumentavector.rs
// Este ejercicio muestra cómo redimensionar dinámicamente un vector en Rust
use std::process::ExitCode;

fn main() -> ExitCode {
    let n_inicial: usize = 3;
    let n_nuevo: usize = 5;

    let mut vector: Vec<i32> = Vec::with_capacity(n_nuevo);

    for i in 0..n_inicial {
        vector.push(i as i32 + 10);
    }
    println!("\nVector inicial ({} elementos):", n_inicial);
    for v in &vector {
        print!("{} ", v);
    }
    println!();
    for i in n_inicial..n_nuevo {
        vector.push(i as i32 + 100);
    }
    println!("\nVector redimensionado ({} elementos):", n_nuevo);
    for v in &vector {
        print!("{} ", v);
    }
    println!();
    println!("\nMemoria liberada correctamente.");
    ExitCode::SUCCESS
}
