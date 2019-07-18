/// Transformations
//-------------------------------------------------------------------------
//                        imports
//-------------------------------------------------------------------------
use ndarray::prelude::*;
use num::Float;
use ndarray_linalg::types::Scalar;
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
///
pub fn rotx<SF:Scalar + Float>(angle: SF) -> Array2<SF> {
    let one = SF::one();
    let zero = SF::zero();
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
///
pub fn roty<SF:Scalar + Float>(angle: SF) -> Array2<SF> {
    let one = SF::one();
    let zero = SF::zero();
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
///
pub fn rotz<SF:Scalar + Float>(angle: SF) -> Array2<SF> {
    let one = SF::one();
    let zero = SF::zero();
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
/// Output:
/// R: Rotation matrix(Array2<Float>)
///
pub fn rot2trans<SF:Scalar + Float>(r: &Array2<SF>) -> Array2<SF> {
    let mut R = Array2::<SF>::zeros((4,4));
    for row in 0..R.rows() {
        for column in 0..R.cols() {
            R[[row, column]] = r[[row, column]];
        }
    }
    R[[3, 3]] = SF::one();
    return R;
}

/// Brief.
///
/// Compute the rotation around the `x` axis(in cartesian coordinates)
///
/// Function arguments:
///  `angle`: Float
///
/// Output:
/// Array2<Float>: 4x4
///
pub fn trotx<SF:Scalar + Float>(angle: SF) -> Array2<SF> {
    rot2trans(&rotx(angle.to_radians()))
}

/// Brief.
///
/// Compute the rotation around the `y` axis(in cartesian coordinates)
///
/// Function arguments:
/// `angle`: Float
///
/// Output:
/// Array2<Float>: 4x4
///
pub fn troty<SF:Scalar + Float>(angle: SF) -> Array2<SF> {
    rot2trans(&roty(angle.to_radians()))
}

/// Brief.
///
/// Compute the rotation around the `z` axis(in cartesian coordinates)
///
/// Function arguments:
///  `angle`: Float
///
/// Output:
/// Array2<Float>: 4x4
///
pub fn trotz<SF:Scalar + Float>(angle: SF) -> Array2<SF> {
    rot2trans(&rotz(angle.to_radians()))
}

/// Brief.
///
/// Compute the rotation matrix from euler angles from the convenction(ZYZ)
///
/// Function arguments:
/// phi: first euler angle
/// theta: second euler angle
/// psi: third euler angle
///
/// Output:
/// R: Rotation matrix(Array2<Float>)
///
pub fn euler2rot<SF:Scalar + Float>(angle_phi: SF, angle_theta: SF, angle_psi: SF) -> Array2<SF> {
    rotz(angle_phi) * roty(angle_theta) * rotz(angle_psi)
}

/// Brief.
///
/// Compute the Rotation matrix from an arbitrary axis and angle
///
/// Function arguments:
/// theta: angle of rotation(Float)
/// vector: axis of rotation(Array1<Float>)
///
/// Return:
/// R: Rotation matrix(Array2<Float>)
///
pub fn angle_vector2rot<SF:Scalar + Float>(theta: SF, vector: Array1<SF>) -> Array2<SF> {
    let c = theta.cos();
    let s = theta.sin();
    let comp = SF::one() - c;
    let v_x = vector[0];
    let v_y = vector[1];
    let v_z = vector[2];


    let R = arr2(&[ [v_x * v_x * comp + c, v_y * v_x * comp - v_z * s, v_z * v_x * comp + v_y * s],
                    [v_x * v_y * comp + v_z * s, v_y * v_y * comp + c, v_z * v_y * comp - v_x * s],
                    [v_x * v_z * comp - v_y * s, v_y * v_z * comp + v_x * s, v_z * v_z * comp + c], ]);
    return R
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
///
pub fn rot2euler<SF:Scalar + Float>(R: &Array2<SF>) -> (SF, SF, SF) {

    if R[[0,2]].abs() < SF::epsilon() && R[[1, 2]].abs() < SF::epsilon() {
        // singularity
        let phi   = SF::zero();
        let sp    = SF::zero();
        let cp    = SF::one();
        let theta = (cp * R[[0, 2]] + sp * R[[1, 2]]).atan2(R[[2, 2]]);
        let psi   = (-sp * R[[0, 0]] + cp * R[[1, 0]]).atan2(-sp * R[[0, 1]] + cp * R[[1, 1]]);
        return (phi.to_degrees(), theta.to_degrees(), psi.to_degrees());
    } else {
        // non-singular
        let phi   = R[[1, 2]].atan2(R[[0, 2]]);
        let sp    = phi.sin();
        let cp    = phi.cos();
        let theta = (cp * R[[0, 2]] + sp * R[[1, 2]]).atan2(R[[2, 2]]);
        let psi   = (-sp * R[[0, 0]] + cp * R[[1, 0]]).atan2(-sp * R[[0, 1]] + cp * R[[1, 1]]);
        return (phi.to_degrees(), theta.to_degrees(), psi.to_degrees());
    }
}


pub fn rot_euler_zyx<SF:Scalar + Float>(phi: SF, theta: SF, psi: SF) -> Array2<SF> {
    // NOTE(elsuizo:2019-04-18): el producto de matrices se hace con dot!!!
    (rotz(phi).dot(&roty(theta))).dot(&rotx(psi))
}

/// Brief.
///
/// Compute the rotation matrix from euler angles
///
/// Function arguments:
/// phi: first euler angle(Float)
/// theta: second euler angle(Float)
/// psi: third euler angle(Float)
///
/// Output:
/// R: Rotation matrix(Array2<Float>)
///
pub fn euler2trans<SF:Scalar + Float>(phi: SF, theta: SF, psi: SF) -> Array2<SF> {
    rot2trans(&euler2rot(phi, theta, psi))
}

