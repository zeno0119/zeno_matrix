
type Matrix = super::super::Matrix<i32>;
type Dimension = [usize];

impl Matrix {
    pub fn arange (start: i32, stop: i32, step: i32) -> Matrix {
        let size = ((stop - start) / step) as usize;
        let mut res = Matrix::zeros(&[size]);
        for i in 0..res.size {
            res.data[i] = start + i as i32 * step;
        }
        res
    }

    pub fn linspace(start: i32, stop: i32, num: usize) -> Matrix {
        let mut res = Matrix::zeros(&[num + 1]);

        for i in 0..res.size {
            res.data[i] = start + (stop - start) / num as i32 * i as i32;
        }

        res
    }

    pub fn zeros(d: &Dimension) -> Matrix {
        Matrix{data: vec![0; Matrix::size_init(&d)], size: Matrix::size_init(&d), dim: d.to_owned()}
    }
    pub fn ones(d: &Dimension) -> Matrix {
        Matrix{data: vec![1; Matrix::size_init(&d)], size: Matrix::size_init(&d), dim: d.to_owned()}
    }
}