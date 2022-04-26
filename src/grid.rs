/// Estructura que representa un grilla, similar a una matriz, que almacena enteros sin signar.
///
/// # Campos
///
/// * `columns` - Usize que representa la cantidad de columnas de la grilla usado para acceder a cualquier posición indexando por filas y columnas.
/// * `data` - Vector de usize que contiene los valores de la grilla. Los primeros 'columns' elementos serán los elementos de la primera fila.
/// De esta forma, la posicion (i, j) de la grilla se encuentra en data[(i*columns)+j].
pub struct Grid {
    columns: usize,
    data: Vec<usize>,
}

impl Grid {
    /// Creador del Grid.
    ///
    /// # Argumentos
    ///
    /// * `rows`, `columns` - Usize que indican la cantidad de filas y columnas con la que se creará la grilla.
    pub fn new(rows: usize, columns: usize) -> Grid {
        Grid {
            columns,
            data: vec![0; columns * rows],
        }
    }
    /// Inserta un número en el Grid en una determinada posición.
    ///
    /// # Argumentos
    ///
    /// * `row`, `column` - Usize que indican la posición (row, column) en la que se insertará el elemento
    /// * `number` - Usize que representa el elemento a insertar.
    pub fn insert(&mut self, row: usize, column: usize, number: usize) {
        self.data[(row * (self.columns)) + column] = number;
    }

    /// Obtiene y devuelve el elemento de una determinada posición.
    ///
    /// # Argumentos
    ///
    /// * `row`, `column` - Usize que indican la posición (row, column) de la que se obtendrá el dato.
    pub fn get(&self, row: usize, column: usize) -> usize {
        self.data[(row * (self.columns)) + column]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_creation() {
        let (rows, columns) = (5, 5);
        let grid = Grid::new(rows, columns);
        for i in 0..rows {
            for j in 0..columns {
                assert!(grid.get(i, j) == 0);
            }
        }
    }
    #[test]
    fn grid_insertion() {
        let (rows, columns) = (5, 5);
        let mut grid = Grid::new(rows, columns);
        for i in 0..rows {
            for j in 0..columns {
                grid.insert(i, j, i + j);
            }
        }
        for i in 0..rows {
            for j in 0..columns {
                assert!(grid.get(i, j) == i + j);
            }
        }
    }

    #[test]
    fn grid_replace() {
        let (rows, columns) = (5, 5);
        let mut grid = Grid::new(rows, columns);
        for i in 0..rows {
            for j in 0..columns {
                grid.insert(i, j, i + j);
            }
        }
        for i in 0..rows {
            for j in 0..columns {
                grid.insert(i, j, i);
            }
        }
        for i in 0..rows {
            for j in 0..columns {
                assert!(grid.get(i, j) == i);
            }
        }
    }
}
