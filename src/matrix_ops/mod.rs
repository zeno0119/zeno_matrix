use crate::Dimension;

type Matrix = super::Matrix;

impl Matrix {
    pub fn t(&self) -> Matrix {
        // transpose Matrix
        // 1次元のベクトルは暗黙的に行ベクトルとして考えて転置
        if self.dim.len() != 2 && self.dim.len() != 1 {
            panic!("Dimension is not suitable for this function");
        }
        if self.dim.len() == 1 {
            let dim = vec![1, self.dim[0]];
            return Matrix{data: self.data.clone(), dim: dim.clone(), size: Matrix::size_init(&dim)};
        }
        let dim = vec![self.dim[1], self.dim[0]];
        let mut res = Matrix::zeros(dim);
        for i in 0..self.size {
            println!("{:}, {:?}", i % res.dim[1] * res.dim[0] + i / res.dim[0] * res.dim[1], i);
            res.data[i % res.dim[1] * res.dim[0] + i / res.dim[0] * res.dim[1]] = self.data[i];
        }
        return res;
    }

    pub fn reshape(&self, dim: &Vec<usize>) -> Matrix {
        if self.size != Matrix::size_init(dim) {
            panic!("Matrix form is not correct");
        }
        return Matrix{data: self.data.clone(), dim: dim.clone(), size: self.size};
    }
}