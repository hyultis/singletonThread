use std::thread::JoinHandle;
#[cfg(not(feature = "thread-priority"))]
use std::thread::Builder;
#[cfg(feature = "thread-priority")]
use thread_priority::{ThreadBuilder, ThreadPriorityValue, ThreadPriority};

pub struct Thread
{
	_name: Option<String>,
	#[cfg(feature = "thread-priority")]
	_inner: ThreadPriority,
}

impl Thread
{
	#[cfg(not(feature = "thread-priority"))]
	pub(crate) fn build(&self, func: impl FnOnce() + Send + Sync + 'static ) -> std::io::Result<JoinHandle<()>>
	{
		let mut thread = Builder::new();
		if let Some(name) = &self._name
		{
			thread = thread.name(name.to_string());
		}

		return thread.spawn(func);
	}

	#[cfg(feature = "thread-priority")]
	pub(crate) fn build(&self, func: impl FnOnce() + Send + Sync + 'static ) -> std::io::Result<JoinHandle<()>>
	{
		let mut thread = ThreadBuilder::default();
		if let Some(name) = &self._name
		{
			thread = thread.name(name.to_string());
		}
		thread = thread.priority(self._inner);
		return thread.spawn(|_|func());
	}

	#[cfg(feature = "thread-priority")]
	pub(crate) fn priority_set(&mut self, priority: ThreadPriority)
	{
		self._inner = priority;
	}

	pub(crate) fn name_set(&mut self, name: Option<String>)
	{
		self._name = name;
	}
}


impl Default for Thread
{
	fn default() -> Self {
		#[cfg(not(feature = "thread-priority"))]
		return Thread { _name: None };

		#[cfg(feature = "thread-priority")]
		return Thread {
			_name: None,
			_inner: ThreadPriority::Crossplatform(ThreadPriorityValue::default())
		};
	}
}