#![allow(non_snake_case)]
#![allow(unused_parens)]

use std::sync::Arc;
use std::time::Instant;
use parking_lot::RwLock;
use singletonThread::SingletonThread;

#[test]
fn simple() {
	let testdefault = Arc::new(RwLock::new(0));
	let cloneintoThread = testdefault.clone();
	let mut testThread = SingletonThread::new(move || {
		println!("i'm running !");
		*cloneintoThread.clone().write() += 1;
	});
	
	let starttime = Instant::now();
	while (*testdefault.read() < 30)
	{
		testThread.thread_launch();
	}
	
	println!("duration : {}ms",starttime.elapsed().as_millis());
	assert_eq!(*testdefault.read(), 30);
}
