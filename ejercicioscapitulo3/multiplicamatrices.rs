// fichero: multiplicamatrices.rs
// Multiplica dos matrices cuadradas 3x3 y muestra el resultado

const N_SIZE: usize = 3;

fn multiplicar_matrices(a: [[i32; N_SIZE]; N_SIZE], b: [[i32; N_SIZE]; N_SIZE]) -> [[i32; N_SIZE]; N_SIZE] {
    let mut c = [[0; N_SIZE]; N_SIZE];

    for i in 0..N_SIZE {
        for j in 0..N_SIZE {
            for k in 0..N_SIZE {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    c
}

fn imprimir_matriz(matriz: &[[i32; N_SIZE]; N_SIZE]) {
    for fila in matriz.iter() {
        for valor in fila.iter() {
            print!("{:4} ", valor);
        }
        println!();
    }
}

fn main() -> ExitCode {
    let a: [[i32; N_SIZE]; N_SIZE] = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];

    let b: [[i32; N_SIZE]; N_SIZE] = [
        [9, 8, 7],
        [6, 5, 4],
        [3, 2, 1],
    ];

    let c = multiplicar_matrices(a, b);

    println!("Matriz A ({}x{}):", N_SIZE, N_SIZE);
    imprimir_matriz(&a);

    println!("\nMatriz B ({}x{}):", N_SIZE, N_SIZE);
    imprimir_matriz(&b);

    println!("\nMatriz Resultado C = A * B:");
    imprimir_matriz(&c);

    ExitCode::SUCCESS
}
