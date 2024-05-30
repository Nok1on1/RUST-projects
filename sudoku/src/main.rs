fn main() {
    let x: Sudoku = Sudoku {
        data: vec![
            vec![7, 8, 4, 1, 5, 9, 3, 2, 6],
            vec![5, 3, 9, 6, 7, 2, 8, 4, 1],
            vec![6, 1, 2, 4, 3, 8, 7, 5, 9],
            vec![9, 2, 8, 7, 1, 5, 4, 6, 3],
            vec![3, 5, 7, 8, 4, 6, 1, 9, 2],
            vec![4, 6, 1, 9, 2, 3, 5, 8, 7],
            vec![8, 7, 6, 3, 9, 4, 2, 1, 5],
            vec![2, 4, 3, 5, 6, 1, 9, 7, 8],
            vec![1, 9, 5, 2, 8, 7, 6, 3, 4],
        ],
    };

    print!("{}", x.checkblocks(&vec![1, 9, 5, 2, 8, 7, 6, 3, 4]));
}

struct Sudoku {
    data: Vec<Vec<u32>>,
}

impl Sudoku {
    fn is_valid(&self) -> bool {
        for i in &self.data {
            if i.len() != self.data.len() {
                return false;
            }
        }

        let sqrt_n = (self.data.len() as f64).sqrt();
        if sqrt_n.fract() != 0.0 {
            return false;
        };

        let num1: Vec<u32> = (1..self.data.len() + 1).map(|x| x as u32).collect();
        return Self::checkrow(self, &num1)
            && Self::checkcolumn(self, &num1)
            && Self::checkblocks(self, &num1);
    }
    fn checkrow(&self, num1: &Vec<u32>) -> bool {
        let mut num = num1.clone();

        for i in &self.data {
            for j in i {
                if num.contains(&j) {
                    num.retain(|x| *x != *j)
                } else {
                    return false;
                }
            }
            if !num.is_empty() {
                return false;
            }
            num = num1.clone();
        }
        return true;
    }

    fn checkcolumn(&self, num1: &Vec<u32>) -> bool {
        let mut num = num1.clone();
        let length = num1.len();
        for i in 0..length {
            for j in 0..length {
                let value = match self.data.get(j as usize) {
                    None => return false,
                    Some(x) => x,
                };

                let value = match value.get(i as usize) {
                    None => return false,
                    Some(x) => x,
                };

                if num.contains(value) {
                    num.retain(|x| *x != *value);
                } else {
                    return false;
                }
            }
            if !num.is_empty() {
                return false;
            }
            num = num1.clone();
        }
        return true;
    }

    fn nextcheck(row: i32, column: i32, len: i32) -> (i32, i32, i32) {
        if row + len == len * len && column + len == len * len {
            return (-1, -1, -1);
        } else if column + len == len * len {
            return (row + len, 0, len);
        } else {
            return (row, column + len, len);
        }
    }

    fn checkblocks(&self, num1: &Vec<u32>) -> bool {
        let mut row = 0;
        let mut column = 0;
        let length = f64::sqrt(num1.len() as f64) as i32;

        let mut num = num1.clone();

        loop {
            for i in 0..length {
                for j in 0..length {
                    let value = match self.data.get((j + column) as usize) {
                        None => return false,
                        Some(x) => x,
                    };

                    let value = match value.get((i + row) as usize) {
                        None => return false,
                        Some(x) => x,
                    };

                    if num.contains(value) {
                        num.retain(|x| *x != *value);
                    } else {
                        return false;
                    }
                }
            }
            if !num.is_empty() {
                return false;
            }
            num = num1.clone();
            match Self::nextcheck(row, column, length) {
                (-1, -1, -1) => break,
                (x, y, _) => {
                    row = x;
                    column = y;
                }
            }
        }

        return true;
    }
}
