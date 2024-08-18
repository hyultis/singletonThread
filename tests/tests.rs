#![allow(non_snake_case)]
#![allow(unused_parens)]

use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, Instant};
use parking_lot::RwLock;
use singletonThread::SingletonThread;

#[test]
fn simple() {
	let testdefault = Arc::new(RwLock::new(0));
	let cloneintoThread = testdefault.clone();
	let mut testThread = SingletonThread::new(move || {
		let clone = cloneintoThread.clone();
		let mut binding = clone.write();
		println!("i'm running ! {}",*binding);
		*binding += 1;
	});
	
	let starttime = Instant::now();
	while (*testdefault.read() < 15)
	{
		testThread.thread_launch();
	}
	
	println!("duration : {}ms",starttime.elapsed().as_millis());
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
	testThread.setLoop(true);
	
	let starttime = Instant::now();
	testThread.thread_launch();
	while (*testdefault.read() < 15)
	{
	}
	// thread is still running here, but it's out of scope for the test
	
	println!("duration : {}ms",starttime.elapsed().as_millis());
	assert_eq!(*testdefault.read(), 15);
}

#[test]
fn delayed() {
	let state = Arc::new(RwLock::new(0u8));
	let cloneintoThread = state.clone();
	let mut testThread = SingletonThread::new(move || {
		sleep(Duration::from_millis(500));
		let clone = cloneintoThread.clone();
		*clone.write()+=1;
	});
	
	let starttime = Instant::now();
	testThread.thread_launch();
	testThread.thread_launch_delayabe();
	
	while(*state.read()<2)
	{
		//println!("state : {}",state.read());
	}
	
	println!("duration : {}ms",starttime.elapsed().as_millis());
	assert_eq!(*state.read(), 2);
}
