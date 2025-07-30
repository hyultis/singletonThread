#![allow(non_snake_case)]
#![allow(unused_parens)]

mod thread;
mod r#loop;

#[cfg(feature = "thread-priority")]
extern crate thread_priority;

use std::sync::Arc;
use std::thread::{sleep, JoinHandle};
use std::time::{Duration, Instant};
use parking_lot::Mutex;
#[cfg(feature = "thread-priority")]
use thread_priority::{ThreadPriority, ThreadPriorityValue};
use crate::r#loop::Loop;
use crate::thread::Thread;

pub struct SingletonThread
{
	_thread: Option<JoinHandle<()>>,
	_threadName: Option<String>,
	#[cfg(feature = "thread-priority")]
	_threadPriority: ThreadPriority,
	_threadFunc: Arc<Mutex<Box<dyn FnMut() + Send + Sync + 'static>>>,
	_filterPrefunc: Box<dyn FnMut() -> bool + Send + Sync + 'static>,
	_durationmin: Arc<Mutex<Duration>>,
	_loop: Arc<Mutex<Loop>>,
}

impl SingletonThread
{
	/// create a new singletonThread
	pub fn new(mainFn: impl Fn() + Send + Sync + 'static) -> Self
	{
		return Self::newFiltered(mainFn,||{true});
	}
	
	/// create a new singletonThread with a filter.
	/// filter is run each time, after testing if the thread is running or not,
	/// filter must return true if the thread can be launched, or false if not
	pub fn newFiltered(mainFn: impl Fn() + Send + Sync + 'static, filterFn: impl Fn() -> bool + Send + Sync + 'static) -> Self
	{
		return SingletonThread {
			_thread: None,
			_threadName: None,
			#[cfg(feature = "thread-priority")]
			_threadPriority: ThreadPriority::Crossplatform(ThreadPriorityValue::default()),
			_threadFunc: Arc::new(Mutex::new(Box::new(mainFn))),
			_filterPrefunc: Box::new(filterFn),
			_durationmin: Arc::new(Mutex::new(Duration::from_nanos(1))),
			_loop: Arc::new(Mutex::new(Loop::NO)),
		};
	}
	
	/// try to launch a new run, check if not running AND the filter function return true
	/// return if the thread have been launched
	pub fn thread_launch(&mut self) -> bool
	{
		return self.internal_thread_launch(false);
	}
	
	/// try to launch a new run, check if not running, AND the filter function returns true
	/// if it cannot be launched immediately, mark the class to run another time after the current one. can only mark one time.
	pub fn thread_launch_delayabe(&mut self)
	{
		self.internal_thread_launch(true);
	}
	
	/// minimum duration before another run can occur, default 1ns
	pub fn duration_set(&mut self, duration: Duration)
	{
		*self._durationmin.lock() = duration;
	}
	
	/// minimum duration before another run can occur from frame per second (60 fps = 16.666 ms), minimum 1 fps (1 ms)
	pub fn duration_setFPS(&mut self, fps: u8)
	{
		let durationmillis = 1000/fps.max(1) as u64;
		*self._durationmin.lock() = Duration::from_millis(durationmillis);
	}
	
	/// define if the thread auto relaunch himself (technically it just use the same thread)
	/// set to false if you want to stop a looped thread that running.
	pub fn loop_set(&mut self, canLoop: bool)
	{
		*self._loop.lock() = canLoop.into();
	}

	/// set the name of the thread.
	/// the name is updated only if the thread launch is completely relaunched (no change when a loop ends)
	pub fn thread_setName(&mut self, name: impl Into<String>)
	{
		self._threadName = Some(name.into());
	}

	/// get the name of the thread
	pub fn thread_getName(&mut self) -> &Option<String>
	{
		return &self._threadName;
	}

	/// set the priority of the thread
	#[cfg(feature = "thread-priority")]
	pub fn thread_setPriority(&mut self, priority: ThreadPriority)
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
				let mut tmp = self._loop.lock();
				match *tmp
				{
					Loop::NO => {*tmp = Loop::ONE}
					_ => {}
				}
			}
			return false;
		}
		
		let arced = self._threadFunc.clone();
		let arcloop = self._loop.clone();
		let durationtowait = self._durationmin.clone();
		let spawnfunc = move ||
			{
				Self::InternalThreadFunc(arced, arcloop, durationtowait);
			};

		let mut innerthread = Thread::default();
		innerthread.name_set(self._threadName.clone());
		#[cfg(feature = "thread-priority")]
		{
			innerthread.priority_set(self._threadPriority);
		}
		if let Ok(spawned) = innerthread.build(spawnfunc)
		{
			self._thread = Some(spawned);
			return true;
		}
		
		return false;
	}
	
	fn InternalThreadFunc(arced: Arc<Mutex<Box<dyn FnMut() + Send + Sync>>>, arcloop: Arc<Mutex<Loop>>, durationtowait: Arc<Mutex<Duration>>)
	{
		loop
		{
			let instant = Instant::now();
			{
				let tmp = &mut arced.lock();
				tmp();
			}
			
			let timeelapsed = instant.elapsed();
			let durationtowait = durationtowait.lock().clone();
			if(timeelapsed<durationtowait)
			{
				sleep(durationtowait - timeelapsed);
			}
			
			let mut tmp = arcloop.lock();
			match *tmp
			{
				Loop::NO => {return;}
				Loop::ONE => {
					*tmp=Loop::NO;
				}
				Loop::YES => {}
			}
		}
	}
}

impl Drop for SingletonThread
{
	fn drop(&mut self) {
		self.loop_set(false);
	}
}
