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

#[test]
fn test_linspace() {
    let res = Matrix::linspace(0.0, 2.0, 200);
    assert_eq!(res.size(), 201);
}

#[test]
fn test_linspace_reverse() {
    let res = Matrix::linspace(2.0, 0.0, 200);
    println!("{:?}", res)
}

#[test]
fn test_sin() {
    let res = Matrix::linspace(0.0, 2.0, 200).sin();
    assert_eq!(res.size(), 201);
}

#[test]
#[should_panic(expected = "Matrix form is not correct")]
fn panic_reshape() {
    let res = Matrix::zeros(vec![5, 5]);
    let res = res.reshape(&vec![4, 6]);
}

#[test]
fn test_t(){
    let res = Matrix::linspace(0.0, 2.0, 200);
    let t = res.t();
    let a = res.reshape(&vec![201, 1]) + t;
    assert_eq!(a.size(), 201 * 201);
}