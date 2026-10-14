/// ।। ॐ नमः शिवाय ।। \\\

/// @date_start 18th September, 2026

/// @learning
/// This projects is an attempt to dive into advanced Rust.
/// Aims to get better at multi-threading.
/// An attempt to build my own parallelism library.

use std::thread;
use std::sync::{mpsc, Arc, Mutex};
/// mpsc (Multi-Producer, Single-Consumer): This is a communication channel. Think of it like a mail chute. Many parts of your program (Multi-Producer) can drop tasks into the chute, but only one receiver (Single-Consumer) picks them up on the other side.
/// Arc (Atomically Reference Counted): In basic Rust, a value can only have one owner. But what if 4 different worker threads all need to share access to the same task receiver? Arc wraps data so it can be safely owned by multiple threads simultaneously. (The "Atomic" part means it's thread-safe).
/// Mutex (Mutual Exclusion): If multiple threads try to grab a task from the receiver at the exact same microsecond, data corruption can happen. A Mutex acts like a bathroom key: only one thread can hold the key, lock the receiver, pull a task out, and unlock it for the next thread.

type Job = Box<dyn FnOnce() + Send + 'static>;
/// Box: Puts data onto the heap. Since tasks (closures) can be of different sizes, putting them in a Box gives them a fixed size so we can store them in a queue easily.
/// dyn FnOnce(): This means "a closure that can be called at least (and exactly) once." In Rust, closures are just anonymous chunks of code you want to run later.
/// Send: A special marker trait telling Rust: "It is safe to pack this data up and move it across a thread boundary to another CPU core."
/// 'static: A lifetime constraint meaning the data inside the closure lives long enough and doesn't point to temporary local variables that might vanish while another thread is still running them.

pub struct MiniRayonPool {
  threads: u32,
  sender: Option<mpsc::Sender<Job>>,
}

impl MiniRayonPool {
  fn get_cores() -> u32 {
    match thread::available_parallelism() {
      Ok(count) => count.get() as u32, // .get() extracts inner usize from NonZero<usize>
      Err(_) => 1
    }
  }
  fn new() -> Self {
    MiniRayonPool { threads: Self::get_cores() }
  }
}

fn main() {
  let pools = MiniRayonPool::new();
  println!("Available cores: {}.", pools.threads);
}