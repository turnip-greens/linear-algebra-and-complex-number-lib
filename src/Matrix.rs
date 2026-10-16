#![feature(generic_const_exprs)]

use num_traits::{Num, Signed};
use std::ops::{Index, IndexMut};
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::iter::Sum;
use std::iter::Product;

pub struct Matrix<T, const ROWS: usize, const COLS: usize> {
    pub data: [[T; COLS]; ROWS],
}

pub type RowVector<T, const N: usize> = Matrix<T, 1, N>;
pub type ColVector<T, const N: usize> = Matrix<T, N, 1>;

pub trait Det {
    type Output;
    fn det(self) -> Self::Output;
}

pub trait Inv {
    type Output;
    fn inv(&self) -> Self::Output;
}

pub trait SubMat<const NEW_SIZE: usize> {
    type Output;
    fn get_submatrix(&self, x : usize, y: usize) -> Self::Output;
}

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

impl<T, const ROWS: usize, const COLS: usize> Clone for Matrix<T, ROWS, COLS> 
where 
    T: Default + Copy + Num + Clone
{
    fn clone(&self) -> Matrix::<T, ROWS, COLS> {
        Matrix::from_data(self.data)
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

impl<T, const ROWS: usize, const COLS: usize> Sub for Matrix<T, ROWS, COLS>
where
    T: Default + Copy + Num
{
    type Output = Matrix<T, ROWS, COLS>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = Self::new(T::default());

        for r in 0..ROWS {
            for c in 0..COLS {
                result[(r, c)] = self[(r, c)] - rhs[(r, c)];
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

impl<T: Copy + Default + Num, const SIZE: usize> SubMat<{SIZE - 1}> for Matrix<T, SIZE, SIZE> 
where
    [(); SIZE - 1]:,
{
    type Output = Matrix<T, {SIZE - 1}, {SIZE - 1}>;

    fn get_submatrix(&self, x : usize, y: usize) -> Self::Output {
        let mut sub = Self::Output::new(T::default());

        let mut current_col = 0;
        let mut current_row = 0;
        for i in 0..SIZE {
            if i == x {
                continue;
            } else {
                for j in 0..SIZE {
                    if j == y {
                        continue;
                    } else {
                        sub[(current_row, current_col)] = self[(i, j)];
                        current_col += 1;
                    }
                    
                }
                current_row += 1;
                current_col = 0;
            }
        }

        sub
    }
}

impl<T> Det for Matrix<T, 1, 1>
where
    T: Default + Copy + Num + Sum + Product + Signed,
{
    type Output = T;
    fn det(self) -> Self::Output {
        self[(0, 0)]
    }
}

impl<T> Det for Matrix<T, 2, 2>
where
    T: Default + Copy + Num + Sum + Product + Signed,
{
    type Output = T;
    fn det(self) -> Self::Output {
        self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)]
    }
}

impl<T> Det for Matrix<T, 3, 3>
where
    T: Default + Copy + Num + Sum + Product + Signed,
{
    type Output = T;
    fn det(self) -> Self::Output {
        self[(0, 0)] * (self[(1, 1)] * self[(2, 2)] - self[(1, 2)] * self[(2, 1)]) 
      - self[(0, 1)] * (self[(1, 0)] * self[(2, 2)] - self[(1, 2)] * self[(2, 0)]) 
      + self[(0, 2)] * (self[(1, 0)] * self[(2, 1)] - self[(1, 1)] * self[(2, 0)])
    }
}

impl<T> Det for Matrix<T, 4, 4>
where
    T: Default + Copy + Num + Sum + Product + Signed,
    Matrix<T, 4, 4>: SubMat<3>,
    <Matrix<T, 4, 4> as SubMat<3>>::Output: Det<Output = T>,
    [(); 3]:,
{
    type Output = T;
    fn det(self) -> Self::Output {
        self[(0, 0)] * self.get_submatrix(0, 0).det() - 
        self[(0, 1)] * self.get_submatrix(0, 1).det() +
        self[(0, 2)] * self.get_submatrix(0, 2).det() -
        self[(0, 3)] * self.get_submatrix(0, 3).det()
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

    let mat_e = mat_c.clone() * mat_d;
    println!("Result: {} {} \n       {} {}", mat_e[(0, 0)], mat_e[(0, 1)], mat_e[(1, 0)], mat_e[(1, 1)]);
    
    
    let det_e = mat_e.det();
    println!("Result: {}", det_e);

    let det_c = mat_c.det();
    println!("Result: {}", det_c);

    let mat_h = Matrix::from_data([
        [2, 0, 1],
        [3, 0, 0],
        [5, 1, 1]
    ]);

    let det_h = mat_h.det();
    println!("Result: {}", det_h);

    let mat_j = Matrix::from_data([
        [1, 2, 0, 0],
        [3, 4, 0, 0],
        [0, 0, 2, 1],
        [0, 0, 5, 3],
    ]);

    let det_j = mat_j.det();
    println!("Result: {}", det_j);
}