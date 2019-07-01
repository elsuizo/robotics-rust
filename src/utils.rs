/// Utility functions and algorithms
//-------------------------------------------------------------------------
//                        imports
//-------------------------------------------------------------------------
use num::Float;
use super::error::UtilError;
use nalgebra::{Vector3, Vector2, Vector1, Matrix3, Matrix2, Scalar, RowVector3, RowVector2};
use alga::general::RingCommutative;

// NOTE(elsuizo:2019-04-12): creo que no hace falta ya que nalgebra tiene cross
pub fn is_rotation<SF: Scalar + Float + RingCommutative + na::ComplexField>(R: &Matrix3<SF>) -> bool {
    let mut result = false;
    // the determinant must be almost one
    if R.determinant() - Float::abs(SF::one()) < SF::epsilon() {
        result = true;
    } else {
        result = false;
    }

    result
}


pub fn skew_from_vector3<SF:Scalar + Float + RingCommutative>(v: &Vector3<SF>) -> Matrix3<SF> {
    let zero = SF::zero();
       Matrix3::from_rows(&[RowVector3::new(zero,   -v[(2)],   v[(1)]),
                            RowVector3::new(v[(2)],   zero,   -v[(0)]),
                            RowVector3::new(v[(1)],   v[(0)], zero),])
}

pub fn skew_from_vector1<SF:Scalar + Float + RingCommutative>(v: &Vector1<SF>) -> Matrix2<SF> {
    let zero = SF::zero();
    Matrix2::from_rows(&[RowVector2::new(zero,   -v[(0)]),
                         RowVector2::new(v[(0)],   zero,),
                        ])
}
