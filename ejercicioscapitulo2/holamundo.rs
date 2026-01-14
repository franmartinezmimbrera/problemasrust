// archivo: holamundo.rs
// Este programa muestra el saludo hola mundo por pantalla.
use std::process::ExitCode;
fn main() -> ExitCode {
    println!("Hola Mundo");
    ExitCode::SUCCESS
}
