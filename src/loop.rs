#[derive(Copy, Clone, Debug)]
pub enum Loop
{
	NO,
	ONE,
	YES
}

impl From<bool> for Loop
{
	fn from(value: bool) -> Self {
		value.then(|| Loop::YES).unwrap_or(Loop::NO)
	}
}