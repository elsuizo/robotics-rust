/// A Robotics library in Rust programming language
///

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
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use crate::transformations::{rotx, roty};
    use crate::utils::cross;
    use ndarray::arr1;
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
    }

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
}
