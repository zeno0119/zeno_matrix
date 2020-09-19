use zeno_matrix::Matrix;

#[test]
fn test_add_non_broadcast() {
    let a = Matrix::zeros(vec![5, 5]);
    let b = Matrix::zeros(vec![5, 5]);
    let res = a + b;
    let c = Matrix::zeros(vec![5, 5]);
    assert_eq!(c, res);
}

#[test]
fn test_add_with_broadcast() {
    let a = Matrix::zeros(vec![5, 1]);
    let b = Matrix::zeros(vec![1, 5]);
    let res = a + b;
    let c = Matrix::zeros(vec![5, 5]);
    assert_eq!(c, res);
}

#[test]
#[should_panic(expected = "operands could not be broadcast together with shapes [5, 4], [5, 5]")]
fn test_add_operands_error() {
    let a = Matrix::zeros(vec![5, 4]);
    let b = Matrix::zeros(vec![5, 5]);
    let res = a + b;
}