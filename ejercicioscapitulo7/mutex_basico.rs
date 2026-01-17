// Archivo: mutex_basico.rs
// Descripción: Uso básico de Mutex para proteger datos en un contexto simple.

use std::sync::Mutex;
use std::process::ExitCode;

fn main() -> ExitCode {
    // Creamos un Mutex que protege un entero
    let m = Mutex::new(5);

    {
        // Para acceder al dato, debemos bloquear (lock) el mutex.
        // Esto devuelve un MutexGuard inteligente.
        let mut num = m.lock().unwrap();
        
        println!("Valor original protegido: {}", num);
        
        *num = 10; // Modificamos el valor a través de la referencia
        
        // El bloqueo se libera automáticamente aquí, cuando 'num' sale del scope.
    }

    println!("Bloqueo liberado. Nuevo valor: {:?}", m);
    
    ExitCode::SUCCESS
}
