use anyhow::Result;
use std::{
    collections::HashSet,
    fs,
    iter::{Enumerate, FlatMap, Rev},
    ops::Range,
    slice::Iter,
};

struct Grid {
    data: Vec<u8>,
    rows: usize,
}
impl Grid {
    fn row_begin(&self, i: usize, j: usize) -> Rev<Iter<'_, u8>> {
        let start = i * self.rows;
        self.data[start..start + j].iter().rev()
    }
    fn row_end(&self, i: usize, j: usize) -> Iter<'_, u8> {
        let start = i * self.rows + j + 1;
        self.data[start..self.rows * (i + 1)].iter()
    }
    fn col_begin(&self, i: usize, j: usize) -> Rev<ColIter> {
        ColIter {
            grid: &self,
            front_i: 0,
            back_i: i,
            j,
        }
        .rev()
    }
    fn col_end(&self, i: usize, j: usize) -> ColIter {
        ColIter {
            grid: &self,
            front_i: i + 1,
            back_i: self.rows,
            j,
        }
    }
    fn at(&self, i: usize, j: usize) -> &u8 {
        &self.data[i * self.rows + j]
    }
    fn visible_score(&self, i: usize, j: usize) -> i32 {
        let height = self.at(i, j);
        let rb = visible_score(height, self.row_begin(i, j));
        let re = visible_score(height, self.row_end(i, j));
        let cb = visible_score(height, self.col_begin(i, j));
        let ce = visible_score(height, self.col_end(i, j));
        rb * re * cb * ce
    }
    fn max_score(&self) -> i32 {
        let mut s = 0;
        // Ignore the edges since there score is always 0
        for i in 1..self.rows - 1 {
            for j in 1..self.rows - 1 {
                let score = self.visible_score(i, j);
                if score > s {
                    s = score
                }
            }
        }
        s
    }
}

fn visible_score<'a, I>(height: &u8, iter: I) -> i32
where
    I: IntoIterator<Item = &'a u8>,
{
    let mut count = 0;
    for h in iter {
        if height > h {
            count += 1;
        } else {
            // We can't see anymore trees but we can see the one blocking the rest
            count += 1;
            return count;
        }
    }
    return count;
}
struct ColIter<'a> {
    grid: &'a Grid,
    back_i: usize,
    front_i: usize,
    j: usize,
}
impl<'a> Iterator for ColIter<'a> {
    type Item = &'a u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front_i >= self.back_i {
            None
        } else {
            let next = Some(self.grid.at(self.front_i, self.j));
            self.front_i += 1;
            next
        }
    }
}
impl<'a> DoubleEndedIterator for ColIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front_i >= self.back_i {
            None
        } else {
            self.back_i -= 1;
            let next = Some(self.grid.at(self.back_i, self.j));
            next
        }
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let mut grid = Grid {
        data: Vec::new(),
        rows: 0,
    };

    for (i, c) in input.chars().enumerate() {
        match c {
            '0'..='9' => grid.data.push(c.to_digit(10).unwrap() as u8),
            '\n' => {
                if grid.rows == 0 {
                    grid.rows = i;
                }
            }
            _ => panic!("invalid input"),
        };
    }
    println!("Score: {}", grid.max_score());

    Ok(())
}
