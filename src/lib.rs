/// A Robotics library in Rust programming language
///

// extern crate ndarray;
// extern crate ndarray_linalg;
extern crate nalgebra as na;
extern crate assert_approx_eq;
extern crate num;

// NOTE(elsuizo:2019-04-06): todavia no lo necesito
// use ndarray_linalg::*;
pub mod transformations;
// pub mod utils;
// pub mod error;
pub mod utils;
pub mod error;
pub mod types;

//-------------------------------------------------------------------------
//                        tests
//-------------------------------------------------------------------------

#[cfg(test)]
mod tests_transformations {
    use assert_approx_eq::assert_approx_eq;
    use crate::transformations::{rotx, roty, rotz, rot2euler, euler2rot, rot_euler_zyx};

    #[test]
    fn test_rotx() {
        let a = rotx(360.0);
        let b = rotx(0.0);
        // NOTE(elsuizo:2019-04-06): tengo que ponerle que Float quiero probar porque sino no se da
        // cuenta y pone error que no sabe que float es para evaluar `abs()`
        // test all elements of the matrix
        for row in 0..3 {
            for column in 0..3 {
                assert_approx_eq!(a[(row, column)] as f64, b[(row, column)] as f64, 1.0e-6);
            }
        }
        // NOTE(elsuizo:2019-04-12): the proper rotation matrix must be determinant 1
        assert_approx_eq!(a.determinant() as f64, 1.0, 1.0e-6);
        assert_approx_eq!(b.determinant() as f64, 1.0, 1.0e-6);
    }

    #[test]
    fn test_roty() {
        let a = roty(90.0);
        let b = roty(90.0 + 360.0);
        for row in 0..3 {
            for column in 0..3 {
                assert_approx_eq!(a[(row, column)] as f64, b[(row, column)] as f64, 1.0e-6);
            }
        }
        // NOTE(elsuizo:2019-04-12): the proper rotation matrix must be determinant 1
        assert_approx_eq!(a.determinant() as f64, 1.0, 1.0e-6);
        assert_approx_eq!(b.determinant() as f64, 1.0, 1.0e-6);
    }

    #[test]
    fn test_rotz() {
        let a = rotz(90.0);
        let b = rotz(90.0 + 360.0);
        for row in 0..3 {
            for column in 0..3 {
                assert_approx_eq!(a[(row, column)] as f64, b[(row, column)] as f64, 1.0e-6);
                }
        }
        assert_approx_eq!(a.determinant() as f64, 1.0, 1.0e-6);
        assert_approx_eq!(b.determinant() as f64, 1.0, 1.0e-6);
    }

    #[test]
    fn test_rot2euler() {
        let phi_in = 10.0;
        let theta_in = 20.0;
        let psi_in  = 30.0;
        let R = rot_euler_zyx(phi_in, theta_in, psi_in);
        let values = rot2euler(&R);
        assert_approx_eq!(values.0 as f64, phi_in, 1.0e-6);
        assert_approx_eq!(values.1 as f64, theta_in, 1.0e-6);
        assert_approx_eq!(values.2 as f64, psi_in, 1.0e-6);
    }

}

#[cfg(test)]
mod tests_utils {
    use crate::utils::{is_rotation};
    use crate::transformations::{rotx, roty};
    use assert_approx_eq::assert_approx_eq;
    use nalgebra::Vector3;

    // #[test]
    // fn test_cross() {
    //     // testing X x Y = Z
    //     let u = Vector3::new(1.0, 0.0, 0.0);
    //     let v = Vector3::new(0.0, 1.0, 0.0);
    //
    //     match cross(&u, &v) {
    //         Ok(w) => {
    //             let z = Vector3::new(0.0, 0.0, 1.0):
    //             for num in 0..3 {
    //                 assert_approx_eq!(w[(num)] as f64, z[(num)] as f64);
    //             }
    //         }
    //     }
    // }

    // #[test]
    // fn test_cross() {
    //     // testing X x Y = Z
    //     let u = arr1(&[1.0, 0.0, 0.0]);
    //     let v = arr1(&[0.0, 1.0, 0.0]);
    //
    //     match cross(&u, &v) {
    //         Ok(w) => {
    //             let z = arr1(&[0.0, 0.0, 1.0]);
    //             for num in 0..3 {
    //                 assert_approx_eq!(w[num] as f64, z[num] as f64);
    //             }
    //         }
    //         Err(e) => println!("error: {}", e),
    //     }
    //
    // NOTE(elsuizo:2019-04-11): este test no funciona por problemas con el metodo `det()`
    #[test]
    fn test_is_rotation() {
        let R = rotx(90.0);
        assert_eq!(is_rotation(&R), true);
    }
}
