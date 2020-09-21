type Matrix = super::super::Matrix<f64>;

impl Matrix {
    pub fn sin(&self) -> Matrix{
        let mut res = Matrix::zeros(self.dim.clone());
        for i in 0..res.data.len() {
            res.data[i] = self.data[i].sin();
        }
        return res;
    }

    pub fn cos(&self) -> Matrix{
        let mut res = Matrix::zeros(self.dim.clone());
        for i in 0..res.data.len() {
            res.data[i] = self.data[i].cos();
        }
        return res;
    }

    pub fn tan(&self) -> Matrix{
        let mut res = Matrix::zeros(self.dim.clone());
        for i in 0..res.data.len() {
            res.data[i] = self.data[i].tan();
        }
        return res;
    }

    pub fn function(&self, f: fn(x: f64) -> f64) -> Matrix {
        let mut data = self.data.clone();
        for i in 0..data.len() {
            data[i] = f(data[i]);
        }
        return Matrix{data, dim: self.dim.clone(), size: self.size.clone()};
    }
}