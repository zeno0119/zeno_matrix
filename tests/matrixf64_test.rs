use zeno_matrix::Matrix;

mod operations {
    type Matrixf64 = super::Matrix<f64>;

    #[test]
    fn add_non_broadcast() {
        let a = Matrixf64::zeros(vec![5, 5]);
        let b = Matrixf64::zeros(vec![5, 5]);
        let res = a + b;
        let c = Matrixf64::zeros(vec![5, 5]);
        assert_eq!(c, res);
    }

    #[test]
    #[should_panic(expected = "operands could not be broadcast together with shapes [5, 4], [5, 5]")]
    fn add_operands_error() {
        let a = Matrixf64::zeros(vec![5, 4]);
        let b = Matrixf64::zeros(vec![5, 5]);
        let res = a + b;
    }

    #[test]
    fn add_with_broadcast() {
        let a = Matrixf64::zeros(vec![5, 1]);
        let b = Matrixf64::zeros(vec![1, 5]);
        let res = a + b;
        let c = Matrixf64::zeros(vec![5, 5]);
        assert_eq!(c, res);
    }

    #[test]
    #[should_panic(expected = "Matrix form is not correct")]
    fn panic_reshape() {
        let res = Matrixf64::zeros(vec![5, 5]);
        let res = res.reshape(&vec![4, 6]);
    }

    #[test]
    fn reshape() {
        let res = Matrixf64::zeros(vec![4, 6]);
        let res = res.reshape(&vec![6, 4]);
    }

    #[test]
    fn transpose1() {
        let res = Matrixf64::linspace(0.0, 2.0, 200);
        let t = res.t();
        let a = res.reshape(&vec![201, 1]) + t;
        assert_eq!(a.size(), 201 * 201);
    }

    #[test]
    fn transpose2() {
        let res = Matrixf64::rand(vec![5, 6], 0.0, 1.0);
        let t = res.t();
        let t = t.t();
        assert_eq!(res, t);
    }

    #[test]
    fn transpose3() {
        let res = Matrixf64::linspace(0.0, 2.0, 200);
        let t = res.t();
        let t = t.t();
        let a = res.reshape(&vec![201, 1]);
        assert_eq!(a, t);
    }
}

mod generations {
    type Matrixf64 = super::Matrix<f64>;
    #[test]
    fn test_linspace() {
        let res = Matrixf64::linspace(0.0, 2.0, 200);
        assert_eq!(res.size(), 201);
    }

    #[test]
    fn test_linspace_reverse() {
        let res = Matrixf64::linspace(2.0, 0.0, 200);
        assert_eq!(res.size(), 201);
    }
}

mod typed_functions{
    type Matrixf64 = super::Matrix<f64>;

    #[test]
    fn test_sin() {
        let res = Matrixf64::linspace(0.0, 2.0, 200).sin();
        assert_eq!(res.size(), 201);
    }

    #[test]
    fn test_rand() {
        let res = Matrixf64::rand(vec![5, 5], 0.0, 1.0);
        assert_eq!(res.size(), 25);
    }

    #[test]
    #[should_panic(expected = "end Must be larger than start")]
    fn test_panic_rand() {
        let res = Matrixf64::rand(vec![5, 5], 1.0, 0.0);
    }
}