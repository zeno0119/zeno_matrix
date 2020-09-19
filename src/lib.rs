use std::ops::*;
use std::cmp::max;
use std::cmp;

type Dimension = Vec<usize>;

#[derive(Debug)]
pub struct Matrix<T: Default = f64>
    where
        T: std::ops::Add + std::ops::Div + std::ops::Mul + std::ops::Neg + std::cmp::PartialEq + std::ops::Sub
{
    dim: Dimension,
    data: Vec<T>,
    size: usize
}

impl Matrix {
    pub fn zeros(d: Dimension) -> Matrix<f64> {
        let res = Matrix{data: vec![0.0; Matrix::size_init(&d)], size: Matrix::size_init(&d), dim: d.clone()};
        return res
    }
    pub fn ones(d: Dimension) -> Matrix<f64> {
        let res = Matrix{data: vec![1.0; Matrix::size_init(&d)], size: Matrix::size_init(&d), dim: d.clone()};
        return res
    }

    pub fn size(self) -> usize {
        let mut res = 1;
        for d in self.dim {
            res *= d;
        }
        return res;
    }

    fn size_init(d: &Dimension) -> usize{
        let mut res = 1;
        for d_el in d {
            res *= *d_el
        }
        return res;
    }

    fn validate_dim_match(a: &Matrix, b: &Matrix) -> Result<Option<Dimension>, String>{
        if a.dim.len() != 2 ||  b.dim.len() != 2 {
            return Err("Dimension is not Correct".to_string())
        }

        let mut res = Ok(Some(a.dim.clone()));

        for i in 0..a.dim.len() {
            if a.dim[i] != b.dim[i] {
                if a.dim[i] != 1 && b.dim[i] != 1{
                    return Err(format!("operands could not be broadcast together with shapes {:?}, {:?}", a.dim, b.dim).to_string());
                }
                else if a.dim[i] != 1 || b.dim[i] != 1{
                    res = Ok(Some(vec![max(a.dim[0], b.dim[0]), max(a.dim[1], b.dim[1])]))
                }
            }
        }
        return res;
    }

    fn get_index_on_arithmetic_ops (i: usize, self_dim: &Dimension, other_dim: &Dimension) ->((usize, usize), (usize, usize)) {
        let index_self = (cmp::min(i % self_dim[0], self_dim[0] - 1) as usize, cmp::min(i / self_dim[1], self_dim[1] - 1) as usize);
        let index_other = (cmp::min(i % other_dim[0], other_dim[0] - 1) as usize, cmp::min(i / other_dim[1], other_dim[1] - 1) as usize);
        return (index_self, index_other);
    }
}

impl Add for Matrix {
    type Output = Self;
    fn add(self, other: Self) -> Self{
        let validate = Matrix::validate_dim_match(&self, &other);
        if validate.is_err() { panic!(validate.err().unwrap()); }

        let validate = validate.unwrap();
        let mut adder = Matrix::zeros(validate.unwrap());
        for i in 0..adder.size {
            let index = Matrix::get_index_on_arithmetic_ops(i, &self.dim, &other.dim);
            adder.data[i] = self.data[(index.0).1 * self.dim[0] + (index.0).0] + other.data[(index.1).1 * other.dim[0] + (index.1).0];
        }

        adder
    }
}

impl Sub for Matrix{
    type Output = Self;
    fn sub(self, other: Self) -> Matrix{
        let validate = Matrix::validate_dim_match(&self, &other);
        if validate.is_err() { panic!(validate.err().unwrap()); }

        let validate = validate.unwrap();
        let mut suber = Matrix::zeros(validate.unwrap());
        for i in 0..suber.size {
            let index = Matrix::get_index_on_arithmetic_ops(i, &self.dim, &other.dim);
            suber.data[i] = self.data[(index.0).1 * self.dim[0] + (index.0).0] - other.data[(index.1).1 * other.dim[0] + (index.1).0];
        }

        suber
    }
}

impl Mul for Matrix{
    //Hadamard product
    type Output = Self;
    fn mul(self, other:Self) -> Matrix{
        let validate = Matrix::validate_dim_match(&self, &other);
        if validate.is_err() { panic!(validate.err().unwrap()); }

        let validate = validate.unwrap();
        let mut muller = Matrix::zeros(validate.unwrap());
        for i in 0..muller.size {
            let index = Matrix::get_index_on_arithmetic_ops(i, &self.dim, &other.dim);
            muller.data[i] = self.data[(index.0).1 * self.dim[0] + (index.0).0] * other.data[(index.1).1 * other.dim[0] + (index.1).0];
        }

        muller
    }
}

impl PartialEq for Matrix{
    fn eq(&self, other: &Self) -> bool{
        if self.size != other.size { return false; }

        if self.dim.len() != self.dim.len() { return false; }

        for i in 0..self.data.len() {
            if self.data[i] != other.data[i] { return false; }
        }
        return true;
    }
}

