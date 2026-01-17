// Archivo: contador_arc.rs
// Descripción: Uso de Arc para compartir un Mutex entre múltiples hilos.
use std::sync::{Arc, Mutex};
use std::thread;
use std::process::ExitCode;
fn main() -> ExitCode {
    // Arc (Atomic Reference Counting) permite tener múltiples dueños del Mutex
    let contador = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let contador_clon = Arc::clone(&contador);
        let handle = thread::spawn(move || {
            // Bloqueamos el mutex para incrementar
            let mut num = contador_clon.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    // Adquirimos el bloqueo una última vez para leer el resultado
    let resultado = *contador.lock().unwrap();
    println!("Resultado final (esperado 10): {}", resultado);

    if resultado == 10 { ExitCode::SUCCESS } else { ExitCode::FAILURE }
}
