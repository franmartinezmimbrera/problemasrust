// fichero shellsort.rs
// Implementación del algoritmo Shellsort usando la secuencia de Knuth

use std::process::ExitCode;

fn shellsort_knuth<T: PartialOrd + Copy>(arr: &mut Vec<T>) {
    let n = arr.len();
    let mut h = 1;

    while h < n / 3 {
        h = 3 * h + 1;
    }

    while h >= 1 {
        for i in h..n {
            let temp = arr[i];
            let mut j = i;

            while j >= h && arr[j - h] > temp {
                arr[j] = arr[j - h];
                j -= h;
            }
            arr[j] = temp;
        }
        h /= 3;
    }
}

fn main() -> ExitCode {
    let mut datos = vec![90, 8, 70, 6, 50, 4, 30, 2, 10, 0, 85, 15, 65, 35];

    println!("Array original desordenado:");
    for x in &datos {
        print!("{} ", x);
    }
    println!();

    shellsort_knuth(&mut datos);

    println!("\nArray ordenado con Shellsort (Knuth):");
    for x in &datos {
        print!("{} ", x);
    }
    println!();

    ExitCode::SUCCESS
}
