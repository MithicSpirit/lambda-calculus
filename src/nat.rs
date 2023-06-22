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

pub fn from_lambda(f: Lambda) -> u128
{
	#[allow(unused_mut)]
	let mut counter = 0;
	let cptr = &counter as *const u128 as *mut u128;
	let count = Lambda::new(move |g: Lambda| -> Lambda {
		unsafe { *cptr += 1 };
		return g;
	});
	f.clone()(count)(Lambda::new(id));
	return counter;
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
		for x in 0..1000 {
			assert_eq!(x, from_lambda(to_lambda(x)));
		}
	}
}
