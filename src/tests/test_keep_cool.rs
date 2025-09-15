use crate::keep_cool::{frac_to_sparse, tupvec_to_sparse};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frac_to_sparse() {
        let dense = vec![
            vec![1.0, 2.0, f32::NAN],
            vec![0.0, 2.0, 3.0],
            vec![f32::NAN, f32::NAN, 4.0],
        ];
        let sparse = frac_to_sparse(dense);
        assert_eq!(sparse.shape(), (3, 3));
        assert_eq!(sparse.nnz(), 6);
        assert_eq!(sparse.get(0, 0), Some(&1.0));
        assert_eq!(sparse.get(0, 1), Some(&2.0));
        assert_eq!(sparse.get(1, 0), Some(&0.0));
        assert_eq!(sparse.get(1, 1), Some(&2.0));
        assert_eq!(sparse.get(1, 2), Some(&3.0));
        assert_eq!(sparse.get(2, 2), Some(&4.0));
    }

    #[test]
    fn test_tupvec_to_sparse() {
        let dense = vec![
            vec![(1.0, 1.0, 1.0), (2.0, 2.0, 2.0), (f32::NAN, f32::NAN, f32::NAN)],
            vec![(0.0, 0.0, 0.0), (2.0, 2.0, 2.0), (3.0, 3.0 ,3.0)],
            vec![(f32::NAN, f32::NAN, f32::NAN), (f32::NAN, f32::NAN, f32::NAN), (4.0, 4.0, 4.0)],
        ];
        let (mat1, mat2, mat3) = tupvec_to_sparse(dense);
        for sparse in &[mat1, mat2, mat3] {
            assert_eq!(sparse.shape(), (3,3));
            assert_eq!(sparse.nnz(), 6);
            assert_eq!(sparse.get(0, 0), Some(&1.0));
            assert_eq!(sparse.get(0, 1), Some(&2.0));
            assert_eq!(sparse.get(1, 0), Some(&0.0));
            assert_eq!(sparse.get(1, 1), Some(&2.0));
            assert_eq!(sparse.get(1, 2), Some(&3.0));
            assert_eq!(sparse.get(2, 2), Some(&4.0));
        }
    }
}