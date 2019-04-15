/// A Robotics library in Rust programming language
///

#[macro_use]
extern crate ndarray;
extern crate ndarray_linalg;
extern crate assert_approx_eq;
extern crate num;

// NOTE(elsuizo:2019-04-06): todavia no lo necesito
// use ndarray_linalg::*;
pub mod transformations;
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
    use crate::utils::cross;
    use ndarray::{arr1, arr2};
    use ndarray::prelude::*;
    // use ndarray_linalg::close_l1;

    #[test]
    fn test_rotx() {
        let a = rotx(360.0);
        let b = rotx(0.0);
        // NOTE(elsuizo:2019-04-06): tengo que ponerle que Float quiero probar porque sino no se da
        // cuenta y pone error que no sabe que float es para evaluar `abs()`
        // test all elements of the matrix
        for row in 0..3 {
            for column in 0..3 {
                assert_approx_eq!(a[[row, column]] as f64, b[[row, column]] as f64, 1.0e-6);
            }
        }
        // test property A^(-1) = A^(T)

        let a_inv = a.t();
        // identity matrix
        let I: Array2<f64> = Array::eye(3);
        // matrix product
        let I_star = a_inv.dot(&a);
        for row in 0..3 {
            for column in 0..3 {
                assert_approx_eq!(I_star[[row, column]] as f64, I[[row, column]] as f64, 1.0e-6);
            }
        }
    }

    #[test]
    fn test_roty() {
        let a = roty(90.0);
        let b = roty(90.0 + 360.0);
        for row in 0..3 {
            for column in 0..3 {
                assert_approx_eq!(a[[row, column]] as f64, b[[row, column]] as f64, 1.0e-6);
            }
        }
        // test property A^(-1) = A^(T)

        let a_inv = a.t();
        // identity matrix
        let I: Array2<f64> = Array::eye(3);
        // matrix product
        let I_star = a_inv.dot(&a);
        for row in 0..3 {
            for column in 0..3 {
                assert_approx_eq!(I_star[[row, column]] as f64, I[[row, column]] as f64, 1.0e-6);
            }
        }
    }

    #[test]
    fn test_rotz() {
        let a = rotz(90.0);
        let b = rotz(90.0 + 360.0);
        for row in 0..3 {
            for column in 0..3 {
                assert_approx_eq!(a[[row, column]] as f64, b[[row, column]] as f64, 1.0e-6);
            }
        }
        // test property A^(-1) = A^(T)

        let a_inv = a.t();
        // identity matrix
        let I: Array2<f64> = Array::eye(3);
        // matrix product
        let I_star = a_inv.dot(&a);
        for row in 0..3 {
            for column in 0..3 {
                assert_approx_eq!(I_star[[row, column]] as f64, I[[row, column]] as f64, 1.0e-6);
            }
        }
    }

    // TODO(elsuizo:2019-04-12): el test falla
    #[test]
    #[ignore]
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
    use crate::utils::{cross, is_rotation};
    use ndarray::arr1;
    use crate::transformations::{rotx, roty};
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_cross() {
        // testing X x Y = Z
        let u = arr1(&[1.0, 0.0, 0.0]);
        let v = arr1(&[0.0, 1.0, 0.0]);

        match cross(&u, &v) {
            Ok(w) => {
                let z = arr1(&[0.0, 0.0, 1.0]);
                for num in 0..3 {
                    assert_approx_eq!(w[num] as f64, z[num] as f64);
                }
            }
            Err(e) => println!("error: {}", e),
        }

    }

    #[test]
    fn test_is_rotation() {
        let R = rotx(90.0);
        assert_eq!(is_rotation(&R), true);
    }
}
