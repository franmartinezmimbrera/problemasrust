// fichero: parimpar.rs
// Este programa dice si un número es par o impar
use std::io;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut entrada = String::new();
    println!("Introduzca un número entero:");
    if io::stdin().read_line(&mut entrada).is_err() {
        eprintln!("ERROR: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    let numero: i64 = match entrada.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("ERROR: Entrada no válida (no se introdujo un número).");
            return ExitCode::FAILURE;
        }
    };
    if numero % 2 == 0 {
        println!("ES PAR");
    } else {
        println!("ES IMPAR");
    }
    ExitCode::SUCCESS
}
