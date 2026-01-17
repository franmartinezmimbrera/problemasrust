// Archivo: local_storage.rs
// Descripción: Variables estáticas que son únicas para cada hilo.

use std::cell::RefCell;
use std::thread;
use std::process::ExitCode;

// Definimos una variable local al hilo
thread_local! {
    // RefCell permite mutabilidad interior
    pub static CONTADOR_LOCAL: RefCell<u32> = RefCell::new(0);
}

fn main() -> ExitCode {
    let handle = thread::spawn(|| {
        // En este hilo, modificamos SU copia de CONTADOR_LOCAL
        CONTADOR_LOCAL.with(|c| {
            *c.borrow_mut() = 55;
            println!("Hilo secundario: Contador = {}", *c.borrow());
        });
    });

    handle.join().unwrap();

    // En el hilo principal, el valor sigue siendo 0 (independiente)
    CONTADOR_LOCAL.with(|c| {
        println!("Hilo Main: Contador = {}", *c.borrow());
    });

    ExitCode::SUCCESS
}
