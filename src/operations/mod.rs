use std::ops::*;

type Matrix<T> = super::Matrix<T>;

impl<T> Add for Matrix<T>
where T: std::ops::Add<Output = T> + std::clone::Clone
{
    type Output = Self;
    fn add(self, other: Self) -> Matrix<T>{
        let validate = Matrix::validate_dim_match(&self, &other);
        if validate.is_err() { panic!(validate.err().unwrap()); }

        let dim = validate.unwrap();
        let size: usize = Matrix::<T>::size_init(&dim);
        let mut data: Vec<T> = Vec::new();
        for i in 0..size {
            let index = Matrix::<T>::get_index_on_arithmetic_ops(i, &self.dim, &other.dim);
            data.push(self.data[(index.0).1 * self.dim[0] + (index.0).0].clone() + other.data[(index.1).1 * other.dim[0] + (index.1).0].clone());
        }

        Matrix{data, size, dim}
    }
}

impl<T> Sub for Matrix<T>
where T: std::ops::Sub<Output = T> + std::clone::Clone
{
    type Output = Self;
    fn sub(self, other: Self) -> Matrix<T>{
        let validate = Matrix::validate_dim_match(&self, &other);
        if validate.is_err() { panic!(validate.err().unwrap()); }

        let dim = validate.unwrap();
        let size = Matrix::<T>::size_init(&dim);
        let mut data: Vec<T> = Vec::new();
        for i in 0..size {
            let index = Matrix::<T>::get_index_on_arithmetic_ops(i, &self.dim, &other.dim);
            data.push(self.data[(index.0).1 * self.dim[0] + (index.0).0].clone() - other.data[(index.1).1 * other.dim[0] + (index.1).0].clone());
        }

        Matrix{data, size, dim}
    }
}

impl<T> Mul for Matrix<T>
where T: std::ops::Mul<Output = T> + std::clone::Clone
{
    //Hadamard product
    type Output = Self;
    fn mul(self, other:Self) -> Matrix<T>{
        let validate = Matrix::validate_dim_match(&self, &other);
        if validate.is_err() { panic!(validate.err().unwrap()); }

        let dim = validate.unwrap();
        let size = Matrix::<T>::size_init(&dim);
        let mut data: Vec<T> = Vec::new();
        for i in 0..size {
            let index = Matrix::<T>::get_index_on_arithmetic_ops(i, &self.dim, &other.dim);
            data.push(self.data[(index.0).1 * self.dim[0] + (index.0).0].clone() * other.data[(index.1).1 * other.dim[0] + (index.1).0].clone());
        }

        Matrix{data, size, dim}
    }
}

impl<T> PartialEq for Matrix<T>
where T:std::cmp::PartialEq
{
    fn eq(&self, other: &Self) -> bool{
        if self.size != other.size { return false; }

        if self.dim.len() != self.dim.len() { return false; }

        for i in 0..self.data.len() {
            if self.data[i] != other.data[i] { return false; }
        }
        return true;
    }
}

