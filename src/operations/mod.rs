type Matrix = super::Matrix;
use std::ops::*;

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

