/// Utility functions and algorithms
//-------------------------------------------------------------------------
//                        imports
//-------------------------------------------------------------------------
use ndarray::prelude::*;
use num::Float;
use super::error::UtilError;

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
        w[0] = (u[1] * v[2] - u[2] * v[1]);
        w[1] = (u[2] * v[1] - u[0] * v[2]);
        w[2] = (u[0] * v[1] - u[1] * v[0]);

        return Ok(w);
    }
    else {
        return Err(UtilError::InvalidDimentionOrNotEq{
            len_u: u.len(),
            len_v: v.len(),
        })
    }
}
