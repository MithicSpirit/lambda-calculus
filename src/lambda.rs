#![allow(non_camel_case_types)]
mod r#fn;

use std::rc::Rc;

#[derive(Clone)]
pub struct λ(Rc<dyn Lambdable>); // defined as λ for nice compiler output
pub type Lambda = λ;

pub trait Lambdable: Fn(Lambda) -> Lambda + NotLambda {}
impl<T> Lambdable for T where T: Fn(Lambda) -> Lambda + NotLambda {}

pub auto trait NotLambda {}
//impl !NotLambda for Lambda {}

impl Lambda
{
	pub fn new<T: Lambdable + 'static>(f: T) -> Self
	{
		λ(Rc::new(f))
	}
}

/* impl<T: Lambdable> From<T> for Lambda
{
	fn from(value: T) -> Self
	{
		Lambda(Box::new(value))
	}
} */
