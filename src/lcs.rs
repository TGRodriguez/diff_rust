use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::grid::Grid;

pub fn diff(first_path: &str, second_path: &str) -> std::io::Result<()>{
    let first_file_lines = read_file_lines(first_path)?;
    let second_file_lines = read_file_lines(second_path)?;
    let grid = lcs(&first_file_lines, &second_file_lines);
    print_diff(
        grid,
        &first_file_lines,
        &second_file_lines,
        first_file_lines.iter().len(),
        second_file_lines.iter().len(),
    );
    Ok(())
}

/// Función que dada la ruta de un archivo de texto devuelve sus líneas
/// en un vector de Strings.
///
/// Recibe un slice de la ruta del archivo, lo abre, vuelca sus lineas
/// en un buffer y las colecciona en un vector de Strings. Devuelve un
/// Result<Vec<String>> porque la apertura y la lectura de lineas puede fallar.
fn read_file_lines(path: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(path)?;
    let buffer = BufReader::new(file);
    buffer.lines().collect()
}

/// Encuentra la mayor subsecuencia en común entre dos secuencias de lineas
/// y guarda la información en un Grid.
///
/// Recibe las dos secuencias como referencias a vectores de Strings,
/// aplica el algoritmo de LCS y devuelve un Grid con el que ya se puede
/// conseguir cual es la subsecuencia, que lineas se deben agregar y cuales
/// se deben quitar para hacer coincidir las secuencias.
fn lcs(first_sequence: &[String], second_sequence: &[String]) -> Grid {
    let m = first_sequence.len();
    let n = second_sequence.len();
    let mut grid = Grid::new(m+1, n+1);

    for (i, first_item) in first_sequence.iter().enumerate(){
        for (j, second_item) in second_sequence.iter().enumerate(){
            if first_item == second_item{
                grid.insert(i+1, j+1,grid.get(i, j) +1);
            } else {
                grid.insert(i+1, j+1, std::cmp::max(grid.get(i+1, j), grid.get(i, j+1)));
            }

        }
    }
    grid
}

/// Funcion recursiva que imprime la diferencia entre dos secuencias
/// de lineas dada la información de un Grid.
///
/// Recibe un Grid, dos referencias a las secuencias, dos usize para
/// leer las filas y columnas del Grid e imprime las subsecuencias
/// de distinta forma dependiendo si son comunes a ambas secuencias
/// o no.
fn print_diff(
    grid: Grid,
    first_sequence: &[String],
    second_sequence: &[String],
    i: usize,
    j: usize,
) {
    if i > 0 && j > 0 && first_sequence[i - 1] == second_sequence[j - 1] {
        print_diff(grid, first_sequence, second_sequence, i - 1, j - 1);
        println!("{}", first_sequence[i - 1]);
    } else if j > 0 && (i == 0 || grid.get(i, j - 1) >= grid.get(i - 1, j)) {
        print_diff(grid, first_sequence, second_sequence, i, j - 1);
        println!("> {}", second_sequence[j - 1]);
    } else if i > 0 && (j == 0 || grid.get(i, j - 1) < grid.get(i - 1, j)) {
        print_diff(grid, first_sequence, second_sequence, i - 1, j);
        println!("< {}", first_sequence[i - 1]);
    } else {
        println!();
    }
}
