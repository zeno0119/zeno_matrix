type Matrix = super::super::Matrix<f64>;

impl Matrix {
    pub fn sin(&self) -> Matrix{
        let mut data = self.data.clone();
        for i in &mut data {
            *i = i.sin();
        }
        Matrix{data, dim: self.dim.clone(), size: self.size}
    }

    pub fn cos(&self) -> Matrix{
        let mut data = self.data.clone();
        for i in &mut data {
            *i = i.cos();
        }
        Matrix{data, dim: self.dim.clone(), size: self.size}
    }

    pub fn tan(&self) -> Matrix{
        let mut data = self.data.clone();
        for i in &mut data {
            *i = i.tan();
        }
        Matrix{data, dim: self.dim.clone(), size: self.size}
    }

    pub fn function(&self, f: fn(x: f64) -> f64) -> Matrix {
        let mut data = self.data.clone();
        for i in &mut data {
            *i = f(*i);
        }
        Matrix{data, dim: self.dim.clone(), size: self.size}
    }
}