/// Utility functions and algorithms
//-------------------------------------------------------------------------
//                        imports
//-------------------------------------------------------------------------
use ndarray::prelude::*;
use ndarray_linalg::solve::Determinant;
use num::Float;
use super::error::UtilError;
use ndarray_linalg::types::Scalar;

//-------------------------------------------------------------------------
//                        code
//-------------------------------------------------------------------------
/// brief.
///
/// Compute the cross product of two Arrays of 3D
pub fn cross<F: Float>(u: &Array1<F>, v: &Array1<F>) -> Result<Array1<F>, UtilError> {
    // NOTE(elsuizo:2019-04-07): no olvidar que como estamos haciendo el codigo generico tenemos
    // que hacerlo tambien cuando inicializamos, por eso utilizamos F::zero()
    if ((u.len() == v.len()) && (u.len() == 3)) {
        let mut w = arr1(&[F::zero(), F::zero(), F::zero()]);
        w[0] = u[1] * v[2] - u[2] * v[1];
        w[1] = u[2] * v[1] - u[0] * v[2];
        w[2] = u[0] * v[1] - u[1] * v[0];

        return Ok(w);
    }
    else {
        return Err(UtilError::InvalidDimentionOrNotEq{
            len_u: u.len(),
            len_v: v.len(),
        })
    }
}

// NOTE(elsuizo:2019-04-11): esta funcion con Float solo na andaba le tuve que
// agregar Scalar al bound

// NOTE(elsuizo:2019-04-11): deberia devolver un Result???
/// Brief.
///
/// verify if a Array2<Float> is a propper rotation matrix
///
/// Function arguments:
/// R: Array2<Float>
///
pub fn is_rotation<SF: Scalar + Float>(R: &Array2<SF>) -> bool {
    let mut result = false;
    let shape = R.shape();
    if (shape[0] == shape[1]) && (shape[0] == 3) {
        // the determinant must be almost one
        if (R.det().unwrap() - SF::one()).abs() < SF::epsilon() {
            result = true;
        }
    } else {
        result = false;
    }

    result
/// Brief.
///
/// Verify if the Array2 is a proper rotation matrix
///
/// Function arguments:
/// R: Rotation matrix(Array2<Float> 3x3)
///
pub fn is_rotation<F: Float>(R: Array2<F>) -> bool {

}
