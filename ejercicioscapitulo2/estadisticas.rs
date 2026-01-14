// archivo: estadisticas.rs
// Este programa calcula estadísticas sobre alumnos

use std::io;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut entrada = String::new();
    let (suspensos, aprobados, notables, sobresalientes): (i32, i32, i32, i32);
    let (poraprot, porsus, pornot, porsobre, porapro, totalalumnos): (f64, f64, f64, f64, f64, f64);

    // Leer suspensos
    println!("Introduce el número total de suspensos:");
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("Error: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    suspensos = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El valor debe de ser un número.");
            return ExitCode::FAILURE;
        }
    };
    if suspensos < 0 {
        println!("Error: El valor debe ser un número positivo");
        return ExitCode::FAILURE;
    }

    // Leer aprobados
    entrada.clear();
    println!("Introduce el número total de aprobados:");
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("Error: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    aprobados = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El valor debe de ser un número.");
            return ExitCode::FAILURE;
        }
    };
    if aprobados < 0 {
        println!("Error: El valor debe ser un número positivo");
        return ExitCode::FAILURE;
    }

    entrada.clear();
    println!("Introduce el número total de notables:");
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("Error: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    notables = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El valor debe de ser un número.");
            return ExitCode::FAILURE;
        }
    };
    if notables < 0 {
        println!("Error: El valor debe ser un número positivo");
        return ExitCode::FAILURE;
    }

    entrada.clear();
    println!("Introduce el número total de sobresalientes:");
    if io::stdin().read_line(&mut entrada).is_err() {
        println!("Error: No se pudo leer la entrada.");
        return ExitCode::FAILURE;
    }
    sobresalientes = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El valor debe de ser un número.");
            return ExitCode::FAILURE;
        }
    };
    if sobresalientes < 0 {
        println!("Error: El valor debe ser un número positivo");
        return ExitCode::FAILURE;
    }

    totalalumnos = (suspensos + aprobados + notables + sobresalientes) as f64;

    if totalalumnos == 0.0 {
        println!("No se han introducido alumnos (Total 0), no se pueden calcular estadísticas.");
        return ExitCode::FAILURE;
    }

    poraprot = ((aprobados + notables + sobresalientes) as f64 / totalalumnos) * 100.0;
    porsus = (suspensos as f64 / totalalumnos) * 100.0;
    pornot = (notables as f64 / totalalumnos) * 100.0;
    porsobre = (sobresalientes as f64 / totalalumnos) * 100.0;
    porapro = (aprobados as f64 / totalalumnos) * 100.0;

    println!(
        "El porcentaje de alumnos que superan la asignatura es: {:.2}",
        poraprot
    );
    println!(
        "El porcentaje de alumnos que suspenden la asignatura es: {:.2}",
        porsus
    );
    println!(
        "El porcentaje de alumnos que sacan notable en la asignatura es: {:.2}",
        pornot
    );
    println!(
        "El porcentaje de alumnos que sobresalen en la asignatura es: {:.2}",
        porsobre
    );
    println!(
        "El porcentaje de alumnos que han sacado un aprobado en la asignatura es: {:.2}",
        porapro
    );

    ExitCode::SUCCESS
}
