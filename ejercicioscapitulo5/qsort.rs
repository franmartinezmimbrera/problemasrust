// fichero qsort.rs
// Implementación del algoritmo Quicksort

use std::process::ExitCode;

fn particion<T: Ord>(arr: &mut Vec<T>, bajo: isize, alto: isize) -> isize {
    let pivote = arr[alto as usize].clone();
    let mut i = bajo - 1;

    for j in bajo..alto {
        if arr[j as usize] <= pivote {
            i += 1;
            arr.swap(i as usize, j as usize);
        }
    }
    arr.swap((i + 1) as usize, alto as usize);
    i + 1
}

fn quicksort<T: Ord>(arr: &mut Vec<T>, bajo: isize, alto: isize) {
    if bajo < alto {
        let pi = particion(arr, bajo, alto);
        quicksort(arr, bajo, pi - 1);
        quicksort(arr, pi + 1, alto);
    }
}

fn main() -> ExitCode {
    let mut datos = vec![10, 70, 8, 90, 1000, 5];

    println!("Vector original: ");
    for x in &datos {
        print!("{} ", x);
    }
    println!();

    let n = datos.len();
    if n > 0 {
        quicksort(&mut datos, 0, (n - 1) as isize);
    }

    println!("Vector ordenado con Quicksort:");
    for x in &datos {
        print!("{} ", x);
    }
    println!();

    ExitCode::SUCCESS
}
