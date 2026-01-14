// archivo: capturar.rs
// Este programa hace preguntas y con ello hace respuestas

use std::io;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut entrada = String::new();
    let (edad, peso): (i32, f32);
    let color: String;

    println!("Dinos tu Edad, peso y color favorito:");

    println!("\n       Edad: ");
    entrada.clear();
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("Error: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    edad = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: La edad debe ser un número entero.");
            return ExitCode::FAILURE;
        }
    };

    println!("\n       Peso: ");
    entrada.clear();
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("Error: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    peso = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El peso debe ser un número.");
            return ExitCode::FAILURE;
        }
    };

    println!("\n Color favorito: ");
    entrada.clear();
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("Error: No se pudo leer el color.");
        return ExitCode::FAILURE;
    }
    color = entrada.trim().to_string();

    println!("¡El {}!!!", color);
    println!(
        "¿Cómo puede gustarte el {} con {} años y pesando {}Kg.?",
        color, edad, peso
    );

    ExitCode::SUCCESS
}
