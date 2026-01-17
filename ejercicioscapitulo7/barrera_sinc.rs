// Archivo: barrera_sinc.rs
// Descripción: Barrier obliga a los hilos a esperar hasta que todos lleguen al punto.

use std::sync::{Arc, Barrier};
use std::thread;
use std::process::ExitCode;

fn main() -> ExitCode {

    let n_hilos = 5;
    
    // La barrera se abrirá cuando 'n_hilos' llamen a wait()
    let barrera = Arc::new(Barrier::new(n_hilos));
    let mut handles = vec![];

    for i in 0..n_hilos {
        let b = Arc::clone(&barrera);
        handles.push(thread::spawn(move || {
            println!("Hilo {}: Trabajando...", i);
            // Cada hilo tarda un tiempo diferente
            thread::sleep(std::time::Duration::from_millis(100 * i as u64));
            
            println!("Hilo {}: Esperando en barrera...", i);
            // Aquí se detienen hasta que los 5 lleguen
            b.wait(); 
            
            println!("Hilo {}: ¡Barrera superada!", i);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    ExitCode::SUCCESS
}
