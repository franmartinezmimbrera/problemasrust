// archivo: alturatriangulo.rs
// Este programa calcula la altura de un triángulo equilátero
use std::f64;
use std::io;
use std::process::ExitCode;
fn main() -> ExitCode {
    println!("Introduzca lado de un triángulo equilátero: ");
    let mut entrada = String::new();
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("Error: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    let l: f64 = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El valor debe ser un número.");
            return ExitCode::FAILURE;
        }
    };
    if l <= 0.0 {
        println!("Error: El lado de un triángulo equilátero debe ser un número mayor que 0.");
        return ExitCode::FAILURE;
    }
    let h = (f64::sqrt(3.0) * l) / 2.0;
    println!(
        "La altura de un triángulo equilátero de lado {} es: {}",
        l, h
    );
    ExitCode::SUCCESS
}
