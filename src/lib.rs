//! Adv_LinAlg: An optimizable linear algebra.
//! 
//! This library is based on a more basic linear algebra library named
//! [Simp_LinAlg].
//! 
//! This library aims to give the tools to developers to optimize vector
//! and matrix operations as far as they like.
//! 
//! The two main types in this library are the [Vector][crate::Vector] and the
//! [Matrix][crate::Matrix] type. For most simple use cases, these two types
//! are all one may ever need.
//! 
//! Other vector and matrix types are provided useful for more specific uses.
//! Regardless of the specific type, each vector type has all the same base
//! functionality as the base [Vector]. The same is true for all matrix types
//! as well.
//! 
//! The following operations are supported between all vector types:
//! - Addition ([+][std::ops::Add])
//! - Dot Product ([*][std::ops::Mul])
//! - Scalar Multiplication ([*][std::ops::Mul])
//! - Lambda Functionality ([Lambda][crate::traits::Lambda])
//! - Map Functionality ([Map][crate::traits::Map])
//! 
//! Similarly, the following operations are supported between all matrix types:
//! - Addition ([+][std::ops::Add])
//! - Matrix Multiplication ([*][std::ops::Mul])
//! - Scalar Multiplication ([*][std::ops::Mul])
//! - Lambda Functionality ([Lambda][crate::traits::Lambda])
//! - Map Functionality ([Map][crate::traits::Map])
//! 
//! Lastly, the following operations are supported between all 
//! matrix/vector types:
//! - Matrix Transformation ([*][std::ops::Mul])
//! 
//! [Simp_LinAlg]: <https://crates.io/crates/simp_linalg>

/// Module hosting all vector types
pub mod vectors;

/// Module hosting all matrix types
pub mod matrices;

/// Module hosting the [Error][crate::error::Error] enum
pub mod error;

/// Module hosting all available traits, especially
/// [Lambda][crate::traits::Lambda] and [Map][crate::traits::Map].
pub mod traits;

/// Baseline functionality re-exports
pub mod prelude;

mod macros;

pub use crate::vectors::Vector;
pub use crate::matrices::Matrix;