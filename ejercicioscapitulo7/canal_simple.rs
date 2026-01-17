// Archivo: canal_simple.rs
// Descripción: Paso de mensajes básico entre dos hilos.
use std::sync::mpsc; // mpsc: Multi-Producer, Single-Consumer
use std::thread;
use std::time::Duration;
use std::process::ExitCode;
fn main() -> ExitCode {
    // tx = transmisor, rx = receptor
    let (tx, rx) = mpsc::channel();
    // Hilo Productor
    thread::spawn(move || {
        let mensajes = vec!["Hola", "desde", "el", "otro", "lado"];¡
        for msg in mensajes {
            println!("Productor: Enviando '{}'", msg);
            // send devuelve un Result. Si el receptor se cierra, da error.
            tx.send(msg).unwrap(); 
            thread::sleep(Duration::from_millis(200));
        }
        // tx se cierra automáticamente al salir del ámbito
    });
    // Hilo Consumidor (Main)
    println!("Consumidor: Esperando mensajes...");
    // Iterar sobre 'rx' bloquea el hilo esperando mensajes hasta que tx se cierre
    for recibido in rx {
        println!("Consumidor: Recibido '{}'", recibido);
    }
    println!("Consumidor: Canal cerrado, no hay más mensajes.");
    ExitCode::SUCCESS
}
