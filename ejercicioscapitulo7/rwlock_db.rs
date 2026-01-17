// Archivo: rwlock_db.rs
// Descripción: RwLock permite múltiples lectores simultáneos o un solo escritor.

use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use std::process::ExitCode;

fn main() -> ExitCode {

    // Simulamos una base de datos pequeña protegida por RwLock
    let db = Arc::new(RwLock::new(vec!["Dato 1", "Dato 2"]));
    let mut handles = vec![];
    
    // Creamos 5 hilos lectores
    for i in 0..5 {
        let db_r = Arc::clone(&db);
        handles.push(thread::spawn(move || {
            // read() permite acceso concurrente si no hay nadie escribiendo
            let datos = db_r.read().unwrap();
            println!("Lector {}: Leído {:?}", i, *datos);
            thread::sleep(Duration::from_millis(100));
        }));
    }
    
    // Creamos 1 hilo escritor
    let db_w = Arc::clone(&db);
    handles.push(thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        // write() exige exclusividad total
        let mut datos = db_w.write().unwrap();
        datos.push("Dato NUEVO");
        println!("Escritor: Dato insertado.");
    }));
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Estado final de la DB: {:?}", *db.read().unwrap());
    
    ExitCode::SUCCESS
    
}
