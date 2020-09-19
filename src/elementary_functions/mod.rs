type Matrix = super::Matrix;

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
}