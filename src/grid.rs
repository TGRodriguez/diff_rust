/// Estructura que representa un grilla, similar a una matriz.
pub struct Grid {
    columns: usize,
    data: Vec<usize>,
}

impl Grid {
    /// Creador del Grid.
    ///
    /// Recibe la cantidad de filas y columnas que tendrá el Grid
    /// y devuelve un Grid con todas las posiciones iniciadas en 0.
    pub fn new(rows_number: usize, columns_number: usize) -> Grid {
        Grid {
            columns: columns_number,
            data: vec![0; columns_number * rows_number],
        }
    }
    /// Inserta un número en el Grid en una determinada posición.
    ///
    /// Recibe los números de fila y columna en las que insertar el
    /// número que se recibe. Si ya había un elemento, se pisa.
    pub fn insert(&mut self, row: usize, column: usize, number: usize) {
        self.data[(row * (self.columns)) + column] = number;
    }

    /// Obtiene el elemento de una determinada posición.
    ///
    /// Recibe los números de fila y columna de los que se quiere
    /// obtener el dato, y devuelve ese número.
    pub fn get(&self, row: usize, column: usize) -> usize {
        self.data[(row * (self.columns)) + column]
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn grid_creation(){
        let (rows, columns) = (5, 5);
        let grid = Grid::new(rows, columns);
        for i in 0..rows{
            for j in 0..columns{
                assert!(grid.get(i, j) == 0);
            }
        }

    }
    #[test]
    fn grid_insertion(){
        let (rows, columns) = (5, 5);
        let mut grid = Grid::new(rows, columns);
        for i in 0..rows{
            for j in 0..columns{
                grid.insert(i, j, i+j);
            }
        }
        for i in 0..rows{
            for j in 0..columns{
                assert!(grid.get(i,j) == i+j);
            }
        }
    }

    #[test]
    fn grid_replace(){
        let (rows, columns) = (5, 5);
        let mut grid = Grid::new(rows, columns);
        for i in 0..rows{
            for j in 0..columns{
                grid.insert(i, j, i+j);
            }
        }
        for i in 0..rows{
            for j in 0..columns{
                grid.insert(i, j, i);
            }
        }
        for i in 0..rows{
            for j in 0..columns{
                assert!(grid.get(i,j) == i);
            }
        }

    }
}