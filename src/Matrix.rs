use num_traits::Num;
use std::ops::{Index, IndexMut};
use std::ops::Add;
use std::ops::Mul;
use std::iter::Sum;

pub struct Matrix<T, const ROWS: usize, const COLS: usize> {
    pub data: [[T; COLS]; ROWS],
}

// pub type RowVector<T, const N: usize> = Matrix<T, 1, N>;
// pub type ColVector<T, const N: usize> = Matrix<T, N, 1>;

impl<T, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> 
where 
    T: Default + Copy + Num
{
    pub fn from_data(data: [[T; COLS]; ROWS]) -> Self {
        Self { data }
    }

    pub fn new(fill : T) -> Self {
        Self { data: [[fill; COLS]; ROWS] }
    }
}

impl<T, const ROWS: usize, const COLS: usize> Index<(usize, usize)> for Matrix<T, ROWS, COLS> 
where
    T: Default + Copy + Num
{
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

impl<T, const ROWS: usize, const COLS: usize> IndexMut<(usize, usize)> for Matrix<T, ROWS, COLS>
where
    T: Default + Copy + Num
{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0][index.1]
    }
}

pub type RowVector<T, const N: usize> = Matrix<T, 1, N>;
pub type ColVector<T, const N: usize> = Matrix<T, N, 1>;

impl<T, const ROWS: usize, const COLS: usize> Add for Matrix<T, ROWS, COLS>
where
    T: Default + Copy + Num
{
    type Output = Matrix<T, ROWS, COLS>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Self::new(T::default());

        for r in 0..ROWS {
            for c in 0..COLS {
                result[(r, c)] = self[(r, c)] + rhs[(r, c)];
            }
        }

        result
    }
}

impl<T, const ROWS: usize, const COLS: usize, const COLS_RHS: usize> Mul<Matrix<T, COLS, COLS_RHS>> for Matrix<T, ROWS, COLS>
where
    T: Default + Copy + Num + Sum
{
    type Output = Matrix<T, ROWS, COLS_RHS>;

    fn mul(self, rhs: Matrix<T, COLS, COLS_RHS>) -> Self::Output {
        let mut result = Self::Output::new(T::default());

        for row in 0..ROWS {
            for col_rhs in 0..COLS_RHS {
                let total: T = (0..COLS).map(|c| self[(row, c)] * rhs[(c, col_rhs)]).sum();
                result[(row, col_rhs)] = total;
            }
        }

        result
    }
}

fn main() {
    let mat_a = Matrix::from_data([
        [1, 2, 3],
        [4, 5, 6]
    ]);

    let mat_b = Matrix::from_data([
        [7,   8],
        [9,  10],
        [11, 12]
    ]);

    let mat_c = mat_a * mat_b;

    println!("Result: {} {} \n       {} {}", mat_c[(0, 0)], mat_c[(0, 1)], mat_c[(1, 0)], mat_c[(1, 1)]);

    let mat_d = Matrix::from_data([
        [2, 2],
        [2, 2]
    ]);

    // Fails to compile if uncommented
    // let _mat_f = mat_c + mat_b;

    let mat_e = mat_c * mat_d;
    println!("Result: {} {} \n       {} {}", mat_e[(0, 0)], mat_e[(0, 1)], mat_e[(1, 0)], mat_e[(1, 1)]);
}