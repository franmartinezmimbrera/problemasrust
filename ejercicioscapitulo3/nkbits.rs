// fichero: nkbits.rs
// Muestra los últimos k bits de un número entero

use std::process::ExitCode;

fn mostrar_ultimos_k_bits(numero: i32, k: u32) {
    let tamano_int = std::mem::size_of::<i32>() as u32 * 8;

    if k == 0 {
        eprintln!("Error: k debe ser mayor que 0.");
        return ExitCode::FAILURE;
    }
    let k = if k > tamano_int { tamano_int } else { k };

    print!("Los últimos {} bits de {} son: ", k, numero);

    for i in (0..k).rev() {
        let bit = (numero >> i) & 1;
        print!("{}", bit);
    }

    println!();
}

fn main() -> ExitCode {
    let num1 = 45;
    mostrar_ultimos_k_bits(num1, 8);
    ExitCode::SUCCESS
}
