type Matrix<T> = super::Matrix<T>;

impl<T: std::clone::Clone> super::Matrix<T> {
    pub fn t(&self) -> Self {
        // transpose Matrix
        // 1次元のベクトルは暗黙的に行ベクトルとして考えて転置
        if self.dim.len() != 2 && self.dim.len() != 1 {
            panic!("Dimension is not suitable for this function");
        }
        if self.dim.len() == 1 {
            let dim = vec![1, self.dim[0]];
            return Matrix { data: self.data.clone(), dim: dim.clone(), size: Self::size_init(&dim) };
        }
        let dim = vec![self.dim[1], self.dim[0]];
        let size = Matrix::<T>::size_init(&dim);
        let mut data = Vec::<T>::with_capacity(size);

        for i in 0..self.size {
            data.push(self.data[(i % dim[0]) * dim[1] + i / dim[0]].clone());
        }
        return Matrix { data, size, dim };
    }

    pub fn reshape(&self, dim: &Vec<usize>) -> Self {
        if self.size != Matrix::<T>::size_init(dim) {
            panic!("Matrix form is not correct");
        }
        return Matrix { data: self.data.clone(), dim: dim.clone(), size: self.size };
    }

    pub fn dot(a: &Matrix<T>, b: &Matrix<T>) -> Matrix<T>
        where T: std::ops::Mul<Output = T> + std::clone::Clone + std::default::Default + std::ops::Add<Output = T>
    {
        // validation
        if a.dim.len() != 2 || b.dim.len() != 2 {
            panic!("Dimension is need to be 2")
        }
        if a.dim[0] != b.dim[1] {
            panic!("left dim[0] must be equal to right dim[1]")
        }
        let mut data: Vec<T> = vec![Default::default(); a.dim[1] * b.dim[0]];
        let dim = vec![b.dim[0], a.dim[1]];

        let left = a.duplicate(b.dim[0]);
        let right = b.repeat(a.dim[1]);

        let res = {
            let mut res = Vec::<T>::new();
            for i in 0..left.len() {
                res.push(left[i].clone() * right[i].clone());
            }
            res
        };
        for i in 0..data.len() {
            data[i] = {
                let offset = i % b.dim[1] + i / a.dim[0] * b.dim[0] * b.dim[1];
                let mut r:T = Default::default();
                for j in 0..a.dim[0] {
                    r = r + res[offset.clone() + j * b.dim[0]].clone();
                }
                r
            };
        }
        return Matrix { data, size: Matrix::<T>::size_init(&dim), dim };
    }
}