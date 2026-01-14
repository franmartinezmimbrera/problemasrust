// fichero radixsort.rs
// Implementación del algoritmo Radix Sort
use std::process::ExitCode;

fn get_max(arr: &[u32]) -> u32 {
    *arr.iter().max().unwrap_or(&0)
}
fn counting_sort(arr: &mut Vec<u32>, exp: u32) {
    let mut output = vec![0u32; arr.len()];
    let mut count = vec![0usize; 10];
    for &num in arr.iter() {
        let digit = (num / exp % 10) as usize;
        count[digit] += 1;
    }
    for i in 1..10 {
        count[i] += count[i - 1];
    }
    for &num in arr.iter().rev() {
        let digit = (num / exp % 10) as usize;
        count[digit] -= 1;
        output[count[digit]] = num;
    }
    *arr = output;
}

fn radix_sort(arr: &mut Vec<u32>) {
    if arr.is_empty() {
        return;
    }
    let max = get_max(arr);
    let mut exp = 1;
    while max / exp > 0 {
        counting_sort(arr, exp);
        exp *= 10;
    }
}
fn main() -> ExitCode {
    let mut arr = vec![170, 45, 75, 90, 802, 24, 2, 66];
    println!("Array original:");
    for x in &arr {
        print!("{} ", x);
    }
    println!();
    radix_sort(&mut arr);
    println!("\nArray ordenado:");
    for x in &arr {
        print!("{} ", x);
    }
    println!();
    ExitCode::SUCCESS
}
