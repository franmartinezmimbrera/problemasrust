// fichero binterpola.rs
// Ejemplo de búsqueda por interpolación en un array ordenado
use std::process::ExitCode;
fn busqueda_interpolacion(arr: &[i32], objetivo: i32) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }
    let mut bajo = 0usize;
    let mut alto = arr.len() - 1;

    while bajo <= alto && objetivo >= arr[bajo] && objetivo <= arr[alto] {
        if arr[bajo] == arr[alto] {
            return if arr[bajo] == objetivo { Some(bajo) } else { None };
        }

        let pos = bajo as usize + (((objetivo - arr[bajo]) as usize)
            * (alto - bajo))
            / ((arr[alto] - arr[bajo]) as usize);

        let pos = pos.clamp(bajo, alto);

        if arr[pos] == objetivo {
            return Some(pos);
        } else if arr[pos] < objetivo {
            bajo = pos + 1;
        } else {
            if pos == 0 {
                break;
            }
            alto = pos - 1;
        }
    }

    None
}
fn main() -> ExitCode {
    let datos_ordenados = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    let objetivo1 = 70;
    match busqueda_interpolacion(&datos_ordenados, objetivo1) {
        Some(i) => println!("Resultado para {}: Encontrado en el índice {}.", objetivo1, i),
        None => println!("Resultado para {}: No encontrado.", objetivo1),
    }
    let objetivo2 = 45;
    match busqueda_interpolacion(&datos_ordenados, objetivo2) {
        Some(i) => println!("Resultado para {}: Encontrado en el índice {}.", objetivo2, i),
        None => println!("Resultado para {}: No encontrado.", objetivo2),
    }
    ExitCode::SUCCESS
}
