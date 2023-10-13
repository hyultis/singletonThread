#![allow(non_snake_case)]
#![allow(unused_parens)]

use std::sync::Arc;
use std::thread::{JoinHandle, sleep};
use std::time::{Duration, Instant};
use parking_lot::RwLock;
use thread_priority::ThreadBuilder;

pub type ThreadPriority = thread_priority::ThreadPriority;
pub type ThreadPriorityValue = thread_priority::ThreadPriorityValue;

pub struct SingletonThread
{
	_thread: Option<JoinHandle<()>>,
	_threadName: Option<String>,
	_threadPriority: ThreadPriority,
	_threadFunc: Arc<RwLock<Box<dyn FnMut() + Send + Sync + 'static>>>,
	_filterPrefunc: Box<dyn FnMut() -> bool + Send + Sync + 'static>,
	_durationmin: Duration,
	_loop: Arc<RwLock<bool>>,
}

impl SingletonThread
{
	/// create a new singletonThread
	pub fn new(mainFn: impl Fn() + Send + Sync + 'static) -> Self
	{
		return SingletonThread {
			_thread: None,
			_threadName: None,
			_threadPriority: ThreadPriority::Crossplatform(ThreadPriorityValue::default()),
			_threadFunc: Arc::new(RwLock::new(Box::new(mainFn))),
			_filterPrefunc: Box::new(||{true}),
			_durationmin: Duration::from_millis(17), // default 60 fps
			_loop: Arc::new(RwLock::new(false)),
		};
	}
	
	/// create a new singletonThread with a filter
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
			_durationmin: Duration::from_millis(17), // default 60 fps
			_loop: Arc::new(RwLock::new(false)),
		};
	}
	
	/// try to launch a new run, check if not running AND the filter function return true
	/// return if the thread have been launched
	pub fn thread_launch(&mut self) -> bool
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
		}
		
		return true;
	}
	
	/// minimum duration before another run can occur, default 166ms (60fps)
	pub fn setDuration(&mut self, duration: Duration)
	{
		self._durationmin = duration;
	}
	
	/// minimum duration before another run can occur, default 166ms (60fps)
	pub fn setDuration_FPS(&mut self,fps: u8)
	{
		let durationmillis = 1000/fps as u64;
		self._durationmin = Duration::from_millis(durationmillis);
	}
	
	/// define if the thread auto relaunch himself (technically it just use the same thread)
	pub fn setLoop(&mut self, canLoop: bool)
	{
		self._loop = Arc::new(RwLock::new(canLoop));
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
	
	fn ThreadFunc(arced: Arc<RwLock<Box<dyn FnMut() + Send + Sync>>>, arcloop: Arc<RwLock<bool>>, durationtowait: Duration)
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
			
			if !*arcloop.read()
			{
				break;
			}
		}
	}
}
