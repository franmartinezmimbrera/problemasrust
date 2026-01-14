// fichero asignalibera.rs
// Este ejercicio nos enseña a asignar y liberar memoria dinámica a un vector

use std::process::ExitCode;

fn main() -> ExitCode {
    let n: usize = 5;
    let mut vector: Vec<i32> = Vec::new();

    // Intento de reserva de memoria (equivalente a new[])
    if vector.try_reserve(n).is_err() {
        eprintln!("Error: No se pudo asignar la memoria.");
        return ExitCode::FAILURE;
    }

    println!("Vector Dinámico de {} Elementos", n);

    for i in 0..n {
        vector.push((i as i32) * 10);
        print!("{} ", vector[i]);
    }
    println!();

    println!("Memoria liberada correctamente.");

    ExitCode::SUCCESS
}
