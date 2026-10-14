/// ।। ॐ नमः शिवाय ।। \\\

/// @date 18th September, 2026
/// @learning
/// Aims to learn Arc (Atomic Reference Counter).
/// Necessary to build mental model & architectural intuition for my mini rayon.

use std::thread;
use std::sync::Arc;

/// Normally in Rust, data can only have one owner. If you want to give a piece of data to Thread A, you have to move it there.
/// But what if you have three different threads that all need to read the exact same heavy configuration or data vector at the same time?
/// Copying the data for every single thread wastes memory and CPU time.
/// Arc (Atomically Reference Counted) solves this by letting multiple threads share ownership of the exact same block of memory safely.
/// The Library Analogy:
/// Think of an Arc like a library book. The book itself sits on a shelf in memory. When Thread A wants to read it, it checks out a copy
/// of the "library card" (increments a counter). When Thread B checks it out, the counter goes up again. When all threads finish reading
/// and drop their cards, the counter hits zero, and the library automatically recycles the book.
/// Atomic:
/// The "Atomic" part just means the counter is updated safely across multiple CPU cores without race conditions.
/// (Note: Regular Rc is for single threads; Arc is thread-safe).

fn main() {
  // << wrap data in Arc so it can be shared >> \\
  let base_data = Arc::new(vec![1, 2, 3, 4, 5, 6]);
  println!("{:#?}", base_data);

  let mut handles = Vec::new();

  for i in 0..3 {
    // << notice we clone the arc pointer, not the data >> \\
    // << this cheaply increments the shared reference counter >> \\
    let cloned_data = Arc::clone(&base_data);
    println!("{:#?}", cloned_data);

    let handle = thread::spawn(move || {
      // << each thread can safely read the data >> \\
      println!("Thread {} reading data {:#?}.", i, cloned_data);
    });

    handles.push(handle);
  }
  // << wait for al threads to finish >> \\
  for handle in handles {
    handle.join().unwrap();
  }
  println!("Done!! Arc worked successfully!!!");
}