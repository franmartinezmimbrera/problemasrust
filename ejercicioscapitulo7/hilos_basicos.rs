//Archivo: hilos_basicos.rs
//Descripción:Crea varios hilos y espera a que terminen usando join().
use std::thread;
use std::time::Duration;
use std::process::ExitCode;
fn main() -> ExitCode {
    let mut handles = vec![];
    println!("Main: Iniciando el lanzamiento de hilos...");
    for i in 1..=5 {
        // spawn lanza un nuevo hilo. 
        let handle = thread::spawn(move || {
            println!("Hilo {}: Iniciado.", i);
            thread::sleep(Duration::from_millis(100 * i)); // Simulamos trabajo
            println!("Hilo {}: Terminado.", i);
        });
        handles.push(handle);
    }
    println!("Main: Esperando a que terminen los hilos...");
    // Si no hacemos join, el hilo principal terminaría y mataría a los hijos antes de que acaben.
    for handle in handles {
        if handle.join().is_err() {
            eprintln!("Error al esperar a un hilo.");
            return ExitCode::FAILURE;
        }
    }
    println!("Main: Todos los hilos han terminado correctamente.");
    ExitCode::SUCCESS
}
