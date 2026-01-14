// fichero burbuja.rs
// Ordenación burbuja (Bubble Sort)
use std::process::ExitCode;
fn ordenar_burbuja<T: Ord>(vec: &mut Vec<T>) {
    let n = vec.len();
    if n < 2 {return;}
    for i in 0..n - 1 {
        for j in 0..n - 1 - i {
            if vec[j] > vec[j + 1] {vec.swap(j, j + 1);}
        }
    }
}
fn main() -> ExitCode {
    let mut numeros = vec![10, 7, 8, 9, 1, 5];
    print!("\nConjunto original: ");
    for x in &numeros {
        print!("{} ", x);
    }
    ordenar_burbuja(&mut numeros);
    print!("\nConjunto ordenado (Bubble Sort): ");
    for x in &numeros {
        print!("{} ", x);
    }
    println!();
    ExitCode::SUCCESS
}
