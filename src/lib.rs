#![feature(unboxed_closures, fn_traits)]
#![feature(auto_traits, negative_impls)]

pub mod lambda;
pub mod nat;

pub use lambda::{λ, Lambda};
