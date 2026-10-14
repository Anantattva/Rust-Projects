/// ।। ॐ नमः शिवाय ।। \\\

/// @date 18th September, 2026

/// This project aims to learn mpsc.
/// Multiple-Producers, Single-Consumer
/// Necessary to build mental model & architectural intuition for my mini rayon.

use std::thread;
use std::sync::mpsc;
use std::time::Duration;

/// Normally, when you call a function, it does some work and hands you back a return value. But when you spawn a thread, it runs in its own completely isolated universe.
/// You can't just do let result = thread::spawn(|| { 5 + 5 }); and expect main() to casually catch the return value, because main keeps running immediately while the thread is still doing its job.
/// An mpsc channel gives you a safe pipe: one end (tx for transmitter) stays with the background thread to drop a message in, and the other end (rx for receiver) stays with main to listen for it.

fn main() {
  /// << create a channel >> \\\
  println!("Active...");
  let (tx, rx) = mpsc::channel();
  println!("{:#?}", tx);
  println!("{:#?}", rx);

  /// << spawn a background thread >> \\\
  thread::spawn(move || {
    println!("Background thread active... doing work...");
    thread::sleep(Duration::from_secs(1));

    let result = (((19*19*19) as f32).sqrt()).sqrt();
    tx.send(result).unwrap();
    println!("Calculation done!!!");
  });

  /// << receive back >> \\\
  println!("Main thread waiting...");
  let received = rx.recv().unwrap();
  println!("Calculation result: {}.", received);
}