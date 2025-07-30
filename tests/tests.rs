#![allow(non_snake_case)]
#![allow(unused_parens)]

use std::sync::Arc;
#[cfg(feature = "thread-priority")]
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::sleep;
use std::time::{Duration, Instant};
use parking_lot::RwLock;
#[cfg(feature = "thread-priority")]
use thread_priority::ThreadPriority;
use singletonThread::SingletonThread;

#[test]
fn simple() {
	let testdefault = Arc::new(RwLock::new(0));
	let cloneintoThread = testdefault.clone();
	let mut testThread = SingletonThread::new(move || {
		let mut binding = cloneintoThread.write();
		println!("i'm running ! {}",*binding);
		*binding += 1;
	});
	
	let starttime = Instant::now();
	while (*testdefault.read() < 15)
	{
		testThread.thread_launch();
	}
	
	println!("duration \"simple\" : {}us",starttime.elapsed().as_micros());
	assert_eq!(*testdefault.read(), 15);
}

#[test]
fn looping() {
	let testdefault = Arc::new(RwLock::new(0));
	let cloneintoThread = testdefault.clone();
	let mut testThread = SingletonThread::new(move || {
		let clone = cloneintoThread.clone();
		let mut binding = clone.write();
		if(*binding<15)
		{
			*binding += 1;
		}
	});
	testThread.loop_set(true);
	
	let starttime = Instant::now();
	testThread.thread_launch();
	while (*testdefault.read() < 15)
	{
	}
	// the thread is still running here, but it's out of scope for the test, so we stop it
	testThread.loop_set(false);

	println!("duration \"looping\" : {}us",starttime.elapsed().as_micros());
	assert_eq!(*testdefault.read(), 15);
}

#[test]
fn delayed() {
	let state = Arc::new(RwLock::new(0u8));
	let cloneintoThread = state.clone();
	let mut testThread = SingletonThread::new(move || {
		sleep(Duration::from_micros(500));
		*cloneintoThread.write()+=1;
	});
	
	let starttime = Instant::now();
	testThread.thread_launch();
	testThread.thread_launch_delayabe();
	
	while(*state.read()<2)
	{
		//println!("state : {}",state.read());
	}

	println!("duration \"delayed\" : {}us",starttime.elapsed().as_micros());
	assert_eq!(*state.read(), 2);
}

#[cfg(feature = "thread-priority")]
#[test]
fn priority() {
	let boolean = Arc::new(AtomicBool::new(false));
	let innerboolean = boolean.clone();
	let mut testThread = SingletonThread::new(move || {
		innerboolean.store(true, Ordering::Relaxed);
	});

	testThread.thread_setPriority(ThreadPriority::Max);
	testThread.thread_launch();

	sleep(Duration::from_millis(150));

	assert_eq!(boolean.load(Ordering::Relaxed), true);
}