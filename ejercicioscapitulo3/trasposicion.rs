// archivo: trasposicion.rs
// Función para trasponer una matriz MxN

use std::process::ExitCode;
use std::io::{self, Write};

const M_FILAS: usize = 3;
const N_COLUMNAS: usize = 4;

fn trasponer_matriz(
    original: &[[i32; N_COLUMNAS]; M_FILAS],
    transpuesta: &mut [[i32; M_FILAS]; N_COLUMNAS],
) {
    for i in 0..M_FILAS {
        for j in 0..N_COLUMNAS {
            transpuesta[j][i] = original[i][j];
        }
    }
}

fn imprimir_matriz<T: std::fmt::Display, const R: usize, const C: usize>(matriz: &[[T; C]; R]) {
    for i in 0..R {
        for j in 0..C {
            print!("{:4}", matriz[i][j]);
        }
        println!();
    }
}

fn main() -> ExitCode {
    let a: [[i32; N_COLUMNAS]; M_FILAS] = [
        [1, 2, 3, 4],
        [5, 6, 7, 8],
        [9, 10, 11, 12],
    ];

    let mut at: [[i32; M_FILAS]; N_COLUMNAS] = [[0; M_FILAS]; N_COLUMNAS];

    println!("Matriz Original A ({} x {}):", M_FILAS, N_COLUMNAS);
    imprimir_matriz(&a);

    trasponer_matriz(&a, &mut at);

    println!("\nMatriz Transpuesta AT ({} x {}):", N_COLUMNAS, M_FILAS);
    imprimir_matriz(&at);

    ExitCode::SUCCESS
}
