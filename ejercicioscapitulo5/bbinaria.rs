// fichero bbinaria.rs
// Ejemplo de búsqueda binaria en un array ordenado
use std::process::ExitCode;
fn busqueda_binaria<T: PartialOrd + Copy>(arr: &[T], objetivo: T) -> Option<usize> {
    let mut bajo = 0;
    let mut alto = arr.len().wrapping_sub(1);
    while bajo <= alto {
        let medio = bajo + (alto - bajo) / 2;
        if arr[medio] == objetivo {
            return Some(medio);
        } else if arr[medio] < objetivo {
            bajo = medio + 1;
        } else {
            if medio == 0 {
                break;
            }
            alto = medio - 1;
        }
    }
    None
}
fn main() -> ExitCode {
    let datos_ordenados = vec![5, 8, 12, 15, 20, 30, 40];
    let objetivo1 = 20;
    let objetivo2 = 25;
    println!("Conjunto de datos (ordenado): {:?}", datos_ordenados);
    match busqueda_binaria(&datos_ordenados, objetivo1) {
        Some(idx) => println!("Resultado para {}: Encontrado en el índice {}.", objetivo1, idx),
        None => println!("Resultado para {}: No encontrado.", objetivo1),
    }
    match busqueda_binaria(&datos_ordenados, objetivo2) {
        Some(idx) => println!("Resultado para {}: Encontrado en el índice {}.", objetivo2, idx),
        None => println!("Resultado para {}: No encontrado.", objetivo2),
    }
    ExitCode::SUCCESS
}
