/// Common types
//-------------------------------------------------------------------------
//                        imports
//-------------------------------------------------------------------------
use ndarray::prelude::*;
use num::Float;

//-------------------------------------------------------------------------
//                        code
//-------------------------------------------------------------------------
type RotationMatrix<SF> = Array2<SF>; // type alias for readability

#[derive(Debug)]
pub struct Pose<F> {
    Rotation: RotationMatrix<SF>,
    coordinate_frame_name: &'static str
}
