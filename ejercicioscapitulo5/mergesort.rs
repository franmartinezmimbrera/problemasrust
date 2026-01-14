// fichero mergesort.rs
// Implementación del algoritmo MergeSort
use std::process::ExitCode;
fn merge<T: Ord + Clone>(arr: &mut Vec<T>, inicio: usize, medio: usize, fin: usize) {
    let mut temp: Vec<T> = Vec::with_capacity(fin - inicio + 1);

    let mut i = inicio;
    let mut j = medio + 1;

    while i <= medio && j <= fin {
        if arr[i] <= arr[j] {
            temp.push(arr[i].clone());
            i += 1;
        } else {
            temp.push(arr[j].clone());
            j += 1;
        }
    }
    while i <= medio {
        temp.push(arr[i].clone());
        i += 1;
    }
    while j <= fin {
        temp.push(arr[j].clone());
        j += 1;
    }
    for (k, val) in temp.into_iter().enumerate() {
        arr[inicio + k] = val;
    }
}
fn merge_sort<T: Ord + Clone>(arr: &mut Vec<T>, inicio: usize, fin: usize) {
    if inicio < fin {
        let medio = inicio + (fin - inicio) / 2;
        merge_sort(arr, inicio, medio);
        merge_sort(arr, medio + 1, fin);
        merge(arr, inicio, medio, fin);
    }
}
fn main() -> ExitCode {
    let mut datos = vec![38, 27, 43, 3, 9, 82, 10];

    println!("Vector original desordenado:");
    for x in &datos {
        print!("{} ", x);
    }
    println!();

    let n = datos.len();
    if n > 0 {
        merge_sort(&mut datos, 0, n - 1);
    }

    println!("\nVector ordenado con MergeSort:");
    for x in &datos {
        print!("{} ", x);
    }
    println!();

    ExitCode::SUCCESS
}
