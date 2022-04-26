use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::grid::Grid;

/**
Imprime la diferencia entre dos archivos de texto, usando un algoritmo para encontrar
la mayor subsecuencia de lineas en común.

# Argumentos

* `first_path` - Un slice de string que contiene el path al primer archivo a comparar
* `second_path` - Un slice de string que contiene el path al segundo archivo a comparar

# Errores

Esta función devolverá un error si falla la lectura de los archivos con `read_file_lines()`

# Ejemplos

```no_run
if let Err(error) = diff::lcs::diff("some_file.txt", "other_file.txt"){
    // Manejamos el error
    println!("Error al generar diff! {}", error);
}
```
*/
pub fn diff(first_path: &str, second_path: &str) -> std::io::Result<()> {
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

/**
Función que dada la ruta de un archivo de texto devuelve sus líneas
en un vector de Strings.

# Argumentos

* `path` - Un slice de string que contiene el path a un archivo

# Errores

Esta función devolverá un error si falla la apertura del archivo, por
ejemplo, si no existe.

# Ejemplos

```ignore
let lines = match diff::lcs::read_file_lines("some_file.txt"){
    Ok(lines) => lines,
    Err(e) => {
        println!("Error en la apertura del archivo {}", e);
        // Manejamos el error
    }
};
```
*/
fn read_file_lines(path: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(path)?;
    let buffer = BufReader::new(file);
    buffer.lines().collect()
}

/**
Algoritmo de LCS que devuelve un Grid con la información para poder reconstruir
la diferencia entre las dos secuencias. Esta función por si misma no tiene sentido
sin llamar a alguna que sepa encontrar la diferencia dado el grid y las secuencias.

# Argumentos

* `first_sequence` - Un slice de array que contiene la primera secuencia de strings
* `second_sequence` - Un slice de array que contiene la segunda secuencia de strings

# Ejemplos

```ignore
let first = vec!["a", "b", "c"];
let second = vec!["a", "c", "b"];
let diff_grid = lcs(&first, &second);
// Usamos el grid para reconstruir la diferencia.
```
*/
fn lcs(first_sequence: &[String], second_sequence: &[String]) -> Grid {
    let m = first_sequence.len();
    let n = second_sequence.len();
    let mut grid = Grid::new(m + 1, n + 1);

    for (i, first_item) in first_sequence.iter().enumerate() {
        for (j, second_item) in second_sequence.iter().enumerate() {
            if first_item == second_item {
                grid.insert(i + 1, j + 1, grid.get(i, j) + 1);
            } else {
                grid.insert(
                    i + 1,
                    j + 1,
                    std::cmp::max(grid.get(i + 1, j), grid.get(i, j + 1)),
                );
            }
        }
    }
    grid
}

/**
Funcion recursiva que imprime la diferencia entre dos secuencias
de strings dada la información de un Grid, dependiendo de si son
strings en común, o si se deben quitar de alguna secuencia para
hacerlas coincidir.

# Argumentos

* `grid` - Grid resultado de la aplicación de un algoritmo de LCS sobre dos secuencias
* `first_sequence` - Slice de array que contiene la primera de las secuencias a comparar
* `second_sequence` - Slice de array que contiene la segunda secuencia a comparar
* `i`, `j` - Usize que representan la cantidad de elementos de la primera y segunda secuencia respectivamente

# Ejemplos
```ignore
let first = vec!["a", "b", "c", "d", "a", "f"];
let second = vec!["a", "c", "b", "c", "f"];
// Notemos que la mayor subsecuencia en común es a, b, c, f
// Y que deberíamos quitar "d, a" de la primera y "c" de la segunda
let diff_grid = lcs(&first, &second);
print_diff(
grid,
&first_file_lines,
&second_file_lines,
first_file_lines.iter().len(),
second_file_lines.iter().len(),
);
```
*/

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
