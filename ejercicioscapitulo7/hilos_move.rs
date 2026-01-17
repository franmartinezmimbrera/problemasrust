// Archivo: hilos_move.rs
// Descripción: Uso de la palabra clave 'move' para transferir propiedad de datos.

use std::thread;
use std::process::ExitCode;

fn main() -> ExitCode {
    let datos = vec![1, 2, 3, 4, 5];

    println!("Main: Datos originales: {:?}", datos);

    // Intentar usar 'datos' dentro del hilo sin 'move' daría error,
    // porque Rust no sabe cuánto tiempo vivirá 'datos' en el main
    // respecto a la vida del hilo.
    let handle = thread::spawn(move || {
        // 'move' transfiere la propiedad de 'datos' a este hilo.
        println!("Hilo: He recibido los datos: {:?}", datos);
        println!("Hilo: La longitud es: {}", datos.len());
        // Aquí termina la vida de 'datos'
    });

    // Si intentáramos usar 'datos' aquí, daría error de compilación
    // porque se ha movido al hilo.
    // println!("Main: {:?}", datos); // <--- Esto fallaría
    if handle.join().is_err() {
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
