// ।। ॐ नमः शिवाय ।। \\

// @date 20th September, 2026
// @learning @building @experimenting
// Moving forward with my Rayon clone.
// The previous attempt spawned threads on the spot on function call & automatically destroyed them.
// Such approach is weak & expensive.
// This attempt aims to spawn global thread pool at once during program start.
// Then use them over-&-over by simply allocating data & passing functions to it.

use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::time::{Instant, Duration};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct MiniThreadPool {
  sender: mpsc::Sender<Job>,
  threads: Vec<thread::JoinHandle<()>>,
}

impl MiniThreadPool {
  // << initialize the pool at the very start >> \\
  fn awake(num: u32) -> Self {
    // << get a global walkie-talkie >> \\
    // << all threads communicate via this >> \\
    let (sender, receiver) = mpsc::channel::<Job>();
    let mut threads = Vec::new();

    // << wrap receiver in Arc<Mutex> so that all threads can receive from it >> \\
    let receiver = Arc::new(Mutex::new(receiver));

    // << spawn global threads >> \\
    for _ in 0..num {
      // << get a local copy of Arc for each thread >> \\
      let rx = Arc::clone(&receiver);

      // << spawn permanent global thread that alive forever by having them locked in an infinite loop >> \\
      let thread = thread::spawn(move || {
        loop {
          // << lock the queue & get the next job >> \\
          /*
          // This was previous attempt
          // it would cause infinite glitch cause Mutex was never freed in thw first place
          // Meaning the main thread is stuck waiting forever
          let lock = rx.lock().unwrap();
          let job = lock.recv();
          */

          let job = {
            let guard = rx.lock().unwrap();
            guard.recv()
          }; // <-- lock is freed automatically here

          match job {
            Ok(task) => task(), // execute task
            Err(_) => break,    // if channel closes, shut down thread loop
          }
        }
      });

      threads.push(thread);
    }
    MiniThreadPool { sender, threads }
  }

  // << dispatches task whenever needed >> \\
  fn act<F: FnOnce() + Send + 'static>(&self, f: F) {
    let job = Box::new(f);
    self.sender.send(job).unwrap();
  }

  // << shuts down all threads safely at program end >> \\
  fn die(self) {
    // << drop channel to send Err to threads >> \\
    drop(self.sender);
    self.threads.into_iter().for_each(|t| {
      t.join().unwrap();
    });
  }
}

fn par_process<T: Send, F: Fn(&mut [T]) + Copy + Send + Sync + 'static>(thread_pool: MiniThreadPool, slice: &mut [T], operation: F) {
  let len = slice.len();
  if len < 1000 {
    operation(slice);
    return;
  }

  let mid = len / 2;
  let (left, right) = slice.split_at_mut(mid);

  thread::scope(|s| {
    thread_pool.act(move || {
      operation(left);
    });
    thread_pool.act(move || {
      operation(right);
    });
  });

  /*
  thread_pool.threads.into_iter().for_each(|t| {
    t.join().unwrap();
  });
  */
}

fn main() {
  println!("Spawning permanent global worker threads...");
  let pool = MiniThreadPool::awake(2);

  // << test some tasks >> \\
  for i in 1..3 {
    pool.act(move || {
      println!("Thread {} is doing work...", i);
    });
  }

  // thread::sleep(Duration::from_millis(100));

  pool.die();
  println!("All threads closed & program exited.");
}