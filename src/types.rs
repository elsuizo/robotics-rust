/// Common types
//-------------------------------------------------------------------------
//                        imports
//-------------------------------------------------------------------------
use ndarray::prelude::*;
use num::Float;

//-------------------------------------------------------------------------
//                        code
//-------------------------------------------------------------------------
//

#[derive(Debug, Clone)]
struct Point3D<T> {
    x: T,
    y: T,
    z: T
}

#[derive(Debug, Clone)]
struct Point2D<T> {
    x: T,
    y: T
}

#[derive(Debug, Clone)]
struct Pose3D<F> {
    theta: F,
    p: Point3D<F>,
    Rotation: Array2<F>
}
