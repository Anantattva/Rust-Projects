// ।। ॐ नमः शिवाय ।। \\

// @date 19th September, 2026
// @learning
// Why static lifetimes matter??

use std::thread;

fn main() {
  let local_data = 81;

  let handle = thread::spawn(|| {
    println!("Reading data from main thread: {}.", local_data);
  });

  handle.join().unwrap();
}

// ++ OBSERVATIONS & INSIGHTS ++ \\
// Compiler throws this error:
// error[E0373]: closure may outlive the current function, but it borrows `local_data`, which is owned by the current function

// What this teaches us:
// Rust's compiler stops you instantly. It says: "Hey, this background thread might live longer than main's local scope.
// If main finishes and destroys local_number, your thread will crash trying to read ghost memory."

// Adding the 'static constraint to our Mini-Rayon job type explicitly enforces this rule:
// any closure sent to our thread pool must own its data and cannot borrow short-lived local variables.

// ++ Why do we need handle.join().unwrap() anyway? ++ 

// When we spawn a background thread, your main thread treats it like firing a rocket:
// it launches it into the sky and keeps running at full speed.
// If our main function reaches the end of its code and exits, the entire program shuts down instantly, killing any background threads that are still mid-sentence.

// handle.join(): Grabs the thread handle and tells the main thread: "Stop right here and freeze.
// Do not let the program finish until this background thread calls home and reports that it is completely done."
// .unwrap(): If the background thread crashes or panics halfway through its job, join() returns an Err. .unwrap() catches that error and crashes the main thread too,
// so we don't pretend everything went fine when a thread exploded.