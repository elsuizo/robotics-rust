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
///
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
///
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
///
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
/// Output:
/// R: Rotation matrix(Array2<Float>)
///
pub fn rot2trans<F: Float>(r: &Array2<F>) -> Array2<F> {
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
/// Output:
/// Array2<Float>: 4x4
///
pub fn trotx<F: Float>(angle: F) -> Array2<F> {
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
pub fn troty<F: Float>(angle: F) -> Array2<F> {
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
pub fn trotz<F: Float>(angle: F) -> Array2<F> {
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
pub fn euler2rot<F: Float>(angle_phi: F, angle_theta: F, angle_psi: F) -> Array2<F> {
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
pub fn angle_vector2rot<F: Float>(theta: F, vector: Array1<F>) -> Array2<F> {
    let c = theta.cos();
    let s = theta.sin();
    let comp = (F::one() - c);
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
pub fn rot2euler<F: Float>(R: Array2<F>) -> (F, F, F) {

    if R[[0,2]].abs() < F::epsilon() && R[[1, 2]].abs() < F::epsilon() {
        // singularity
        let phi   = F::zero();
        let sp    = F::zero();
        let cp    = F::one();
        let theta = (cp * R[[0, 2]] + sp * R[[1, 2]]).atan2(R[[2, 2]]);
        let psi   = (-sp * R[[0, 0]] + cp * R[[1, 0]]).atan2(-sp * R[[0, 1]] + cp * R[[1, 1]]);
        return (phi, theta, psi);
    } else {
        // non-singular
        let phi   = R[[1, 2]].atan2(R[[0, 2]]);
        let sp    = phi.sin();
        let cp    = phi.cos();
        let theta = (cp * R[[0, 2]] + sp * R[[1, 2]]).atan2(R[[2, 2]]);
        let psi   = (-sp * R[[0, 0]] + cp * R[[1, 0]]).atan2(-sp * R[[0, 1]] + cp * R[[1, 1]]);
        return (phi, theta, psi);
    }
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
pub fn euler2trans<F: Float>(phi: F, theta: F, psi: F) -> Array2<F> {
    rot2trans(&euler2rot(phi, theta, psi))
}
