#![allow(non_camel_case_types)]
mod r#fn;

use dyn_clone::{clone_box, DynClone};

#[derive(Clone)]
pub struct λ(Box<dyn Lambdable>);

pub trait Lambdable: FnMut(λ) -> λ + DynClone + NotLambda {}
dyn_clone::clone_trait_object!(Lambdable);
impl<T> Lambdable for T where T: FnMut(λ) -> λ + DynClone + NotLambda {}

pub auto trait NotLambda {}
//impl !NotLambda for λ {}

pub type Lambda = λ;

impl λ
{
	pub fn new<T: Lambdable + 'static>(f: T) -> Self
	{
		λ(clone_box(&f))
	}
}

/* impl<T: Lambdable> From<T> for λ
{
	fn from(value: T) -> Self
	{
		λ(Box::new(value))
	}
} */
