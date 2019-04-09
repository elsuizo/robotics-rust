/// Transformations
//-------------------------------------------------------------------------
//                        imports
//-------------------------------------------------------------------------
use ndarray::prelude::*;
use num::Float;
//-------------------------------------------------------------------------
//                        code
//-------------------------------------------------------------------------
/// brief.
///
/// compute the rotation around the `x` axis(in cartesian coordinates)
///
/// description
///
/// * `angle` - angle of rotation in degrees
pub fn rotx<F: Float>(angle: F) -> Array2<F> {
    let one = F::one();
    let zero = F::zero();
    let c = angle.to_radians().cos();
    let s = angle.to_radians().sin();
    arr2(&[[one, zero, zero],
         [ zero,   c,  -s],
         [zero,   s,   c]])
}


/// Brief.
///
/// Compute the rotation around the `y` axis(in cartesian coordinates)
///
/// Description
///
/// * `angle` - Angle of rotation in degrees
pub fn roty<F: Float>(angle: F) -> Array2<F> {
    let one = F::one();
    let zero = F::zero();
    let c = angle.to_radians().cos();
    let s = angle.to_radians().sin();
    arr2(&[[c,   zero,   s],
           [zero, one, zero],
           [-s,  zero,   c]])
}

/// Brief.
///
/// Compute the rotation around the `z` axis(in cartesian coordinates)
///
/// Description
///
/// * `angle` - Angle of rotation in degrees
pub fn rotz<F: Float>(angle: F) -> Array2<F> {
    let one = F::one();
    let zero = F::zero();
    let c = angle.to_radians().cos();
    let s = angle.to_radians().sin();
    arr2(&[[c,   -s,   zero],
           [s,    c,   zero],
           [zero,  zero, one]])
}

/// Brief.
///
/// Convert a Rotation Matrix to a
///
/// Function arguments:
/// `r`: Array2<Float>
///
fn rot2trans<F: Float>(r: &Array2<F>) -> Array2<F> {
    let mut R = Array2::<F>::zeros((4,4));
    for row in 0..3 {
        for column in 0..3 {
            R[[row, column]] = r[[row, column]];
        }
    }
    R[[3, 3]] = F::one();
    return R;
}

/// Brief.
///
/// Compute the rotation around the `x` axis(in cartesian coordinates)
///
/// Function arguments:
///  `angle`: Float
///
fn trotx<F: Float>(angle: F) -> Array2<F> {
    rot2trans(&rotx(angle.to_radians()))
}

fn troty<F: Float>(angle: F) -> Array2<F> {
    rot2trans(&roty(angle.to_radians()))
}

fn trotz<F: Float>(angle: F) -> Array2<F> {
    rot2trans(&rotz(angle.to_radians()))
}

fn euler2rot<F: Float>(angle_phi: F, angle_theta: F, angle_psi: F) -> Array2<F> {
    rotz(angle_phi) * roty(angle_theta) * rotz(angle_psi)
}


/// Brief.
///
/// Compute the euler angles from a Rotation matrix(ZYZ convention)
///
/// Function arguments:
/// `R`: Rotation matrix
///
/// Output:
/// A tuple with the angles: phi, theta, psi
pub fn rot2euler<F: Float>(R: Array2<F>) -> (F, F, F) {
    if
}
