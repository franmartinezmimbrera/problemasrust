// fichero multiplo3.rs
// Este programa dice si un número es múltiplo de 3
use std::io;
use std::process::ExitCode;
fn main() -> ExitCode {
    let mut entrada = String::new();
    println!("Introduzca un número:");
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("ERROR: Entrada no válida (no se introdujo un número).");
        return ExitCode::FAILURE;
    }
    let numero: i32 = match entrada.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("ERROR: Entrada no válida (no se introdujo un número).");
            return ExitCode::FAILURE;
        }
    };
    if numero % 3 == 0 && numero % 2 == 0 {
        println!(
            "\nEl número {} es MÚLTIPLO DE 3 Y TAMBIÉN ES PAR (Múltiplo de 6)",
            numero
        );
    } else if numero % 3 == 0 {
        println!("El número {} ES MÚLTIPLO DE 3", numero);
    } else {
        println!("El número {} NO ES MÚLTIPLO DE 3", numero);
    }
    ExitCode::SUCCESS
}
