/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    assert!(mat1.len() > 0);
    assert_eq!(mat1[0].len(), mat2.len());

    let mut ret: Matrix = vec![vec![0.0; mat1[0].len()]; mat2.len()];

    for (i, mat1row) in mat1.iter().enumerate() {
        for j in 0..mat1.len() {
            for (k, mat2row) in mat2.iter().enumerate() {
                ret[i][j] += mat2row[j] * mat1row[k]
            }
        }
    }

    ret
}