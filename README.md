# SingletonThread

A library using a FnMut you defined and run it on a thread that follow a singleton pattern :
- At any time, only one instance of the thread can be run.
- to run again, the FnMut must have finished

Some tool allow you to define how you want to rerun the thread :

- setDuration() or setDuration_FPS() : define a minimum time between to run. (default 1ns)
- thread_launch() : try to rerun the thread, do nothing if the thread is already running
- setLoop() : loop the thread


## Online Documentation

[Master branch](https://github.com/hyultis/singletonThread)

## Example

You can check the test as example, here : https://github.com/hyultis/singletonThread/blob/master/tests/tests.rs

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 licence, shall be
dual licensed as above, without any additional terms or conditions.
