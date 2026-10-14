// ।। ॐ नमः शिवाय ।। \\

// @date 19th September, 2026
// @learning
// Closures.

fn main() {
  let tax_rate: f32 = 0.08;

  let calculate_price = |price: f32| price * (1.0 + tax_rate);

  println!("Total price of ₹17000 phone: {}.", calculate_price(17000.0));
}

// ++ What is a closure, and how is it different from a normal function? ++

// Normal Function (fn):
// A regular function is a standalone block of code with a fixed name.
// It lives globally, and it knows nothing about its surroundings unless you explicitly pass variables into it as parameters.

// Closure (||):
// A closure is an anonymous (nameless) inline function.
// Its superpower is that it can "close over" and capture variables from the exact scope it was written in, without you having to pass them as arguments.

// The Analogy:
// A normal function is like a public restaurant chef—they only cook what you hand them on the order slip.
// A closure is like a chef who sneaks into your home kitchen and steals ingredients directly out of your fridge because they are standing right next to it.