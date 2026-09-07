// ।। ॐ नमः शिवाय ।। \\
// ॥ ॐ वामदेवाय नमः ॥ \\
// ++ Enlightened Saint Rust ++ \\

fn main() {
  println!("4+5*6 = {}.", {calc1("4+5*6").unwrap()});
  println!("4+5/9= {}.", {calc1("4+5/9").unwrap()});
  println!("3+4*6-6+81 = {}.", {calc1("3+4*6-6+81").unwrap()});
  println!("6/6/6/6/6 = {}.", {calc1("6/6/6/6/6").unwrap()});
}

fn calc1(input: &str) -> Result<f32, String> {
  // Result<f32, String>: Instead of returning strings like 'Invalid input.' alongside numbers, Rust uses the Result type to return either a successful Ok(value) or an Err(error_msg). \\
  
  // << setup >> \\
  let mut input_str = input.trim();
  // << error checks >> \\
  if input_str.is_empty() {
    return Err("Empty input!".to_string());
  }
  let operators: [char; 4] = ['+', '-', '*', '/'];
  
  // << holder arrays >> \\
  // vec![] is a macro for creating vectors; for types, its just vec<T>;
  let mut nums: Vec<f32> = Vec::new();
  let mut ops: Vec<char> = Vec::new();
  
  // << initial symbol check >> \\
  if input_str.starts_with('-') || input_str.starts_with('+') {
    nums.push(0.0); // push 0 to numbers:
    let first_op = input_str.chars().next().unwrap();
    ops.push(first_op);
    input_str = &input_str[1..]; // slice past first char;
  }
  // << lexer + parser >> \\
  let mut i = 0;
  while i < input_str.len() {
    let current = input_str.chars().nth(i).unwrap(); // inbuilt method; graps char at place i;
    
    // << loop through operators >> \\
    if operators.contains(&current) {
      // << get preceding number >> \\
      let num_str = &input_str[..i];
      let target_num = num_str.parse::<f32>().expect("Unexpected input.");
      nums.push(target_num);
      ops.push(current);
      // << slice & reset >> \\
      input_str = &input_str[i+1..];
      i = 0;
    } else {
      i += 1;
    }
    
    // << exit if no more operators >> \\
    // Rust's String::contains refuses direct array passes like (operators); so we pass complete inline-array;
    if !input_str.contains(['+', '-', '*', '/']) {
      let last_num = input_str.parse::<f32>().expect("Unexpected input.");
      nums.push(last_num);
      break; // clean exit;
    }
  }
  
 // << calculate >> \\
  let mut output: f32 = nums[0];
  let mut k = 0;
  while k < ops.len() {
    let op: char = ops[k];
    let num: f32 = nums[k+1];
    match op {
      '+' => output += num,
      '-' => output -= num,
      '*' => output *= num,
      '/' => output /= num,
      _ => unreachable!(), // wildcard for exhaustive match
    }
    k += 1;
  }
  Ok(output) // wrap in Ok for Return type;
}