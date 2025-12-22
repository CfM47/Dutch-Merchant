use crate::generator::star::matrix::{extend_matrix, gapped_array, prepend_column_and_row, prepend_row};

#[test]
fn simple_matrix_extension() {
    let m1 = [[1, 2], [3, 4]].map(|r| r.to_vec()).to_vec();

    let m2 = m1.clone();

    let m = extend_matrix(m1, m2, &0);

    assert_eq!(m, [[1, 2, 0, 0], [3, 4, 0, 0], [0, 0, 1, 2], [0, 0, 3, 4]])
}

#[test]
fn different_matrix_dimensions() {
    let m1 = [[1, 2], [3, 4]].map(|r| r.to_vec()).to_vec();

    let m2 = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
        .map(|r| r.to_vec())
        .to_vec();

    let m = extend_matrix(m1, m2, &0);

    assert_eq!(
        m,
        [
            [1, 2, 0, 0, 0],
            [3, 4, 0, 0, 0],
            [0, 0, 1, 2, 3],
            [0, 0, 4, 5, 6],
            [0, 0, 7, 8, 9]
        ]
    )
}

#[test]
fn extend_column_vectors() {
    let m1 = [[51.727909291236685], [62.0], [62.0]]
        .map(|r| r.to_vec())
        .to_vec();
    let m2 = [[25.421091776051778], [62.0], [62.0]]
        .map(|r| r.to_vec())
        .to_vec();

    let m = extend_matrix(m1, m2, &62.0);

    assert_eq!(
        m,
        [
            [51.727909291236685, 62.0],
            [62.0, 62.0],
            [62.0, 62.0],
            [62.0, 25.421091776051778],
            [62.0, 62.0],
            [62.0, 62.0]
        ]
    )
}

#[test]
fn prepend_column_and_row_simple() {
    let m1 = [[1, 2], [3, 4]].map(|r| r.to_vec()).to_vec();

    let m = prepend_column_and_row(m1, &0);

    assert_eq!(m, [[0, 0, 0], [0, 1, 2], [0, 3, 4]])
}

#[test]
fn prepend_column_and_row_empty() {
    let m = prepend_column_and_row(vec![], &0);
    let expected: Vec<Vec<i32>> = vec![];

    assert_eq!(m, expected);
}

#[test]
fn prepend_column_and_row_empty_row() {
    let m = prepend_column_and_row(vec![vec![]], &0);

    assert_eq!(m, [[0], [0]]);
}

#[test]
fn prepend_row_simple() {
    let m1 = [[1, 2], [3, 4]].map(|r| r.to_vec()).to_vec();

    let m = prepend_row(m1, &0);

    assert_eq!(m, [[0, 0], [1, 2], [3, 4]])
}

