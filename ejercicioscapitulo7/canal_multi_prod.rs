// Archivo: canal_multi_prod.rs
// Descripción: Múltiples hilos enviando datos al mismo receptor.
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::process::ExitCode;
fn main() -> ExitCode {
    let (tx, rx) = mpsc::channel();
    // Creamos 3 productores
    for i in 1..=3 {
        // Para que varios hilos tengan el transmisor, debemos clonarlo.
        let tx_clon = tx.clone();
        thread::spawn(move || {
            for j in 1..=3 {
                let msg = format!("Hilo {} - Mensaje {}", i, j);
                tx_clon.send(msg).unwrap();
                thread::sleep(Duration::from_millis(150 * i));
            }
        });
    }
    // Es importante soltar (drop) el tx original del main.
    // Si no lo hacemos, el receptor 'rx' nunca sabrá que se acabaron
    // los emisores y el bucle for de abajo se quedaría esperando para siempre.
    drop(tx);
    for recibido in rx {
        println!("Recibido: {}", recibido);
    }
    ExitCode::SUCCESS
}
