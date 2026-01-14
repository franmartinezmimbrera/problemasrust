// archivo: perimetrocir.rs
// Este programa calcula el perímetro de una circunferencia
use std::io;
use std::process::ExitCode;
const PI: f64 = 3.14159265358979323846;
fn main() -> ExitCode {
    let mut entrada = String::new();
    let (radio, perimetro): (f64, f64);
    println!("Introduzca el radio:");
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("Error: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    radio = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El valor debe de ser un número.");
            return ExitCode::FAILURE;
        }
    };
    if radio <= 0.0 {
        println!("Error: El radio debe ser un número >= 0");
        return ExitCode::FAILURE;
    }
    perimetro = 2.0 * PI * radio;
    println!("El perímetro es: {}", perimetro);
    ExitCode::SUCCESS
}
