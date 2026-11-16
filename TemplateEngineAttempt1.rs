// ॐ श्री गणेशाय नम \\
// ।। ॐ नमः शिवाय ।। \\
// @date 16th November, 2026

// @learning @aim
// @type_system @templates
// @languages @compilers

// This project attempts to build the mental model & intuition for type-based metaprogramming
// and how C++/D templates work under the hood

#[derive(Debug, Clone, PartialEq, Eq)]
enum Type {
  // !! base type, like 0 !! \\
  Base,
  // !! successor case !! \\
  Succ(Box<Type>),
  // !! template instantiation !! \\
  Add(Box<Type>, Box<Type>),
}

// !! MASTER FUNCTION !! \\
fn evaluate(t: Type) -> Type {
  match t {
    Type::Base => Type::Base,
    Type::Succ(inner) => Type::Succ(Box::new(evaluate(*inner))),
      
    // ++ template logic: Add<A, B> ++ \\
    Type::Add(a, b) => {
      let evaluated_a = evaluate(*a);
      let evaluated_b = evaluate(*b);

      match evaluated_a {
        // !! base case: Add<Base, B> = B !! \\
        Type::Base => evaluated_b,

        // !! recursive case: Add<Succ<Prev>, B> = Add<Prev, Succ<B>> !! \\
        Type::Succ(prev) => {
          let next_b = Type::Succ(Box::new(evaluated_b));
          evaluate(Type::Add(Box::new(*prev), Box::new(next_b)))
        },

        // !! fallback case !! \\
        other => Type::Add(Box::new(other), Box::new(evaluated_b)),
      }
    },
  }
}

// !! CONVERSION HELPERS !! \\
fn get_number_from_type(t: Type) -> u64 {
  let mut output: u64 = 0;
  match &t {
    Type::Base => output,
    Type::Succ(inner) => {
      match **inner {
        Type::Base => {
          output += 1;
          output
        },
        _ => {
          output += get_number_from_type(*inner.clone()) + 1;
          output
        },
      }
    },
    Type::Add(a, b) => {
      let computed = evaluate(t.clone());
      output = get_number_from_type(computed.clone());
      output
    },
  }
}

fn get_type_from_number(num: u64) -> Type {
  let mut output: Type = Type::Base;
  match num {
    0 => output,
    _ => {
      output = Type::Succ(Box::new(get_type_from_number(num - 1)));
      output
    }
  }
}

// !! SAMPLE USAGE !! \\
fn first_sample() {
  let two = Type::Succ(Box::new(Type::Succ(Box::new(Type::Base))));
  let one = Type::Succ(Box::new(Type::Base));

  println!("++ My Template Engine ++");
  println!("2 = {:?}", two);
  println!("1 = {:?}", one);

  let expression = Type::Add(Box::new(two), Box::new(one));

  println!("2 + 1 = {:?}\n", expression);

  let computed = evaluate(expression);

  println!("Computed output (should be 3): {:?}\n", computed);
}

fn second_sample() {
  let type_num = Type::Succ(Box::new(Type::Succ(Box::new(Type::Succ(Box::new(Type::Base))))));
  let num = get_number_from_type(type_num.clone());
  
  println!("{:?} = {}\n", type_num, num);
  println!("7 = {:?}", get_type_from_number(7));
}

fn main() {
  first_sample();
  second_sample();
}

/*
 * ============================================================================
 *  THE BIG PICTURE: PEANO ARITHMETIC & TYPE-LEVEL COMPUTATION
 * ============================================================================
 *
 *  1. WHAT IS PEANO ARITHMETIC?
 *     In 1889, Giuseppe Peano defined all natural numbers using just two rules:
 *       - A starting symbol: `Base` (represents 0)
 *       - A wrapper function: `Succ(N)` (represents "Successor of N", or N + 1)
 *
 *     Instead of storing numbers as raw binary bits, we represent them as nested
 *     layers of structure (like Russian nesting dolls):
 *       0  =  Base
 *       1  =  Succ(Base)
 *       2  =  Succ(Succ(Base))
 *       3  =  Succ(Succ(Succ(Base)))
 *
 *  2. WHY DO COMPILERS & TYPE SYSTEMS USE THIS?
 *     Type systems (like C++ templates or Rust generics) do not have built-in CPU
 *     arithmetic instructions (`+`, `-`, `*`) at compile time. They only know how
 *     to match patterns and wrap/unwrap types.
 *
 *     Peano numbers turn math into pure structural pattern matching:
 *       - Addition becomes "unwrapping layers from A and wrapping them onto B."
 *       - When A hits `Base` (0), the recursion stops and B holds the answer.
 *
 *  3. HOW OUR ENGINE MIMICS A C++ TEMPLATE COMPILER
 *     - Defining `Type` variants = Writing template blueprints (Phase 1).
 *     - Calling `evaluate()`     = Template instantiation & monomorphization (Phase 2).
 *     - `match` arms             = Template specialization (Base Case vs Recursive Case).
 *
 * ============================================================================
 */
