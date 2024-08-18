#![allow(non_snake_case)]
#![allow(unused_parens)]

use std::sync::Arc;
use std::thread::{JoinHandle, sleep};
use std::time::{Duration, Instant};
use parking_lot::RwLock;
use thread_priority::ThreadBuilder;

pub type ThreadPriority = thread_priority::ThreadPriority;
pub type ThreadPriorityValue = thread_priority::ThreadPriorityValue;

#[derive(Copy, Clone, Debug)]
enum Loop
{
	NO,
	ONE,
	YES
}

pub struct SingletonThread
{
	_thread: Option<JoinHandle<()>>,
	_threadName: Option<String>,
	_threadPriority: ThreadPriority,
	_threadFunc: Arc<RwLock<Box<dyn FnMut() + Send + Sync + 'static>>>,
	_filterPrefunc: Box<dyn FnMut() -> bool + Send + Sync + 'static>,
	_durationmin: Duration,
	_loop: Arc<RwLock<Loop>>,
}

impl SingletonThread
{
	/// create a new singletonThread
	pub fn new(mainFn: impl Fn() + Send + Sync + 'static) -> Self
	{
		return Self::newFiltered(mainFn,||{true});
	}
	
	/// create a new singletonThread with a filter.
	/// filter is run each time, after testing if thread is running or not
	/// filter must return true if the thread can be launched, or false if not
	pub fn newFiltered(mainFn: impl Fn() + Send + Sync + 'static, filterFn: impl Fn() -> bool + Send + Sync + 'static) -> Self
	{
		return SingletonThread {
			_thread: None,
			_threadName: None,
			_threadPriority: ThreadPriority::Crossplatform(ThreadPriorityValue::default()),
			_threadFunc: Arc::new(RwLock::new(Box::new(mainFn))),
			_filterPrefunc: Box::new(filterFn),
			_durationmin: Duration::from_nanos(1),
			_loop: Arc::new(RwLock::new(Loop::NO)),
		};
	}
	
	/// try to launch a new run, check if not running AND the filter function return true
	/// return if the thread have been launched
	pub fn thread_launch(&mut self) -> bool
	{
		return self.internal_thread_launch(false);
	}
	
	/// try to launch a new run, check if not running AND the filter function return true
	/// if it cannot be launched immediately, mark the class to run another time after the current one. can only mark one time.
	pub fn thread_launch_delayabe(&mut self)
	{
		self.internal_thread_launch(true);
	}
	
	/// minimum duration before another run can occur, default 1ns
	pub fn setDuration(&mut self, duration: Duration)
	{
		self._durationmin = duration;
	}
	
	/// minimum duration before another run can occur
	pub fn setDuration_FPS(&mut self,fps: u8)
	{
		let durationmillis = 1000/fps as u64;
		self._durationmin = Duration::from_millis(durationmillis);
	}
	
	/// define if the thread auto relaunch himself (technically it just use the same thread)
	/// set to false if you want to stop a looped thread that running.
	pub fn setLoop(&mut self, canLoop: bool)
	{
		if(canLoop)
		{
			*self._loop.write() = Loop::YES;
		}
		else
		{
			*self._loop.write() = Loop::NO;
		}
	}
	
	pub fn setThreadName(&mut self, name: impl Into<String>)
	{
		let name = name.into();
		self._threadName = Some(name);
	}
	
	pub fn setThreadPriority(&mut self, priority: ThreadPriority)
	{
		self._threadPriority = priority;
	}
	
	////// PRIVATE
	
	
	fn internal_thread_launch(&mut self,delayabe: bool) -> bool
	{
		let mut canLaunch = match &self._thread {
			None => true,
			Some(threadinfo) => {
				threadinfo.is_finished()
			}
		};
		
		if (canLaunch)
		{
			let tmp = &mut self._filterPrefunc;
			canLaunch = tmp();
		}
		
		if (!canLaunch)
		{
			if(delayabe)
			{
				let tmp = {*self._loop.clone().read()};
				match tmp
				{
					Loop::NO => {*self._loop.clone().write() = Loop::ONE}
					_ => {}
				}
			}
			return false;
		}
		
		let arced = self._threadFunc.clone();
		let arcloop = self._loop.clone();
		let durationtowait = self._durationmin.clone();
		let spawnfunc = move |_|
			{
				Self::ThreadFunc(arced,arcloop,durationtowait);
			};
		let mut thread = ThreadBuilder::default();
		if let Some(name) = &self._threadName
		{
			thread = thread.name(name.to_string());
		}
		thread = thread.priority(self._threadPriority);
		self._thread = None;
		if let Ok(spawned) = thread.spawn(spawnfunc)
		{
			self._thread = Some(spawned);
			return true;
		}
		
		return false;
	}
	
	fn ThreadFunc(arced: Arc<RwLock<Box<dyn FnMut() + Send + Sync>>>, arcloop: Arc<RwLock<Loop>>, durationtowait: Duration)
	{
		loop
		{
			let instant = Instant::now();
			{
				let tmp = &mut arced.write();
				tmp();
			}
			
			let timeelapsed = instant.elapsed();
			if(timeelapsed<durationtowait)
			{
				sleep(durationtowait - timeelapsed);
			}
			
			let tmp = {*arcloop.read()};
			match tmp
			{
				Loop::NO => {return;}
				Loop::ONE => {
					*arcloop.write()=Loop::NO;
				}
				Loop::YES => {}
			}
		}
	}
}

impl Drop for SingletonThread
{
	fn drop(&mut self) {
		self.setLoop(false);
	}
}
