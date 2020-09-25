use zeno_matrix::Matrix;

mod operations {
    type Matrixf64 = super::Matrix<f64>;

    #[test]
    fn add_non_broadcast() {
        let a = Matrixf64::zeros(&[5, 5]);
        let b = Matrixf64::zeros(&[5, 5]);
        let res = a + b;
        let c = Matrixf64::zeros(&[5, 5]);
        assert_eq!(c, res);
    }

    #[test]
    #[should_panic(expected = "operands could not be broadcast together with shapes [5, 4], [5, 5]")]
    fn add_operands_error() {
        let a = Matrixf64::zeros(&[5, 4]);
        let b = Matrixf64::zeros(&[5, 5]);
        let _res = a + b;
    }

    #[test]
    fn add_with_broadcast() {
        let a = Matrixf64::zeros(&[5, 1]);
        let b = Matrixf64::zeros(&[1, 5]);
        let res = a + b;
        let c = Matrixf64::zeros(&[5, 5]);
        assert_eq!(c, res);
    }

    #[test]
    fn add_with_dimensional_broadcast() {
        let a = Matrixf64::mat(1.0);
        let b = Matrixf64::ones(&[5, 6]);
        let res = a + b;
        let c = Matrixf64::ones(&[5, 6]) + Matrixf64::ones(&[5, 6]);
        assert_eq!(c, res);
    }
}

mod generations {
    type Matrixf64 = super::Matrix<f64>;

    #[test]
    fn test_linspace() {
        let res = Matrixf64::linspace(0.0, 2.0, 200);
        assert_eq!(res.size(), 200);
    }

    #[test]
    fn test_linspace_reverse() {
        let res = Matrixf64::linspace(2.0, 0.0, 200);
        assert_eq!(res.size(), 200);
    }
}

mod typed_functions {
    type Matrixf64 = super::Matrix<f64>;

    #[test]
    fn test_sin() {
        let res = Matrixf64::linspace(0.0, 2.0, 200).sin();
        assert_eq!(res.size(), 200);
    }

    #[test]
    fn test_rand() {
        let res = Matrixf64::rand(&[5, 5], 0.0, 1.0);
        assert_eq!(res.size(), 25);
    }

    #[test]
    #[should_panic(expected = "end Must be larger than start")]
    fn test_panic_rand() {
        let _res = Matrixf64::rand(&[5, 5], 1.0, 0.0);
    }
}

mod matrix_ops {
    type Matrixf64 = super::Matrix<f64>;
    #[test]
    fn dot() {
        let a = Matrixf64::ones(&[3, 2]);
        let b = Matrixf64::ones(&[2, 4]);
        let res = super::Matrix::dot(&a, &b);
        println!("{:?}", res);
    }

    #[test]
    fn dot2() {
        let a = Matrixf64::linspace(0.0, 1.0, 5).reshape(&[5, 1]);
        let b = Matrixf64::linspace(0.0, 1.0, 5).reshape(&[1, 5]);
        let res = super::Matrix::dot(&a, &b);
        println!("{:?}", res);
    }

    #[test]
    #[should_panic(expected = "Matrix form is not correct")]
    fn panic_reshape() {
        let res = Matrixf64::zeros(&[5, 5]);
        let _res = res.reshape(&[4, 6]);
    }

    #[test]
    fn reshape() {
        let res = Matrixf64::zeros(&[4, 6]);
        let _res = res.reshape(&[6, 4]);
    }

    #[test]
    fn transpose1() {
        let res = Matrixf64::linspace(0.0, 2.0, 200);
        let t = res.t();
        let a = res.reshape(&[200, 1]) + t;
        assert_eq!(a.size(), 200 * 200);
    }

    #[test]
    fn transpose2() {
        let res = Matrixf64::rand(&[5, 6], 0.0, 1.0);
        let t = res.t();
        let t = t.t();
        assert_eq!(res, t);
    }

    #[test]
    fn transpose3() {
        let res = Matrixf64::linspace(0.0, 2.0, 200);
        let t = res.t();
        let t = t.t();
        let a = res.reshape(&[200, 1]);
        assert_eq!(a, t);
    }
}