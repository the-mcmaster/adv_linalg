//! Adv_LinAlg_Core: The heart of Adv_LinAlg.
//! 
//! This crate store all the types and their implementations for Adv_LinAlg.
//! 
//! ## Basic types
//! The two main types in this library are the [Vector][crate::Vector] and the
//! [Matrix][crate::Matrix] type. For most simple use cases, these two types
//! are all one may ever need.
//! 
//! Other vector and matrix types are provided useful for more specific uses.
//! Regardless of the specific type, each vector type has all the same base
//! functionality as the base [Vector]. The same is true for all matrix types
//! as well.
//! 
//! ## Universal Functionality
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
//! ## Mutability
//! Other than [MutVector][crate::vectors::MutVector] and 
//! [MutMatrix][crate::matrices::MutMatrix], all vector and matrix types
//! are element-wise immutable. This is to discern a difference between 
//! identifier mutability and internal mutability.
//! 
//! Special implementations of addition and multiplication has been
//! provided for [MutVector][crate::vectors::MutVector] and 
//! [MutMatrix][crate::matrices::MutMatrix] to allow control of element-wise
//! mutability with identifier mutability.
//! ```
//! use adv_linalg_core::prelude::*;
//! use adv_linalg_core::vectors::MutVector;
//! 
//! let vector1 = vector![1., 2., 3.];
//! let vector2 = vector![4., 5., 6.];
//! 
//! let mut mut_vector1 = MutVector::from(&vector![0.0 ; 3]);
//! let mut mut_vector2 = MutVector::from(&vector![0.0 ; 3]);
//! 
//! // element-wise mutability doesn't apply since mut_vector is on the right side
//! let immut_add = &vector1 + &mut mut_vector1;
//! assert_eq!(immut_add, vector![1., 2., 3.]);
//! assert_eq!(mut_vector1, MutVector::from(&vector![0.0 ; 3]));
//! 
//! // element-wise mutability doesn't apply since mut_vector isn't borrowed mutably
//! let immut_add = &mut_vector1 + &vector1;
//! assert_eq!(immut_add, vector![1., 2., 3.]);
//! assert_eq!(mut_vector1, MutVector::from(&vector![0.0 ; 3]));
//! 
//! // result will be placed in mut_vector1's memory
//! &mut mut_vector1 + &vector1;
//! assert_eq!(mut_vector1, MutVector::from(&vector![1., 2., 3.]));
//! ```
//! 
//! ## Credit
//! This library is built upon a more basic linear algebra library named
//! [Simp_LinAlg].
//! 
//! [Simp_LinAlg]: <https://crates.io/crates/simp_linalg>

/// Module hosting all vector types
pub mod vectors;

/// Module hosting all matrix types
pub mod matrices;

/// Module hosting the [Error][crate::error::Error] enum
pub mod error;

pub mod traits;

/// Baseline functionality re-exports
pub mod prelude;

mod macros;

pub use crate::vectors::Vector;
pub use crate::matrices::Matrix;