use std::ops::Deref;
use rand::prelude::*;

type Matrix = super::super::Matrix<f64>;
type Dimension = Vec<usize>;

impl Matrix {
    pub fn arange (start: f64, stop: f64, step: f64) -> Matrix {
        let size = ((stop - start) / step) as usize;
        let mut res = Matrix::zeros(vec![size]);
        for i in 0..res.size {
            res.data[i] = start + i as f64 * step;
        }
        return res;
    }

    pub fn linspace(start: f64, stop: f64, num: usize) -> Matrix {
        let mut res = Matrix::zeros(vec![num + 1]);

        for i in 0..res.size {
            res.data[i] = start + (stop - start) / num as f64 * i as f64;
        }

        return res
    }

    pub fn zeros(d: Dimension) -> Matrix {
        let res = Matrix{data: vec![0.0; Matrix::size_init(&d)], size: Matrix::size_init(&d), dim: d.clone()};
        return res
    }
    pub fn ones(d: Dimension) -> Matrix {
        let res = Matrix{data: vec![1.0; Matrix::size_init(&d)], size: Matrix::size_init(&d), dim: d.clone()};
        return res
    }

    pub fn rand(d: Dimension, start: f64, end: f64) -> Matrix {
        if start > end {
            panic!("end Must be larger than start");
        }
        let size = Matrix::size_init(&d);
        let mut data = vec![0.0; size];
        let mut rng = rand::thread_rng();
        for i in 0..data.len() {
            data[i] = (rng.gen::<f64>() + start) / (1.0 + start) * end;
        }
        return Matrix{size, data, dim: d};
    }
}