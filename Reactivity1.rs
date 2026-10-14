// ।। ॐ नमः शिवाय ।। \\
// @date 4th October, 2026

// This is a basic attempt to try JS-like Reactivity in Rust

use std::cell::RefCell;
use std::rc::Rc;
use std::fmt; // for implementing manual Debug

#[derive(Debug)]
struct NumberPair {
  base: f32,
  bind: f32,
}

impl NumberPair {
  fn new(base_value: f32) -> Self {
    NumberPair {
      base: base_value,
      bind: base_value.sqrt(),
    }
  }

  fn update(&mut self, new_base: f32) {
    self.base = new_base;
    self.bind = new_base.sqrt();
  }
}

// Use generics in structs
// #[derive(Debug)]
struct FunctionNumberPair<F> {
  base: f32,
  bind: f32,
  closure: F,
}

impl<F: Fn(f32) -> f32> FunctionNumberPair<F> {
  fn new(base_value: f32, input_closure: F) -> Self {
    FunctionNumberPair {
      base: base_value,
      bind: input_closure(base_value),
      closure: input_closure,
    }
  }
  
  fn update(&mut self, new_base: f32) {
    self.base = new_base;
    self.bind = (self.closure)(new_base);
  }
}

impl<F: Fn(f32) -> f32> fmt::Debug for FunctionNumberPair<F> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("FunctionNumberPair")
      .field("base", &self.base)
      .field("bind", &self.bind)
      .finish()
  }
}

// Implementing `Debug` to a struct requires all its field to implement `Debug`
// But f32 doesnt implement `Debug` & Rust doesnt allow so
// However, we dont need F closure anyway
// So we manually implemented `Debug` for the necessary fields only using `std::fmt`


fn main() {
  let mut my_pair: NumberPair = NumberPair::new(71.92);

  println!("Before: {:#?}", my_pair);
  my_pair.update(81.73);
  println!("After: {:#?}", my_pair);

  
  let mut my_fn_pair = FunctionNumberPair::new(1883.7272, f32::log2);
  
  println!("Before: {:#?}", my_fn_pair);
  my_fn_pair.update(7777.777);
  println!("After: {:#?}", my_fn_pair);
}