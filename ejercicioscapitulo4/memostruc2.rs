// fichero memostruc2.rs
// Este programa escribe y lee una lista de tareas en un archivo binario
use std::fs::OpenOptions;
use std::io::{self, Read, Write, Seek, SeekFrom, BufReader, BufWriter};
use std::process::ExitCode;

#[derive(Debug)]
struct Tarea {
    id: i32,
    nombre: String,
}
const NOMBRE_ARCHIVO: &str = "registroestructura_string.dat";

fn escribir_cadena<W: Write>(writer: &mut W, cadena: &str) -> io::Result<()> {
    let longitud = cadena.len() as u64;
    writer.write_all(&longitud.to_le_bytes())?;
    writer.write_all(cadena.as_bytes())?;
    Ok(())
}

fn leer_cadena<R: Read>(reader: &mut R) -> io::Result<String> {
    let mut len_buf = [0u8; 8];
    reader.read_exact(&mut len_buf)?;
    let len = u64::from_le_bytes(len_buf);

    let mut buffer = vec![0u8; len as usize];
    reader.read_exact(&mut buffer)?;
    let cadena = String::from_utf8_lossy(&buffer).to_string();
    Ok(cadena)
}
fn main() -> ExitCode {
    let n_tareas = 10;
    let rellenas = 3;

    let lista_tareas = vec![
        Tarea { id: 1, nombre: "Comprar".to_string() },
        Tarea { id: 2, nombre: "Estudiar".to_string() },
        Tarea { id: 3, nombre: "Correr".to_string() },
    ];
    // Escribir archivo binario
    let file = match OpenOptions::new().write(true).create(true).open(NOMBRE_ARCHIVO) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error al escribir el archivo binario: {}", e);
            return ExitCode::FAILURE;
        }
    };
    let mut writer = BufWriter::new(file);
    for tarea in &lista_tareas[0..rellenas] {
        if let Err(e) = writer.write_all(&tarea.id.to_le_bytes()) {
            eprintln!("Error al escribir ID: {}", e);
            return ExitCode::FAILURE;
        }
        if let Err(e) = escribir_cadena(&mut writer, &tarea.nombre) {
            eprintln!("Error al escribir nombre: {}", e);
            return ExitCode::FAILURE;
        }
    }
    println!("Vector de escritura liberado automáticamente.");
    let file = match OpenOptions::new().read(true).open(NOMBRE_ARCHIVO) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error al leer el archivo binario: {}", e);
            return ExitCode::FAILURE;
        }
    };
    let mut reader = BufReader::new(file);
    let mut lista_leida: Vec<Tarea> = Vec::new();
    let mut buffer_id = [0u8; 4];

    println!("\nLista de Tareas leídas del archivo:");
    let mut num_registros = 0;

    loop {
        match reader.read_exact(&mut buffer_id) {
            Ok(_) => {
                let id = i32::from_le_bytes(buffer_id);
                match leer_cadena(&mut reader) {
                    Ok(nombre) => {
                        lista_leida.push(Tarea { id, nombre: nombre.clone() });
                        println!(" - Tarea {}: {}", id, nombre);
                        num_registros += 1;
                    }
                    Err(_) => break,
                }
            }
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
            Err(e) => {
                eprintln!("Error de lectura de un registro: {}", e);
                return ExitCode::FAILURE;
            }
        }
    }
    println!("Se leyeron {} registros.", num_registros);
    println!("Vector de lectura liberado automáticamente.");
    ExitCode::SUCCESS
}
