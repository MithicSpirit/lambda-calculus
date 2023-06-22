use super::*;

impl FnOnce<(Lambda,)> for Lambda
{
	type Output = Lambda;

	extern "rust-call" fn call_once(self, args: (Lambda,)) -> Self::Output
	{
		self.0(args.0)
	}
}

impl FnMut<(Lambda,)> for Lambda
{
	extern "rust-call" fn call_mut(
		&mut self,
		args: (Lambda,),
	) -> Self::Output
	{
		self.0(args.0)
	}
}

impl Fn<(Lambda,)> for Lambda
{
	extern "rust-call" fn call(&self, args: (Lambda,)) -> Self::Output
	{
		self.0(args.0)
	}
}
