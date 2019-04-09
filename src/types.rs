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
    shape: (usize, usize)
}

// pub enum RotationType<F> {
//     Homogeneous<F>,
//     Rotation<F>
// }


impl<F: Float> Homogeneous<F> {
    pub fn new(dim: (usize, usize)) -> Self {
        let m = Array2::<F>::zeros(dim);
        Homogeneous {
            data: m,
            shape: dim,
        }
    }
}
