/// Transformations
//-------------------------------------------------------------------------
//                        imports
//-------------------------------------------------------------------------
// use ndarray::prelude::*;
use na::{Matrix3, Matrix4, RowVector3, Scalar};
use alga::general::RingCommutative;
use num::Float;
use core::fmt::Debug;
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
pub fn rotx<SF: Scalar + Float>(angle: SF) -> Matrix3<SF> {
    let one = SF::one();
    let zero = SF::zero();
    let c = angle.to_radians().cos();
    let s = angle.to_radians().sin();

    let Rotation = Matrix3::from_rows(&[
                        RowVector3::new(one, zero, zero),
                        RowVector3::new(zero,   c,  -s),
                        RowVector3::new(zero,   s,   c),]);
    return Rotation;
}


/// Brief.
///
/// Compute the rotation around the `y` axis(in cartesian coordinates)
///
/// Description
///
/// * `angle` - Angle of rotation in degrees
pub fn roty<SF: Scalar + Float>(angle: SF) -> Matrix3<SF> {
    let one = SF::one();
    let zero = SF::zero();
    let c = angle.to_radians().cos();
    let s = angle.to_radians().sin();
    let Rotation = Matrix3::from_rows(&[
                        RowVector3::new(c,     zero,      s),
                        RowVector3::new(zero,   one,   zero),
                        RowVector3::new(-s,    zero,      c),]
                                      );
    return Rotation
}

/// Brief.
///
/// Compute the rotation around the `z` axis(in cartesian coordinates)
///
/// Description
///
/// * `angle` - Angle of rotation in degrees
pub fn rotz<SF:Scalar + Float>(angle: SF) -> Matrix3<SF> {
    let one = SF::one();
    let zero = SF::zero();
    let c = angle.to_radians().cos();
    let s = angle.to_radians().sin();
    let Rotation = Matrix3::from_rows(&[
                        RowVector3::new(c,     -s,      zero),
                        RowVector3::new(s,      c,      zero),
                        RowVector3::new(zero, zero,      one),]
                                      );
    return Rotation
}

/// Brief.
///
/// Convert a Rotation Matrix to a
///
/// Function arguments:
/// `r`: Array2<Float>
///
fn rot2trans<SF:Scalar + Float>(r: &Matrix3<SF>) -> Matrix4<SF> {
    let mut R = Matrix4::zeros();
    for row in 0..3 {
        for column in 0..3 {
            R[(row, column)] = r[(row, column)];
        }
    }
    R[(3, 3)] = SF::one();
    return R;
}

/// Brief.
///
/// Compute the rotation around the `x` axis(in cartesian coordinates)
///
/// Function arguments:
///  `angle`: Float
///
fn trotx<SF:Scalar + Float>(angle: SF) -> Matrix4<SF> {
    rot2trans(&rotx(angle.to_radians()))
}

fn troty<SF:Scalar + Float>(angle: SF) -> Matrix4<SF> {
    rot2trans(&roty(angle.to_radians()))
}

fn trotz<SF:Scalar + Float>(angle: SF) -> Matrix4<SF> {
    rot2trans(&rotz(angle.to_radians()))
}

fn euler2rot<SF:Scalar + Float + RingCommutative>(angle_phi: SF, angle_theta: SF, angle_psi: SF) -> Matrix3<SF> {
    rotz(angle_phi) * roty(angle_theta) * rotz(angle_psi)
}


// /// Brief.
// ///
// /// Compute the euler angles from a Rotation matrix(ZYZ convention)
// ///
// /// Function arguments:
// /// `R`: Rotation matrix
// ///
// /// Output:
// /// A tuple with the angles: phi, theta, psi
// pub fn rot2euler<F: Float>(R: Array2<F>) -> (F, F, F) {
//     if
// }
