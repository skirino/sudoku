use std::env;
use std::fmt;
use std::mem;
use std::ptr;

type Num      = u8;
type Cell     = Option<Num>;
type CellLine = [Cell; 9];
type Board    = [CellLine; 9];

struct World {
    board: Board
}

impl World {
    fn construct(arg: &String) -> World {
        let lines = arg.split("\\n").collect::<Vec<_>>();
        assert!(lines.len() == 9);
        assert!(lines.iter().all(|l| l.len() == 9));
        unsafe {
            let mut board: Board = mem::uninitialized();
            for (i, line) in board.iter_mut().enumerate() {
                for (j, cell) in line.iter_mut().enumerate() {
                    ptr::write(cell, World::construct_cell(lines[i].chars().nth(j).unwrap()))
                }
            }
            World { board: board }
        }
    }

    fn construct_cell(ch: char) -> Cell {
        if ch == ' ' {
            None
        } else {
            assert!('1' <= ch && ch <= '9');
            Some(ch.to_digit(10).unwrap() as Num)
        }
    }

    fn solve(self) -> World {
        self.solve_impl(0, 0).unwrap()
    }

    fn solve_impl(self, i: usize, j: usize) -> Option<World> {
        if i == 9 && j == 0 {
            return Some(self)
        }
        assert!(i < 9);
        assert!(j < 9);
        let (i_next, j_next) = if j < 8 { (i, j+1) } else { (i+1, 0) };
        let new_worlds = match self.board[i][j] {
            Some(_) => vec![self],
            None    => (1..10).map(|n| self.put_cell(i, j, n)).filter(|w| !w.has_conflict()).collect::<Vec<_>>(),
        };
        new_worlds.into_iter().filter_map(|w| w.solve_impl(i_next, j_next)).next()
    }

    fn put_cell(&self, i: usize, j: usize, n: Num) -> World {
        let mut w = World { board: self.board.clone() };
        w.board[i][j] = Some(n);
        w
    }

    fn has_conflict(&self) -> bool {
        for i in 0..9 {
            for j in 0..9 {
                if self.cell_has_conflict(i, j) {
                    return true
                }
            }
        }
        false
    }

    fn cell_has_conflict(&self, i: usize, j: usize) -> bool {
        match self.board[i][j] {
            Some(n) => self.conflict_h(i, j, n) || self.conflict_v(i, j, n) || self.conflict_b(i, j, n),
            None    => false,
        }
    }

    fn conflict_h(&self, i: usize, j0: usize, n: Num) -> bool {
        (0..9).any(|j| j != j0 && self.board[i][j] == Some(n))
    }

    fn conflict_v(&self, i0: usize, j: usize, n: Num) -> bool {
        (0..9).any(|i| i != i0 && self.board[i][j] == Some(n))
    }

    fn conflict_b(&self, i0: usize, j0: usize, n: Num) -> bool {
        let ibase = (i0 / 3) * 3;
        let jbase = (j0 / 3) * 3;
        for i in ibase..ibase+3 {
            for j in jbase..jbase+3 {
                if (i != i0 || j != j0) && self.board[i][j] == Some(n) {
                    return true
                }
            }
        }
        false
    }
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.board.iter().map(|l|
            l.iter().map(|c|
                match *c {
                    Some(ch) => format!("{}", ch),
                    None     => " ".to_string(),
                }
            ).collect::<Vec<_>>().join("")
        ).collect::<Vec<_>>().join("\n");
        write!(f, "{}", s)
    }
}

fn main() {
    let arg = env::args().last().unwrap();
    let problem = World::construct(&arg);
    println!("Problem:\n{}", problem);
    println!("\nSolution:\n{}", problem.solve());
}
