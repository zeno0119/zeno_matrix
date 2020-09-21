use crate::Dimension;

type Matrix<T> = super::Matrix<T>;

impl <T: std::clone::Clone> super::Matrix<T> {
    pub fn t(&self) -> Self {
        // transpose Matrix
        // 1次元のベクトルは暗黙的に行ベクトルとして考えて転置
        if self.dim.len() != 2 && self.dim.len() != 1 {
            panic!("Dimension is not suitable for this function");
        }
        if self.dim.len() == 1 {
            let dim = vec![1, self.dim[0]];
            return Matrix{data: self.data.clone(), dim: dim.clone(), size: Self::size_init(&dim)};
        }
        let dim = vec![self.dim[1], self.dim[0]];
        let size = Matrix::<T>::size_init(&dim);
        let mut data = Vec::<T>::new();
        for i in 0..self.size {
            println!("{:}, {:?}", i % dim[1] * dim[0] + i / dim[0] * dim[1], i);
            data[i % dim[1] * dim[0] + i / dim[0] * dim[1]] = self.data[i].clone();
        }
        return Matrix{data, size, dim};
    }

    pub fn reshape(&self, dim: &Vec<usize>) -> Self {
        if self.size != Matrix::<T>::size_init(dim) {
            panic!("Matrix form is not correct");
        }
        return Matrix{data: self.data.clone(), dim: dim.clone(), size: self.size};
    }
}