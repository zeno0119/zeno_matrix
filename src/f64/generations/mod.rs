use rand::prelude::*;

type Matrix = super::super::Matrix<f64>;
type Dimension = [usize];

impl Matrix {
    pub fn arange (start: f64, stop: f64, step: f64) -> Matrix {
        let size = ((stop - start) / step) as usize;
        let mut res = Matrix::zeros(&[size]);
        for i in 0..res.size {
            res.data[i] = start + i as f64 * step;
        }
        res
    }

    pub fn linspace(start: f64, stop: f64, num: usize) -> Matrix {
        let mut res = Matrix::zeros(&[num]);

        for i in 0..res.size {
            res.data[i] = start + (stop - start) / (num - 1) as f64 * i as f64;
        }

        res
    }

    pub fn zeros(d: &Dimension) -> Matrix {
        Matrix{data: vec![0.0; Matrix::size_init(&d)], size: Matrix::size_init(&d), dim: d.to_owned()}
    }
    pub fn ones(d: &Dimension) -> Matrix {
        Matrix{data: vec![1.0; Matrix::size_init(&d)], size: Matrix::size_init(&d), dim: d.to_owned()}
    }

    pub fn rand(d: &Dimension, start: f64, end: f64) -> Matrix {
        if start > end {
            panic!("end Must be larger than start");
        }
        let size = Matrix::size_init(&d);
        let mut data = vec![0.0; size];
        let mut rng = rand::thread_rng();
        for i in &mut data {
            *i = rng.gen::<f64>() * (end - start) + start;
        }
        Matrix{size, data, dim: d.to_owned()}
    }
}