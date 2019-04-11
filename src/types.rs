/// Common types
//-------------------------------------------------------------------------
//                        imports
//-------------------------------------------------------------------------
use ndarray::prelude::*;
use num::Float;

//-------------------------------------------------------------------------
//                        code
//-------------------------------------------------------------------------
#[derive(Debug)]
pub struct Homogeneous<F> {
    data: Array2<F>,
}

pub struct Rotation<F> {
    data: Array2<F>,
}

// enum Rotationlalala<T> {
//
//     Homogeneous<T>,
//     Rotation<T>
// }


impl<F: Float> Homogeneous<F> {
    pub fn new() -> Self {
        let m = Array2::<F>::zeros((3,3));
        Homogeneous {
            data: m,
        }
    }
}
