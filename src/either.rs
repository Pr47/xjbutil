//! A simple `Either` structure
//!
//! This structure is in effect isomorphic with `Result`, but does not have certain traits
//! implemented. Personally I think this is the better choice for expressing something which is not
//! really a `Result`.

use std::fmt::{Debug, Formatter};

pub enum Either<T1, T2> {
    Left(T1),
    Right(T2)
}

impl<E1, E2> Debug for Either<E1, E2>
    where E1: Debug,
          E2: Debug
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Either::Left(left) => {
                write!(f, "Left(")?;
                left.fmt(f)?;
                write!(f, ")")
            },
            Either::Right(right) => {
                write!(f, "Right(")?;
                right.fmt(f)?;
                write!(f, ")")
            }
        }
    }
}
