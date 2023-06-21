use crate::lambda::*;

pub fn to_lambda(n: u128) -> Lambda
{
	Lambda::new(move |mut f: Lambda| -> Lambda {
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
	let counter = 0;
	let cptr: *const u128 = &counter;
	let count = Lambda::new(move |g: Lambda| -> Lambda {
		unsafe { *(cptr as *mut u128) += 1 };
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
