// fichero blineal.rs
// Ejemplo de búsqueda lineal en un array
use std::process::ExitCode;
fn busqueda_lineal<T: PartialEq>(arr: &[T], objetivo: T) -> Option<usize> {
    arr.iter().position(|x| *x == objetivo)
}
fn main() -> ExitCode {
    let datos = vec![10, 5, 20, 15, 8, 30];
    let objetivo1 = 15;
    print!("Conjunto de datos: {{");
    for (i, x) in datos.iter().enumerate() {
        print!("{}", x);
        if i < datos.len() - 1 {print!(", ");}
    }
    println!("}}");
    match busqueda_lineal(&datos, objetivo1) {
        Some(indice) => println!("Resultado para {}: Encontrado en índice {}.", objetivo1, indice),
        None => println!("Resultado para {}: No encontrado", objetivo1),
    }
    ExitCode::SUCCESS
}
