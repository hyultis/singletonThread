# SingletonThread

`SingletonThread` is a Rust library designed to manage the execution of a user-defined `FnMut` function on a thread following the **singleton pattern**. This ensures that at any given time, only **one instance of the thread** can be active. Before starting a new execution, the previous one must have completed, providing fine-grained control for thread-safe operations.

This library offers functionality to control and limit thread execution frequency, making it ideal for scenarios where precise timing or controlled thread reuse is needed.

## Features

- **Singleton Thread Execution**: Ensures that only one thread instance runs at a time.
- **Flexible Rerun Control**:
  - `setDuration()` or `setDuration_FPS()`: Specify a minimum time interval between successive thread executions (default: 1 ns).
  - `thread_launch()`: Attempt to relaunch the thread while ensuring that only a single thread instance is running.
  - `setLoop()`: Create a loop that continuously runs the thread.
- Prevents overlapping execution of threads, ensuring consistent usage.

## Getting Started

### Installation

```toml
[dependencies]
singletonThread = "2.0"
```

### features

* thread_priority : allow setting thread priority via `thread-priority` crate

## Example Usage

To see the library in use, check out the test cases provided: [tests.rs](https://github.com/hyultis/singletonThread/blob/master/tests/tests.rs).

Below is a brief example showcasing the usage of the library:

```rust
fn main() 
{ 
	let mut test_thread = SingletonThread::new(move || {
		println!("Running my function in a thread!");
	});

	// Launch the thread.
	test_thread.thread_launch();
}
```

## License

Licensed under either of the following:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
- MIT license ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
