// ।। ॐ नमः शिवाय ।। \\

// @date 19th September, 2026
// @learning @building @experimenting
// This is a bare minimalist attempt to clone ParallelIterator of Rayon.

use std::thread;
use std::time::Instant;

fn par_process<T, F>(slice: &mut [T], operation: F)
where
  T: Send,
  F: Fn(&mut [T]) + Send + Sync
{
  // << base case >> \\
  // << if its too small, run it normally >> \\
  let len = slice.len();
  if len <= 1 {
    operation(slice);
    return;
  }

  // << general case >> \\
  // << get the exact midpoint & slice half >> \\
  let mid = len / 2;
  let (left, right) = slice.split_at_mut(mid);

  // << spawn a new thread for right half & run left half on main thread >> \\
  thread::scope(|s| {
    operation(left);
    s.spawn(|| {
      operation(right);
    });
  });
}

fn test_addition(max: u32) {
  let mut data1: Vec<u32> = (1..=max).collect();
  let mut data2 = data1.clone();
  let start1 = Instant::now();
  for n in data1.iter_mut() {
    *n += 1;
  }
  let time1 = start1.elapsed().as_micros();
  println!("Time to add 1 to {} numbers by single thread: {}mis.", max, time1);

  let start2 = Instant::now();
  par_process(&mut data2, |chunk| {
    for n in chunk.iter_mut() {
      *n += 1;
    }
  });
  let time2 = start2.elapsed().as_micros();
  println!("Time to add 1 to {} numbers by 2 threads: {}mis.", max, time2);
}

fn test_sqrt(max: u32) {
  let mut data1: Vec<u32> = (1..=max).collect();
  let mut data2 = data1.clone();

  let start1 = Instant::now();
  for n in data1.iter_mut() {
    *n = n.isqrt();
  }
  let time1 = start1.elapsed().as_micros();
  println!("Time to sqrt {} numbers by 1 thread: {}mis.", max, time1);

  let start2 = Instant::now();
  par_process(&mut data2, |chunk| {
    for n in chunk.iter_mut() {
      *n = n.isqrt();
    }
  });
  let time2 = start2.elapsed().as_micros();
  println!("Time to sqrt {} numbers by 2 threads: {}mis.", max, time2);
}

fn test_complex_math(max: u32) {
  let mut data1: Vec<u32> = (1..=max).collect();
  let mut data2 = data1.clone();

  let start1 = Instant::now();
  for n in data1.iter_mut() {
    *n = ((*n as f32)/1000.0).sqrt().log2() as u32;
  }
  let time1 = start1.elapsed().as_micros();
  println!("Time to do math for {} numbers by 1 thread: {}mis.", max, time1);

  let start2 = Instant::now();
  par_process(&mut data2, |chunk| {
    for n in chunk.iter_mut() {
      *n = ((*n as f32)/1000.0).sqrt().log2() as u32;
    }
  });
  let time2 = start2.elapsed().as_micros();
  println!("Time to do math for {} numbers by 2 threads: {}mis.", max, time2);
}

fn main() {
  test_addition(1000000);
  test_addition(100000000);
  
  test_sqrt(100000);
  test_sqrt(10000000);
  
  test_complex_math(100000);
  test_complex_math(10000000);
}

// ++ DEVELOPER'S LEARNING NOTES ++
// @learing Rust Generics & Trait Bounds

// ++ 1. <T, F> (The Type Variables) ++
// Instead of hardcoding a specific type like i32 or a specific closure, T and F are generic placeholders.
// T: Represents the type of data inside our vector (e.g., it could be i32, String, or a custom struct).
// F: Represents the type of the closure (the operation) you pass into the function.
// By using <T, F>, this single function can work with any data type and any custom operation you give it!

// 2. slice: &mut [T], operation: F (The Arguments)
// slice: &mut [T]: A mutable slice of elements of type T.
// operation: F: The function or closure you want to run on that slice.
// Since F is a generic type, Rust looks at the closure you pass in and automatically figures out what F is at compile time.

// 3. The where Clause (The Security Guardrails)
// The where clause tells Rust: "I am using generics T and F, but they must follow these strict rules to be allowed in this multi-threaded function:"
// T: Send: Guarantees that the data items inside the slice can be safely sent across thread boundaries.
// F: Fn(&mut [T]): States that F must be a closure that accepts a mutable slice of T as an argument (&mut [T]). This matches the chunk of data we want it to process.
// + Send + Sync: Guarantees that the closure itself can be safely moved across threads (Send) and shared or referenced across threads (Sync).

// ++ In Plain English: ++
// "Hey Rust, par_process is a generic function that takes any type of data T and any operation closure F.
//  As long as the data is safe to send across threads (Send), and the operation takes a mutable slice,
//  can be sent across threads, and is thread-safe (Send + Sync), let it compile!"

// ++ RESULTS ++
// Time to add 1 to 1000000 numbers by single thread: 660mis.
// Time to add 1 to 1000000 numbers by 2 threads: 1381mis.
// Time to add 1 to 100000000 numbers by single thread: 70382mis.
// Time to add 1 to 100000000 numbers by 2 threads: 81489mis.

// Time to sqrt 100000 numbers: 618mis.
// Time to sqrt 100000 numbers: 881mis.
// Time to sqrt 10000000 numbers: 90582mis.
// Time to sqrt 10000000 numbers: 141252mis.

// Time to do math for 100000 numbers: 680mis.
// Time to do math for 100000 numbers: 950mis.
// Time to do math for 10000000 numbers: 67020mis.
// Time to do math for 10000000 numbers: 97531mis.