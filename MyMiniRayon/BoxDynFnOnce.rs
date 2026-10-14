// ।। ॐ नमः शिवाय ।। \\

// @date 19th September, 2026
// @learning
// Closures, Box, dyn trait & FnOnce() trait
// Build foundational for Rayon.

// ++ Heterogenous Tasks List >> \\
fn main() {
  let mut tasks: Vec<Box<dyn FnOnce()>> = vec![];

  // << task 1: prints a string >> \\
  tasks.push(Box::new(|| {
    println!("Task 1 working...");
  }));

  // << task 2: add numbers >> \\
  tasks.push(Box::new(|| {
    let sum = 82 + 71;
    println!("Sum {}.", sum);
  }));

  // << executes all tasks in a loop >> \\
  for task in tasks {
    // << because its a closure with FnOnce(), it consumes itself upon running >> \\
    task();
  }
}

// ++ What this teaches us: ++
// Without Box<dyn FnOnce()>, Rust would throw a type error because Task A and Task B are technically different types.
// Box wraps them up to a uniform size, and dyn FnOnce() tells Rust to use dynamic dispatch so we can loop through and run them interchangeably!

// ++ How is a type different from a struct, and what is a trait? ++

// These three words get thrown around constantly, but they occupy completely different layers of Rust:
​Type: A broad category defining what kind of data something is and how many bytes it takes in memory (e.g., i32, String, bool, or your own custom structs).
​Struct: A specific tool used to create a custom type by grouping multiple pieces of data together under one roof (like a User struct with a name and age).
​Trait: A behavior contract or checklist. A trait holds no data fields. Instead, it says: "If any struct implements this trait, it promises to have these specific methods or capabilities." (Like the Send trait, which is a badge telling Rust: "I am safe to move across threads").
​The Analogy:
​A Type is like the concept of a "Vehicle."
​A Struct is the blueprint for a specific vehicle ("Tesla Model 3" with wheels, battery, doors).
​A Trait is a certification card like "Can Float on Water." Any vehicle struct (boat, car, duck) can implement the "Can Float" trait if it has the required behavior.