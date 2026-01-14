// archivo: sumannumerosnaturales.rs
// Este programa calcula la suma de los "n" primeros números naturales
use std::io;
use std::process::ExitCode;
fn main() -> ExitCode {
    let mut entrada = String::new();
    let (n, suma): (i32, i64);
    println!("Introduzca número de números naturales a sumar:");
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("Error: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    n = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El valor debe de ser un número.");
            return ExitCode::FAILURE;
        }
    };
    if n <= 0 {
        println!("Error: El valor debe ser un número natural.");
        return ExitCode::FAILURE;
    }
    suma = (n as i64 * (n as i64 + 1)) / 2;
    println!(
        "La suma de los {} primeros números naturales es: {}",
        n, suma
    );
    ExitCode::SUCCESS
}
