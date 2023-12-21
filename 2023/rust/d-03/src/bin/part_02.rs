use std::fs;

type Row = usize;
type Col = usize;

const DEBUG: bool = false;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Cell {
    row: Row,
    col: Col,
    c: char,
}

pub fn main() {
    let board = fs::read_to_string("resources/input")
        .expect("failed opening file")
        .lines()
        .enumerate()
        .fold(Board::new(), |board, row_num_and_line| {
            let (row_num, line) = row_num_and_line;
            if DEBUG {
                println!("row_num: {}, line: {}", row_num, line);
            }

            line.chars()
                .enumerate()
                .fold(board, |mut board, col_and_char| {
                    let (col, char) = col_and_char;
                    if DEBUG {
                        println!("row: {} col: {}, char: {}", row_num, col, char);
                    }
                    board.add(Cell::new(row_num, col, char));
                    board
                })
        });

    let total = board
        .gear_ratios()
        .iter()
        .fold(0, |acc, gear| acc + gear.ratio());

    assert_eq!(total, 76314915);
    println!("{} {}", file!().to_string(), total);
}

impl Cell {
    fn new(row: usize, col: usize, c: char) -> Self {
        Cell {
            row: row,
            col: col,
            c,
        }
    }

    fn is_symbol(&self) -> bool {
        if self.row == 1 && self.col == 3 {
            println!("row: {}, col: {}, c: {}", self.row, self.col, self.c);
        }
        let digit = self.c.is_digit(10);
        let symbol = self.c != '.' && !digit;
        symbol
    }

    fn is_right(&self, other: &Cell) -> bool {
        self.row == other.row && other.col != std::usize::MAX && self.col == other.col + 1
    }

    fn is_above(&self, other: &Cell) -> bool {
        self.row != other.row
            && self.col == other.col
            && other.row != std::usize::MAX
            && self.row == other.row + 1
    }

    fn is_diagonal(&self, other: &Cell) -> bool {
        std::cmp::max(self.row, other.row) - std::cmp::min(self.row, other.row) == 1
            && std::cmp::max(self.col, other.col) - std::cmp::min(self.col, other.col) == 1
    }

    fn next_to(&self, other: &Cell) -> bool {
        if self == other {
            return false;
        }
        self.is_above(other)
            || other.is_above(self)
            || self.is_right(other)
            || other.is_right(self)
            || self.is_diagonal(other)
    }
}

struct Board {
    cells: Vec<Cell>,
}
struct Gear {
    part: Part,
    part2: Part,
}

impl Gear {
    fn new(part: Part, part2: Part) -> Self {
        Gear { part, part2 }
    }

    fn ratio(&self) -> usize {
        self.part.power() * self.part2.power()
    }
}
impl Board {
    fn new() -> Self {
        Board { cells: Vec::new() }
    }

    fn neighbors(&self, part: &Part) -> Vec<Cell> {
        self.cells.iter().fold(Vec::new(), |mut acc, cell| {
            if !part.contains(cell) && part.cells.iter().any(|other_cell| cell.next_to(other_cell))
            {
                acc.push(cell.clone())
            }
            acc
        })
    }

    fn add(&mut self, cell: Cell) {
        self.cells.push(cell);
    }

    fn get_parts(&self) -> Vec<Part> {
        let digit_cells = self.cells.iter().filter(|cell| cell.c.is_digit(10));
        let mut parts: Vec<Part> = digit_cells.fold(Vec::new(), |mut parts, cell| {
            let cloned = cell.clone();
            match parts.last_mut() {
                Some(last) => {
                    if !last.add_cell(cloned) {
                        parts.push(Part::new(cloned));
                    }
                }
                None => {
                    parts.push(Part::new(cell.clone()));
                }
            }
            parts
        });

        parts.retain(|part| self.neighbors(part).iter().any(|cell| cell.is_symbol()));
        parts
    }

    fn gear_ratios(&self) -> Vec<Gear> {
        let parts = self.get_parts();
        self.cells
            .iter()
            .filter(|cell| cell.c == '*')
            .fold(Vec::new(), |mut acc, cell| {
                let neighbor_parts: Vec<Part> =
                    parts.iter().fold(Vec::new(), |mut neighbor_parts, part| {
                        if part.next_to(cell) && neighbor_parts.len() < 3 {
                            neighbor_parts.push(part.clone());
                        }
                        neighbor_parts
                    });

                match neighbor_parts.get(0).zip(neighbor_parts.get(1)) {
                    Some((part, part2)) => {
                        acc.push(Gear::new(part.clone(), part2.clone()));
                    }
                    _ => {}
                }

                acc
            })
    }
}

#[derive(Debug, Clone)]
struct Part {
    cells: Vec<Cell>,
}

impl Part {
    fn new(cell: Cell) -> Self {
        let mut cells = Vec::new();
        cells.push(cell);
        Part { cells: cells }
    }

    fn add_cell(&mut self, cell: Cell) -> bool {
        match self.cells.last() {
            Some(last) if last.row == cell.row && cell.is_right(last) => {
                assert!(last.row == cell.row);
                self.cells.push(cell);
                true
            }
            _ => false,
        }
    }

    fn contains(&self, cell: &Cell) -> bool {
        self.cells.contains(cell)
    }

    fn next_to(&self, other: &Cell) -> bool {
        self.cells.iter().any(|cell| cell.next_to(other))
    }

    fn power(&self) -> usize {
        let mut power = 0;
        let mut total = 0;
        for cell in self.cells.iter().rev() {
            total += 10usize.pow(power) * cell.c.to_digit(10).unwrap() as usize;
            power += 1;
        }
        total
    }
}

#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[cfg(test)]
mod tests {

    #[cfg(test)]
    use quickcheck::{Arbitrary, Gen};

    #[cfg(test)]
    impl Arbitrary for Cell {
        fn arbitrary(g: &mut Gen) -> Self {
            let row = usize::arbitrary(g);
            let col = usize::arbitrary(g);
            let c = char::arbitrary(g);
            Cell { row, col, c }
        }
    }
    #[cfg(test)]
    use quickcheck::QuickCheck;
    const SAMPLE_INPUT: &str = r#"
    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598.."#;
    use super::*;

    #[test]
    fn test_board() {
        let board = SAMPLE_INPUT
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .enumerate()
            .fold(Board::new(), |board, row_and_line| {
                let (row, line) = row_and_line;
                println!("row: {}, line: {}", row, line);
                line.chars()
                    .enumerate()
                    .fold(board, |mut board, col_and_char| {
                        let (col, char) = col_and_char;
                        println!("row: {} col: {}, char: {}", row, col, char);
                        let cell = Cell::new(row, col, char);
                        board.add(cell);
                        board
                    })
            });
        let parts = board.get_parts();
        assert_eq!(parts.len(), 8);
        let total: usize = parts.iter().fold(0, |mut acc, part| {
            let mut power = 0;
            for cell in part.cells.iter().rev() {
                acc += 10usize.pow(power) * cell.c.to_digit(10).unwrap() as usize;
                power += 1;
            }
            acc
        });
        assert_eq!(total, 4361);
    }

    #[test]
    fn test_vicinity() {
        let cell = Cell::new(0, 0, 'a');
        let below = Cell::new(1, 0, 'b');
        assert_eq!(below.is_above(&cell), true);
    }

    #[quickcheck]
    fn prop_next_to() {
        fn prop(cell: Cell, other: Cell) -> bool {
            cell.next_to(&other) == other.next_to(&cell)
        }
        QuickCheck::new().quickcheck(prop as fn(Cell, Cell) -> bool);
    }
}
