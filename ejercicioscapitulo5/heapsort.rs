// fichero heapsort.rs
// Implementación del algoritmo Heapsort

use std::process::ExitCode;

fn heapify<T: Ord>(arr: &mut Vec<T>, n: usize, i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && arr[left] > arr[largest] {
        largest = left;
    }

    if right < n && arr[right] > arr[largest] {
        largest = right;
    }

    if largest != i {
        arr.swap(i, largest);
        heapify(arr, n, largest);
    }
}

fn heap_sort<T: Ord>(arr: &mut Vec<T>) {
    let n = arr.len();

    for i in (0..n / 2).rev() {
        heapify(arr, n, i);
    }

    for i in (1..n).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

fn main() -> ExitCode {
    let mut arr = vec![12, 11, 13, 5, 6, 7];

    println!("Array original:");
    for x in &arr {
        print!("{} ", x);
    }
    println!();

    heap_sort(&mut arr);

    println!("Array ordenado con Heapsort:");
    for x in &arr {
        print!("{} ", x);
    }
    println!();

    ExitCode::SUCCESS
}
