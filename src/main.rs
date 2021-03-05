use rand::prelude::*;
use std::fmt::Formatter;

fn shuffled(n: usize) -> Vec<usize> {
    let mut result: Vec<usize> = (0..n).collect();
    result.shuffle(&mut rand::thread_rng());
    return result;
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Sudoku {
    field: [[u8; 9]; 9],
}

impl std::fmt::Display for Sudoku {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for x in 0..9 {
            if x % 3 == 0 {
                writeln!(f, "-------------------------")?;
            }
            for y in 0..9 {
                if y % 3 == 0 {
                    write!(f, "| ")?;
                }
                write!(f, "{} ", self.field[x][y])?;
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "-------------------------")?;
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct PossibleEntries {
    field: [[[bool; 9]; 9]; 9],
}

impl Default for PossibleEntries {
    fn default() -> Self {
        PossibleEntries { field: [[[true; 9]; 9]; 9] }
    }
}

impl PossibleEntries {
    fn update_line(&mut self, val: u8, x: usize, y: usize) {
        if val != 0 {
            let i = (val - 1) as usize;
            self.field[x].iter_mut().for_each(|cell| cell[i] = false);
            self.field.iter_mut().for_each(|column| column[y][i] = false);
        }
    }

    fn update_block(&mut self, val: u8, x: usize, y: usize) {
        if val != 0 {
            let i = (val - 1) as usize;
            for a in x - x % 3..(x - x % 3) + 3 {
                for b in y - y % 3..(y - y % 3) + 3 {
                    self.field[a][b][i] = false;
                }
            }
        }
    }
}

fn unpack_index(index: u8) -> (usize, usize) {
    ((index % 9) as usize, (index / 9) as usize)
}

#[test]
fn test_unpack_index() {
    assert_eq!(unpack_index(0), (0, 0));
    assert_eq!(unpack_index(1), (1, 0));
    assert_eq!(unpack_index(2), (2, 0));
    assert_eq!(unpack_index(3), (3, 0));
    assert_eq!(unpack_index(4), (4, 0));
    assert_eq!(unpack_index(5), (5, 0));
    assert_eq!(unpack_index(80), (8, 8));
}

fn get_block(x: u8, y: u8) -> u8 {
    x / 3 + y - (y % 3)
}

#[test]
fn test_get_block() {
    assert_eq!(get_block(0, 0), 0);
    assert_eq!(get_block(1, 0), 0);
    assert_eq!(get_block(2, 0), 0);
    assert_eq!(get_block(0, 1), 0);
    assert_eq!(get_block(1, 1), 0);
    assert_eq!(get_block(2, 1), 0);
    assert_eq!(get_block(0, 2), 0);
    assert_eq!(get_block(1, 2), 0);
    assert_eq!(get_block(2, 2), 0);
    assert_eq!(get_block(3, 0), 1);
    assert_eq!(get_block(6, 0), 2);
    assert_eq!(get_block(0, 3), 3);
    assert_eq!(get_block(3, 3), 4);
    assert_eq!(get_block(6, 3), 5);
    assert_eq!(get_block(0, 6), 6);
    assert_eq!(get_block(3, 6), 7);
    assert_eq!(get_block(6, 6), 8);
}

impl Sudoku {
    pub fn full() -> Self {
        let result = Sudoku::default();

        result
    }

    fn _solve(&mut self, p: &PossibleEntries, ordering: fn() -> Vec<usize>, index: u8) -> bool {
        if index == 81 /* end condition: out of bounds */ {
            true
        } else {
            let (x, y) = unpack_index(index);
            let orig: u8 = self.field[x][y];
            if index == 80 /* last field */ {
                for i in ordering() {
                    if p.field[x][y][i] {
                        self.field[x][y] = (i + 1) as u8;
                        /*
                        If there still is a remaining position for the last cell,
                        we must have found a valid Sudoku.
                        */
                        return true;
                    }
                }
                false
            } else if orig == 0 /* an empty field */ {
                for i in ordering() {
                    if p.field[x][y][i] {
                        let mut _p = p.clone();
                        self.field[x][y] = (i + 1) as u8;
                        _p.update_line(self.field[x][y], x, y);
                        _p.update_block(self.field[x][y], x, y);
                        if self._solve(&_p, ordering, index + 1) {
                            return true;
                        }
                    }
                }
                self.field[x][y] = orig;
                false
            } else /* field is non-empty: skip*/ {
                self._solve(p, ordering, index + 1)
            }
        }
    }
    fn solve(&mut self, ordering: fn() -> Vec<usize>) -> bool {
        let mut p = PossibleEntries::default();
        // update all blocks and lines (basic conditions)
        for x in 0..9 {
            for y in 0..9 {
                p.update_line(self.field[x][y], x, y);
                p.update_block(self.field[x][y], x, y);
            }
        }
        return self._solve(&p, ordering, 0);
    }
    fn solve_randomized(&mut self) -> bool {
        self.solve(|| shuffled(9))
    }
    fn solve_ordered(&mut self) -> bool {
        self.solve(|| (0..9).collect())
    }
}

#[test]
fn test_update_block() {
    let mut p = PossibleEntries::default();
    let mut s = Sudoku::default();
    s.field[1][1] = 4;
    p.update_block(s.field[1][1], 1, 1);
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..9 {
                if k == 3 {
                    assert_eq!(p.field[i][j][k], false);
                } else {
                    assert_eq!(p.field[i][j][k], true);
                }
            }
        }
    }
}


fn main() {
    //println!("Hello, world!");
    let mut s = Sudoku::default();
    s.solve_randomized();
    println!("{}", s);
}
