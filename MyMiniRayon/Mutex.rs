/// ।। ॐ नमः शिवाय ।। \\\

/// @date 18th September, 2026
/// @learning
/// Aims to learn Mutex (Mutual Exclusion).
/// Necessary to build mental model & architectural intuition for my mini rayon.

use std::thread;
use std::sync::{Arc, Mutex};

/// With Arc, we learned how multiple threads can safely read the same data.
/// But what happens if multiple threads want to write or modify the exact same data at the exact same time?
/// Imagine two threads both reading a counter variable set to 5.
/// Thread A reads 5, adds 1, and tries to write 6.
/// Thread B reads 5, adds 1, and tries to write 6.
/// They both overwrite each other, and instead of the counter becoming 7, it stays 6.
/// This is a data race, and it causes silent bugs or memory corruption.
/// A Mutex (Mutual Exclusion) solves this by acting like a bathroom stall lock.
/// Only one thread can hold the key to the data at any given time.
/// That thread locks the Mutex, reads or changes the data, and then unlocks it so the next thread can take a turn.
/// Because we usually want to share data across threads and mutate it, Rust developers constantly use them together as Arc<Mutex<T>> (Shared + Mutably Safe).

fn main() {
  // << first we create a counter in Mutex >> \\
  // << then wrap it in Arc >> \\
  // << this ensures threads can own & access it >> \\
  let base_counter = Arc::new(Mutex::new(0));
  println!("{:#?}", base_counter);

  let mut handles: Vec<_> = vec![];

  for _ in 0..10 {
    let cloned_counter = Arc::clone(&base_counter);
    println!("{:#?}", cloned_counter);

    let handle = thread::spawn(move || {
      // << lock the mutex to get access to inner data >> \\
      // << .lock().unwrap() blocks execution until the lock is acquired >> \\
      let mut num = cloned_counter.lock().unwrap();
      println!("{:#?}", num);

      // << mutate data safely >> \\
      *num += 1;

      // << our lock is automatically released here when `num` goes out of scope >> \\
    });

    println!("{:#?}", handle);
    handles.push(handle);
  }
  println!("{:#?}", handles);

  // << wait for all threads to finish >> \\
  for handle in handles {
    handle.join().unwrap();
  }

  println!("Work done by multiple-threads. Final value: {}.", *base_counter.lock().unwrap());
}

// ++ OBSERVATIONS & INSIGHTS ++ \\\
// That is one of the most classic "aha!" moments in Rust.
// We didn't write `let mut base_counter`, yet the number inside successfully incremented from 0 to 10.  
// How is that possible?
// The secret lies in a core Rust design pattern called ++ Interior Mutability ++.

// Normally, in basic Rust:
// `let x = 5`; means x is frozen. You can't change it.
// `let mut x = 5`; means x can be reassigned or mutated.
// But a Mutex (and types like Cell or RefCell) completely bends this rule from the inside out.
// The Variable (base_counter): The base_counter binding itself is immutable.
// We aren't reassigning base_counter to point to a totally different Arc object.
// The Contents (Mutex<i32>): The Mutex provides a protective, locked shell around the integer 0.
// When we call .lock().unwrap(), the Mutex hands us a temporary mutable reference (let mut num) to the data inside the box.

// The Locked Box Analogy
// Think of base_counter like a heavy safe bolted to the floor in a bank.
// The safe itself is bolted down and cannot be moved (it is immutable—you can't reassign base_counter).
// But inside the safe, people can open the door with a key, take out the money, count it, change the number, and lock it back up.
// So, Rust's compiler is happy because the Arc pointer never changes, while the Mutex safely manages the chaotic mutations happening to the data inside it across multiple threads.