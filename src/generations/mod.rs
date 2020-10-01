type Matrix<T> = super::Matrix<T>;

impl<T> Matrix<T> {
    pub fn mat(f: T) -> Matrix<T> {
        Matrix{data: vec![f], dim: vec![1], size: 1}
    }

    pub fn array(data: Vec<T>, dim: &[usize]) -> Matrix<T> {
        if data.len() != Matrix::<T>::size_init(dim) { panic!("data size is not correct dim") }

        Matrix{data, size: Matrix::<T>::size_init(dim), dim: dim.to_owned()}
    }
}