// archivo: areacuadrado.rs
// Este programa calcula el área de un cuadrado a partir de uno de sus lados
use std::io;
use std::process::ExitCode;
fn main() -> ExitCode {
    println!("Introduce valor lado de un cuadrado: ");
    let mut entrada = String::new();
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("Error: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    let l1: f64 = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El lado debe de ser un número.");
            return ExitCode::FAILURE;
        }
    };

    if l1 <= 0.0 {
        println!("Error: El lado debe ser un número positivo > 0");
        return ExitCode::FAILURE;
    }
    let area = l1 * l1;
    println!("El Área del cuadrado es: {}", area);
    ExitCode::SUCCESS
}
