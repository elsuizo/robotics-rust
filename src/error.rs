/// Error sources
//-------------------------------------------------------------------------
//                        imports
//-------------------------------------------------------------------------
use std::error;
use std::fmt;

pub type Result<T1> = ::std::result::Result<T1, UtilError>;
// pub type Result<T2> = ::std::result::Result<T2, RoboticsError>;

// enum RoboticsError {
//     ,
// }

#[derive(Debug)]
pub enum UtilError {
    /// the arrays must be equal and dimension 3
    InvalidDimentionOrNotEq {
        len_u: usize,
        len_v: usize,
    }
}

impl fmt::Display for UtilError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UtilError::InvalidDimentionOrNotEq { len_u, len_v } => write!(f, "The arrays must be of dimention 3: len of u:({}) != len of v:({})", len_u, len_v),
        }
    }
}

impl error::Error for UtilError {
    fn description(&self) -> &str {
        "Ha ocurrido un error!!!"
    }
    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
