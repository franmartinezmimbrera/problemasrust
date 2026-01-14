// fichero creabinario.rs
// Crea y lee un fichero binario con un registro de alumno

use std::fs::File;
use std::io::{Read, Write};
use std::process::ExitCode;

const NOMBRE_ARCHIVO: &str = "registro.dat";

struct RegistroAlumno {
    id: i32,
    nombre: String,
    nota: f32,
}

fn crear_binario() -> Result<(), ()> {
    let alumno_original = RegistroAlumno {
        id: 101,
        nombre: String::from("Sofia Perez"),
        nota: 9.5,
    };

    let mut archivo = match File::create(NOMBRE_ARCHIVO) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Error al escribir el archivo binario.");
            return Err(());
        }
    };

    archivo
        .write_all(&alumno_original.id.to_le_bytes())
        .map_err(|_| ())?;

    archivo
        .write_all(&alumno_original.nota.to_le_bytes())
        .map_err(|_| ())?;

    let nombre_len = alumno_original.nombre.len() as u64;
    archivo
        .write_all(&nombre_len.to_le_bytes())
        .map_err(|_| ())?;

    archivo
        .write_all(alumno_original.nombre.as_bytes())
        .map_err(|_| ())?;

    Ok(())
}

fn leer_binario() -> Result<(), ()> {
    let mut archivo = match File::open(NOMBRE_ARCHIVO) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Error al leer el archivo binario.");
            return Err(());
        }
    };

    let mut buffer_i32 = [0u8; 4];
    let mut buffer_f32 = [0u8; 4];
    let mut buffer_u64 = [0u8; 8];

    archivo.read_exact(&mut buffer_i32).map_err(|_| ())?;
    let id = i32::from_le_bytes(buffer_i32);

    archivo.read_exact(&mut buffer_f32).map_err(|_| ())?;
    let nota = f32::from_le_bytes(buffer_f32);

    archivo.read_exact(&mut buffer_u64).map_err(|_| ())?;
    let nombre_len = u64::from_le_bytes(buffer_u64) as usize;

    let mut nombre_bytes = vec![0u8; nombre_len];
    archivo.read_exact(&mut nombre_bytes).map_err(|_| ())?;
    let nombre = String::from_utf8(nombre_bytes).map_err(|_| ())?;

    let alumno_leido = RegistroAlumno { id, nombre, nota };

    println!(
        "Datos cargados del archivo: ID={}, Nombre={}, Nota={}",
        alumno_leido.id, alumno_leido.nombre, alumno_leido.nota
    );

    Ok(())
}

fn main() -> ExitCode {
    if crear_binario().is_err() {
        return ExitCode::FAILURE;
    }

    if leer_binario().is_err() {
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
