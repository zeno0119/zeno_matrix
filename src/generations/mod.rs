type Matrix<T> = super::Matrix<T>;

impl<T> Matrix<T> {
    pub fn mat(f: T) -> Matrix<T> {
        Matrix{data: vec![f], dim: vec![1], size: 1}
    }
}