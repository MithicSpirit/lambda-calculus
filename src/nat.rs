use std::cell::UnsafeCell;

use crate::Lambda;

pub fn to_lambda(n: u128) -> Lambda
{
	Lambda::new(move |f: Lambda| -> Lambda {
		Lambda::new(move |mut x: Lambda| -> Lambda {
			for _ in 0..n {
				x = f(x);
			}
			return x;
		})
	})
}

pub unsafe fn from_lambda(f: Lambda) -> Option<u128>
{
	let mut counter = UnsafeCell::new(Some(0));
	let cptr = counter.get();

	let count = Lambda::new(move |g: Lambda| -> Lambda {
		unsafe { *cptr = (*cptr).and_then(|x: u128| x.checked_add(1)) };
		return g;
	});
	f.clone()(count)(Lambda::new(id));

	return *counter.get_mut();
}

fn id(f: Lambda) -> Lambda
{
	return f;
}

#[cfg(test)]
mod tests
{
	use super::*;

	#[test]
	fn id()
	{
		for x in 0..(u8::MAX as u128) {
			assert_eq!(Some(x), unsafe {
				from_lambda(to_lambda(x))
			});
		}
	}
}
