use zeno_matrix::Matrix;

mod generations {

    type Matrix = super::Matrix<i32>;

    #[test]
    fn mat_test() {
        let a = Matrix::mat(1);
        let b = super::Matrix::<i32>::ones(&[1]);
        assert_eq!(a, b);
    }
}