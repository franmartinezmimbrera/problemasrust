// Archivo: scope_threads.rs
// Descripción: thread::scope permite prestar datos de la pila a los hilos.
use std::thread;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut vector = vec![1, 2, 3];
    let x = 42;

    // thread::scope garantiza que todos los hilos creados dentro
    // terminarán antes de que se cierre el bloque scope.
    // Esto permite usar referencias (&) a variables locales sin 'move' ni 'Arc'.
    thread::scope(|s| {
        // Hilo 1: Lee x (préstamo inmutable)
        s.spawn(|| {
            println!("Hilo 1: Leyendo x = {}", x);
        });

        // Hilo 2: Modifica el vector (préstamo mutable)
        s.spawn(|| {
            println!("Hilo 2: Añadiendo elemento al vector");
            vector.push(4);
        });
    });
    // Aquí los hilos ya han terminado seguro
    println!("Main: Vector modificado: {:?}", vector);

    ExitCode::SUCCESS
}
