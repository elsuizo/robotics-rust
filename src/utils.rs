/// Utility functions and algorithms
//-------------------------------------------------------------------------
//                        imports
//-------------------------------------------------------------------------
use num::Float;
use super::error::UtilError;
use nalgebra::{Vector3, Matrix3, Scalar};
use alga::general::RingCommutative;

// NOTE(elsuizo:2019-04-12): creo que no hace falta ya que nalgebra tiene cross
//-------------------------------------------------------------------------
//                        code
//-------------------------------------------------------------------------
// /// brief.
// ///
// /// Compute the cross product of two Arrays of 3D
// pub fn cross<SF:Scalar + Float>(u: Vector3<SF>, v: Vector3<SF>) -> Result<Vector3<SF>, UtilError> {
//     // NOTE(elsuizo:2019-04-07): no olvidar que como estamos haciendo el codigo generico tenemos
//     // que hacerlo tambien cuando inicializamos, por eso utilizamos F::zero()
//     if ((u.len() == v.len()) && (u.len() == 3)) {
//         let mut w = arr1(&[F::zero(), F::zero(), F::zero()]);
//         w[0] = u[1] * v[2] - u[2] * v[1];
//         w[1] = u[2] * v[1] - u[0] * v[2];
//         w[2] = u[0] * v[1] - u[1] * v[0];
//
//         return Ok(w);
//     }
//     else {
//         return Err(UtilError::InvalidDimentionOrNotEq{
//             len_u: u.len(),
//             len_v: v.len(),
//         })
//     }
// }

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
