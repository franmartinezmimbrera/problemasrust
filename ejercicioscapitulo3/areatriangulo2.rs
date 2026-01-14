// archivo: areatriangulo2.rs
// Este programa calcula el área de un triángulo con la fórmula de Herón
use std::io;
use std::process::ExitCode;

fn area_triangulo_heron(l1: f64, l2: f64, l3: f64) -> f64 {
    if l1 <= 0.0 || l2 <= 0.0 || l3 <= 0.0 ||
       l1 + l2 <= l3 || l1 + l3 <= l2 || l2 + l3 <= l1 {
        return 0.0;
    }
    let sp = (l1 + l2 + l3) / 2.0;
    let area = (sp * (sp - l1) * (sp - l2) * (sp - l3)).sqrt();

    area
}

fn main() -> ExitCode {
    let mut entrada = String::new();
    let (l1x, l2x, l3x): (f64, f64, f64);
    println!("Introduce lo que mide el primer lado del triángulo:");
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("ERROR: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    l1x = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("ERROR: Entrada no válida (no es un número).");
            return ExitCode::FAILURE;
        }
    };
    entrada.clear();
    println!("Introduce lo que mide el segundo lado del triángulo:");
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("ERROR: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    l2x = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("ERROR: Entrada no válida (no es un número).");
            return ExitCode::FAILURE;
        }
    };
    entrada.clear();
    println!("Introduce lo que mide el tercer lado del triángulo:");
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("ERROR: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    l3x = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("ERROR: Entrada no válida (no es un número).");
            return ExitCode::FAILURE;
        }
    };
    let area_final = area_triangulo_heron(l1x, l2x, l3x);
    if area_final > 0.0 {
        println!("El área del triángulo es: {}", area_final);
    } else {
        println!("ERROR: Los lados introducidos NO forman un triángulo válido (la suma de dos lados debe ser mayor que el tercero, y todos deben ser positivos).");
    }
    ExitCode::SUCCESS
}
