use super::*;

impl FnOnce<(λ,)> for λ
{
	type Output = λ;

	extern "rust-call" fn call_once(mut self, args: (λ,)) -> Self::Output
	{
		self.0(args.0)
	}
}

impl FnMut<(λ,)> for λ
{
	extern "rust-call" fn call_mut(&mut self, args: (λ,)) -> Self::Output
	{
		self.0(args.0)
	}
}
