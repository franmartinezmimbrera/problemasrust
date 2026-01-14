// archivo: volumencilindro.rs
// Este programa calcula el volumen de un cilindro
use std::io;
use std::process::ExitCode;
const PI: f64 = 3.14159265358979323846;
fn main() -> ExitCode {
    let mut entrada = String::new();
    let (d, h, r, v): (f64, f64, f64, f64);
    println!("Introduzca el diámetro, en metros:");
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("Error: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    d = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El valor debe ser un número.");
            return ExitCode::FAILURE;
        }
    };
    entrada.clear();
    println!("Introduzca la altura, en metros:");
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("Error: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    h = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El valor debe ser un número.");
            return ExitCode::FAILURE;
        }
    };
    if d <= 0.0 || h <= 0.0 {
        println!("Error: El diámetro y la altura debe ser un número > que 0");
        return ExitCode::FAILURE;
    }
    r = d / 2.0;
    v = PI * r * r * h;
    println!("El volumen del cilindro es de {} m^3", v);
    ExitCode::SUCCESS
}
