mod generations;
mod elementary_functions;
mod operations;

use std::cmp::max;
use std::cmp;

type Dimension = Vec<usize>;

#[derive(Debug)]
pub struct Matrix<T: Default = f64>
    where
        T: std::ops::Add + std::ops::Div + std::ops::Mul + std::ops::Neg + std::cmp::PartialEq + std::ops::Sub
{
    dim: Dimension,
    data: Vec<T>,
    size: usize
}

impl Matrix {
    pub fn size(self) -> usize {
        return self.size;
    }

    fn size_init(d: &Dimension) -> usize{
        let mut res = 1;
        for d_el in d {
            res *= *d_el
        }
        return res;
    }

    fn validate_dim_match(a: &Matrix, b: &Matrix) -> Result<Option<Dimension>, String>{
        if a.dim.len() != 2 ||  b.dim.len() != 2 {
            return Err("Dimension is not Correct".to_string())
        }

        let mut res = Ok(Some(a.dim.clone()));

        for i in 0..a.dim.len() {
            if a.dim[i] != b.dim[i] {
                if a.dim[i] != 1 && b.dim[i] != 1{
                    return Err(format!("operands could not be broadcast together with shapes {:?}, {:?}", a.dim, b.dim).to_string());
                }
                else if a.dim[i] != 1 || b.dim[i] != 1{
                    res = Ok(Some(vec![max(a.dim[0], b.dim[0]), max(a.dim[1], b.dim[1])]))
                }
            }
        }
        return res;
    }

    fn get_index_on_arithmetic_ops (i: usize, self_dim: &Dimension, other_dim: &Dimension) ->((usize, usize), (usize, usize)) {
        let index_self = (cmp::min(i % self_dim[0], self_dim[0] - 1) as usize, cmp::min(i / self_dim[1], self_dim[1] - 1) as usize);
        let index_other = (cmp::min(i % other_dim[0], other_dim[0] - 1) as usize, cmp::min(i / other_dim[1], other_dim[1] - 1) as usize);
        return (index_self, index_other);
    }
}

