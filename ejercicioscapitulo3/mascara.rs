// fichero: mascara.rs
// Invierte (cambia 0->1 y 1->0) los últimos k bits de un número entero

use std::process::ExitCode;

fn invertir_ultimos_k_bits(numero: i64, k: u32) -> i64 {
    let tamano_ll = std::mem::size_of::<i64>() as u32 * 8;

    if k == 0 {
        return numero;
    }

    if k >= tamano_ll {
        return !numero;
    }

    let mascara = (1_u64 << k) - 1;
    (numero as u64 ^ mascara) as i64
}

fn main() -> ExitCode {
    let num: i64 = 45;
    let k: u32 = 4;

    let resultado = invertir_ultimos_k_bits(num, k);

    println!("Número original: {} (Binario: {:08b})", num, num);
    println!("K: {}", k);
    println!("Número resultante: {} (Binario: {:08b})", resultado, resultado);

    ExitCode::SUCCESS
}
