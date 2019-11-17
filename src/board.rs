use crate::utils::*;
use std::fmt;

pub struct Board {
    state: [[char; 15]; 15]
}

/*
#: TWS
^: DWS
+: TLS
-: DLS
*: center
*/

impl Board {
    pub fn default() -> Board {
        Board { state: [
            ['#', '.', '.', '-', '.', '.', '.', '#', '.', '.', '.', '-', '.', '.', '#'],
            ['.', '^', '.', '.', '.', '+', '.', '.', '.', '+', '.', '.', '.', '^', '.'],
            ['.', '.', '^', '.', '.', '.', '-', '.', '-', '.', '.', '.', '^', '.', '.'],
            ['-', '.', '.', '^', '.', '.', '.', '-', '.', '.', '.', '^', '.', '.', '-'],
            ['.', '.', '.', '.', '^', '.', '.', '.', '.', '.', '^', '.', '.', '.', '.'],
            ['.', '+', '.', '.', '.', '+', '.', '.', '.', '+', '.', '.', '.', '+', '.'],
            ['.', '.', '-', '.', '.', '.', '-', '.', '-', '.', '.', '.', '-', '.', '.'],
            ['#', '.', '.', '-', '.', '.', '.', '*', '.', '.', '.', '-', '.', '.', '#'],
            ['.', '.', '-', '.', '.', '.', '-', '.', '-', '.', '.', '.', '-', '.', '.'],
            ['.', '+', '.', '.', '.', '+', '.', '.', '.', '+', '.', '.', '.', '+', '.'],
            ['.', '.', '.', '.', '^', '.', '.', '.', '.', '.', '^', '.', '.', '.', '.'],
            ['-', '.', '.', '^', '.', '.', '.', '-', '.', '.', '.', '^', '.', '.', '-'],
            ['.', '.', '^', '.', '.', '.', '-', '.', '-', '.', '.', '.', '^', '.', '.'],
            ['.', '^', '.', '.', '.', '+', '.', '.', '.', '+', '.', '.', '.', '^', '.'],
            ['#', '.', '.', '-', '.', '.', '.', '#', '.', '.', '.', '-', '.', '.', '#'],
        ] }
    }

    pub fn at_position(&self, p: Position) -> char {
        self.state[p.row][p.col]
    }

    fn set(&self, p: Position, c: char) {

    }

    pub fn valid_at(&self, p: Position) -> [bool; 26] {
        if !"#^+_*.".contains(self.at_position(p)) {
            return [false; 26];
        }

        let mut cross = [false; 26];

        for (i, l) in alph.chars().enumerate() {
            let old = self.at_position(p);
            self.set(p, l);
            cross[i] = self.valid();
            self.set(p, old);
        }

        cross
    }

    pub fn valid(&self) -> bool {
        false
    }
}

impl fmt::Display for Board {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sep = "-".repeat(48);

        write!(f, "{}\n", sep)
        write!(f, "|   |")
        for row in alph.chars().take(15) {
            write!(f, "{}", format!(" {} |", row))
        }
        write!(f, "\n{}\n", sep)

        for row in self.state.iter () {
            write!(f, "|");
            for sq in row.iter() {
                match sq {
                    '#' => write!(f, "TWS"),
                    '^' => write!(f, "DWS"),
                    '+' => write!(f, "TLS"),
                    '-' => write!(f, "DLS"),
                    _ => write!(f, " {} ", sq)
                }
                write!(f, "|")
            }
            write!(f, "\n{}\n", sep)
        }
	}
}
