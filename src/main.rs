fn main() {
    let mut app = App::default();
    app.fill_puzzle();
    app.poke_holes(56);
    app.print_as_string();
    let removed_values = app.removed_values.unwrap();
    println!("{}", removed_values.len());
    println!("{:?}", removed_values);
}
use std::fmt::Debug;

use rand::{seq::SliceRandom, thread_rng};
#[derive(Clone)]
struct App {
    sudoku_matrix: Vec<Cell>,
    removed_values: Option<Vec<u8>>,
}
#[derive(Clone, Copy, Debug)]
struct Cell {
    number: Option<u8>,
    row: Row,
    column: usize,
}

impl Cell {
    fn get_position(&self) -> usize {
        let index_beginning: usize = self.row.into();
        self.column + index_beginning
    }
}
#[derive(Clone, Copy, Debug)]
enum Row {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
}
impl From<Row> for usize {
    fn from(val: Row) -> Self {
        match val {
            Row::A => 0,
            Row::B => 9,
            Row::C => 18,
            Row::D => 27,
            Row::E => 36,
            Row::F => 45,
            Row::G => 54,
            Row::H => 63,
            Row::I => 72,
        }
    }
}
impl From<usize> for Row {
    fn from(val: usize) -> Self {
        let mut result = Row::A;
        match val {
            0..=8 => result = Row::A,
            9..=17 => result = Row::B,
            18..=26 => result = Row::C,
            27..=35 => result = Row::D,
            36..=44 => result = Row::E,
            45..=53 => result = Row::F,
            54..=62 => result = Row::G,
            63..=71 => result = Row::H,
            72..=80 => result = Row::I,
            _ => {}
        }
        result
    }
}

impl Default for App {
    fn default() -> Self {
        let rows = [
            Row::A,
            Row::B,
            Row::C,
            Row::D,
            Row::E,
            Row::F,
            Row::G,
            Row::H,
            Row::I,
        ];
        let mut sudoku_matrix = Vec::with_capacity(81);
        for row in rows {
            for column in 0..=8 {
                sudoku_matrix.push(Cell {
                    number: None,
                    row,
                    column,
                })
            }
        }
        println!("Build the matrix");
        Self {
            sudoku_matrix,
            removed_values: None,
        }
    }
}

impl Debug for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (pos, cell) in self.sudoku_matrix.iter().enumerate() {
            if pos % 9 == 0 {
                writeln!(f).unwrap();
            }
            write!(f, "{} ", cell.number.unwrap_or(0)).unwrap();
        }
        Ok(())
    }
}

impl App {
    fn check_row(&self, cell: &Cell) -> bool {
        let row_beginning: usize = cell.row.into();

        for x in &self.sudoku_matrix[row_beginning..(row_beginning + 9)] {
            if x.number == cell.number {
                return false;
            }
        }

        true
    }
    pub fn print_as_string(&self) {
        for cell in &self.sudoku_matrix {
            match cell.number {
                Some(num) => print!("{}", num),
                None => print!("."),
            }
        }
        println!()
    }
    fn check_column(&self, cell: &Cell) -> bool {
        let column_indexes: Vec<usize> = [0, 9, 18, 27, 36, 45, 54, 63, 72]
            .iter()
            .map(|x| x + cell.column)
            .collect();

        for index in column_indexes {
            if self.sudoku_matrix[index].number == cell.number {
                return false;
            }
        }

        true
    }

    fn check_region(&self, cell: &Cell) -> bool {
        let region_indexes = match cell.row {
            Row::A | Row::B | Row::C => match cell.column {
                0..=2 => [0, 1, 2, 9, 10, 11, 18, 19, 20],
                3..=5 => [3, 4, 5, 12, 13, 14, 21, 22, 23],
                6..=8 => [6, 7, 8, 15, 16, 17, 24, 25, 26],
                _ => [0, 0, 0, 0, 0, 0, 0, 0, 0],
            },
            Row::D | Row::E | Row::F => match cell.column {
                0..=2 => [27, 28, 29, 36, 37, 38, 45, 46, 47],
                3..=5 => [30, 31, 32, 39, 40, 41, 48, 49, 50],
                6..=8 => [33, 34, 35, 42, 43, 44, 51, 52, 53],
                _ => [0, 0, 0, 0, 0, 0, 0, 0, 0],
            },
            Row::G | Row::H | Row::I => match cell.column {
                0..=2 => [54, 55, 56, 63, 64, 65, 72, 73, 74],
                3..=5 => [57, 58, 59, 66, 67, 68, 75, 76, 77],
                6..=8 => [60, 61, 62, 69, 70, 71, 78, 79, 80],
                _ => [0, 0, 0, 0, 0, 0, 0, 0, 0],
            },
        };
        for index in region_indexes {
            if self.sudoku_matrix[index].number == cell.number {
                return false;
            }
        }

        true
    }
    fn safe_to_place(&self, cell: &Cell) -> bool {
        self.check_column(cell) && self.check_row(cell) && self.check_region(cell)
    }
    fn scan_next_cell(&mut self) -> Option<Cell> {
        for cell in &self.sudoku_matrix {
            if cell.number.is_none() {
                return Some(Cell {
                    number: None,
                    row: cell.row,
                    column: cell.column,
                });
            }
        }
        None
    }

    fn is_solved(&self) -> bool {
        self.sudoku_matrix
            .iter()
            .all(|c| c.number.is_some() && !self.safe_to_place(c))
    }
    pub fn fill_puzzle(&mut self) -> bool {
        let mut num_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut rng = thread_rng();
        num_vec.shuffle(&mut rng);

        if let Some(mut cell) = self.scan_next_cell() {
            for num in num_vec {
                cell.number = Some(num);
                if self.safe_to_place(&cell) {
                    self.sudoku_matrix[cell.get_position()].number = cell.number;
                    if self.fill_puzzle() {
                        return true;
                    }
                    self.sudoku_matrix[cell.get_position()].number = None;
                }
            }
        } else {
            return true;
        }

        false
    }
    pub fn poke_holes(&mut self, holes: usize) {
        println!("Holes: {}", holes);
        let mut removed_num: Vec<u8> = Vec::new();
        let mut values: Vec<usize> = (0..=80).collect();
        let mut rng = thread_rng();
        values.shuffle(&mut rng);
        while removed_num.len() < holes {
            for num in &values {
                if removed_num.len() >= holes {
                    break;
                }
                if self.sudoku_matrix[*num].number.is_none() {
                    continue;
                }
                removed_num.push(self.sudoku_matrix[*num].number.unwrap());
                self.sudoku_matrix[*num].number = None;
                let mut clone = self.clone();
                if clone.solve(0, 0) > 1 {
                    self.sudoku_matrix[*num].number = removed_num.pop();
                }
            }
        }

        self.removed_values = Some(removed_num);
    }

    fn next_empty_cell<'a>(&'a self, vec: Vec<&'a Cell>) -> Option<&Cell> {
        vec.into_iter()
            .find(|&cell| self.sudoku_matrix[cell.get_position()].number.is_none())
    }
    fn solve(&mut self, index: usize, mut count: i32) -> i32 {
        if index > 80 {
            return count + 1;
        }
        if self.sudoku_matrix[index].number.is_some() {
            return self.solve(index + 1, count);
        }
        for num in 1..=9 {
            if count < 2 {
                let row: Row = index.into();
                let row_beginning: usize = row.into();
                let column = index - row_beginning;
                let cell = Cell {
                    number: Some(num),
                    row,
                    column,
                };
                if self.safe_to_place(&cell) {
                    self.sudoku_matrix[index].number = cell.number;
                    count = self.solve(index + 1, count)
                }
            }
        }
        self.sudoku_matrix[index].number = None;
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn fill_puzzle() {
    //     println!("Started test");
    //     let mut app = App::default();

    //     app.fill_puzzle();

    //     println!("{:?}", app);
    // }

    #[test]
    fn poke_holes() {
        println!("Started test");
        let mut app = App::default();
        app.fill_puzzle();
        println!("{:?}", app);
        app.poke_holes(55);
        println!("{:?}", app);
        println!("Removed {:?}", app.removed_values);

        app.print_as_string();
    }
}
