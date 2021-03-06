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
    fn pin_cell(&mut self, val: u8, x: usize, y: usize) {
        if val != 0 {
            self.field[x][y] = [false; 9];
            self.field[x][y][val as usize] = true;
        }
    }

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
    fn update_field(&mut self, s: &Sudoku) {
        for x in 0..9 {
            for y in 0..9 {
                self.update_line(s.field[x][y], x, y);
                self.update_block(s.field[x][y], x, y);
            }
        }
    }
    fn get_unique_value(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for i in 0..9 {
            if self.field[x][y][i] {
                count += 1;
            }
        }
        if count == 1 {
            for i in 0..9 {
                if self.field[x][y][i] {
                    return (i + 1) as u8;
                }
            }
        }
        return 0;
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
        p.update_field(self);
        return self._solve(&p, ordering, 0);
    }
    fn solve_randomized(&mut self) -> bool {
        self.solve(|| shuffled(9))
    }
    fn solve_ordered(&mut self) -> bool {
        self.solve(|| (0..9).collect())
    }
}

struct SudokuHumanLikeSolver {
    p: PossibleEntries,
    s: Sudoku,
}

impl SudokuHumanLikeSolver {
    pub fn from_sudoku(s: Sudoku) -> Self {
        let mut p = PossibleEntries::default();
        for x in 0..9 {
            for y in 0..9 {
                p.pin_cell(s.field[x][y], x, y);
            }
        }
        SudokuHumanLikeSolver {
            p,
            s,
        }
    }
    /*
    Apply standard Sudoku rules.
    If there is only a single candidate left for a cell, take it.
     */
    pub fn sole_candidate(&mut self) -> i32 {
        self.p.update_field(&self.s);
        let mut result = 0;
        for x in 0..9 {
            for y in 0..9 {
                let v = self.p.get_unique_value(x, y);
                if v != self.s.field[x][y] {
                    result += 1;
                    self.s.field[x][y] = v;
                }
            }
        }
        result
    }
    /*
    For every line/block, check if a specific number is only
    available in one cell.
    That cell might have other candidates, but we know we can
    take it.
     */
    pub fn unique_candidate(&mut self) -> i32 {
        let mut result = 0;
        // TODO
        result
    }
    /*
    If a number in a block is restricted to only one line,
    this number cannot be anywhere else on that line.
     */
    pub fn line_block_interaction(&mut self) -> i32 {
        let mut result = 0;
        // TODO
        result
    }
    /*
    This is kind of the opposite of the line-block interaction:
    If a number is ruled out in two blocks on the same line,
    it has to be in the third block on that line.
     */
    pub fn block_block_interaction(&mut self) -> i32 {
        let mut result = 0;
        // TODO
        result
    }
    /*
    Naked Subset: If we have n (e.g. 2) cells with only the
    same n candidates in one line/block, those number cannot
    be anywhere else in this line/block.
    TODO: maybe get a sorted list (by number of candidates)
      of cells and compare neighbors?
     */
    pub fn naked_subset(&mut self) -> i32 {
        let mut result = 0;
        // TODO
        result
    }
    /*
    Hidden Subset: Similar to naked subset.
    If only n cells contain the same subset of n numbers,
    (while possibly containing others as well)
    those n numbers can only be placed into these n cells,
    ruling out the other numbers contained in those cells.
    TODO: maybe use a histogram (how many cells for each number)
      for that?
    TODO: does NOT subsume naked subset
    TODO: subsumes unique_candidate
     */
    pub fn hidden_subset(&mut self) -> i32 {
        let mut result = 0;
        // TODO
        result
    }
    /*
    X-Wing: 4 cells in a rectangle spanning at least 2 blocks
    can only contain two values, in alternating fashion.
    TODO: how do I keep this from exploding in complexity?
     */
    pub fn x_wing(&mut self) -> i32 {
        let mut result = 0;
        // TODO
        result
    }
    /*
    Swordfish: This one is incredibly complicated.
    If you can connect cells with common candidate numbers
    alternating between vertical and horizontal movements,
    and you can close the cycle without repetition,
    only every other cell can contain the same number.
    This clearly subsumes x_wing.
    TODO: not sure if feasible.
     */
    /*
    Forcing Chain: If you have some cells,
    containing only two candidates and
    connected by a line/block in a chain,
    you can make a guess and follow that chain using both variants.
    If parts of the chain get locked on the same number,
    no matter your initial decision, you can keep them on that number.
    TODO: no way this is feasible
    */
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
