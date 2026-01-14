// fichero memostruc.rs
// Este programa gestiona dinámicamente una lista de tareas en memoria
use std::process::ExitCode;

#[derive(Default)]
struct Tarea {
    id: i32,
    nombre: String,
}
fn main() -> ExitCode {
    let n_tareas = 10;
    
    let mut lista_tareas: Vec<Tarea> = vec![Tarea::default(); n_tareas];

    lista_tareas[0] = Tarea { id: 1, nombre: "Comprar".to_string() };
    lista_tareas[1] = Tarea { id: 2, nombre: "Estudiar".to_string() };
    lista_tareas[2] = Tarea { id: 3, nombre: "Correr".to_string() };

    println!("Lista de Tareas:");
    for (i, tarea) in lista_tareas.iter().enumerate() {
        if tarea.id != 0 {
            println!(" - Tarea {}: {}", tarea.id, tarea.nombre);
        } else {
            println!(" - Tarea {}: <No asignada>", i + 1);
        }
    }

    println!("Memoria de estructuras liberada correctamente.");
    ExitCode::SUCCESS
}
