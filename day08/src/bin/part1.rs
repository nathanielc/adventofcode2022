use anyhow::Result;
use std::{
    collections::HashSet,
    fs,
    iter::{Enumerate, FlatMap},
    slice::Iter,
};

struct Grid {
    data: Vec<u8>,
    rows: usize,
}
impl Grid {
    fn row_iter(&self, i: usize) -> Iter<'_, u8> {
        let start = i * self.rows;
        self.data[start..start + self.rows].iter()
    }
    fn col_iter(&self, j: usize) -> ColIter {
        ColIter {
            grid: &self,
            front_i: 0,
            back_i: self.rows,
            j,
        }
    }
    fn at(&self, i: usize, j: usize) -> &u8 {
        &self.data[i * self.rows + j]
    }
    fn count_visible(&self) -> usize {
        let mut set = HashSet::<(usize, usize)>::new();
        for i in 0..self.rows {
            // Walk rows
            for j in visible_set(self.row_iter(i)) {
                set.insert((i, j));
            }
            for j in visible_set(self.row_iter(i).rev()) {
                set.insert((i, self.rows - 1 - j));
            }
            // Walk cols
            for j in visible_set(self.col_iter(i)) {
                set.insert((j, i));
            }
            for j in visible_set(self.col_iter(i).rev()) {
                set.insert((self.rows - 1 - j, i));
            }
        }
        set.len()
    }
}

fn visible_set<'a, I>(
    iter: I,
) -> FlatMap<Enumerate<I>, Option<usize>, impl FnMut((usize, &'a u8)) -> Option<usize>>
where
    I: Iterator<Item = &'a u8>,
{
    let mut max_height = -1;
    iter.enumerate()
        .flat_map(move |(i, height)| -> Option<usize> {
            if *height as i32 > max_height {
                max_height = *height as i32;
                Some(i)
            } else {
                None
            }
        })
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
    println!("Count: {}", grid.count_visible());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_grid_at() {
        let grid = Grid {
            data: vec![
                3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0,
            ],
            rows: 5,
        };
        assert_eq!(3, *grid.at(0, 0));
        assert_eq!(3, *grid.at(2, 2));
        assert_eq!(9, *grid.at(4, 3));
        assert_eq!(6, *grid.at(2, 0));
    }
    #[test]
    fn test_grid_row_iter() {
        let grid = Grid {
            data: vec![
                3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0,
            ],
            rows: 5,
        };
        assert_eq!(
            vec![3, 0, 3, 7, 3],
            grid.row_iter(0).map(|i| *i).collect::<Vec<_>>()
        );
        assert_eq!(
            vec![6, 5, 3, 3, 2],
            grid.row_iter(2).map(|i| *i).collect::<Vec<_>>()
        );
    }
    #[test]
    fn test_grid_col_iter() {
        let grid = Grid {
            data: vec![
                3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0,
            ],
            rows: 5,
        };
        assert_eq!(
            vec![3, 2, 6, 3, 3],
            grid.col_iter(0).map(|i| *i).collect::<Vec<_>>()
        );
        assert_eq!(
            vec![3, 3, 6, 2, 3],
            grid.col_iter(0).rev().map(|i| *i).collect::<Vec<_>>()
        );
        assert_eq!(
            vec![7, 1, 3, 4, 9],
            grid.col_iter(3).map(|i| *i).collect::<Vec<_>>()
        );
        assert_eq!(
            vec![9, 4, 3, 1, 7],
            grid.col_iter(3).rev().map(|i| *i).collect::<Vec<_>>()
        );
    }
    #[test]
    fn test_visible_set() {
        let data = vec![0, 1, 2, 2, 2, 3, 2, 1, 0];
        let vs = visible_set(data.iter());
        assert_eq!(vec![0, 1, 2, 5], vs.collect::<Vec<usize>>());
    }
    #[test]
    fn test_grid_count_visible() {
        let grid = Grid {
            data: vec![
                3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0,
            ],
            rows: 5,
        };
        assert_eq!(21, grid.count_visible());
    }
}
