// ।। ॐ नमः शिवाय ।। \\
// @date_start 22nd October, 2026

use crate::cpu::CPU;
use std::time::Instant;
use std::hint::black_box;

pub fn run_rough_test() {
  let mut my_cpu = CPU::new(1024);

  // Define a program:
  // 1. MOV x1, #42    -> [0x01, 0x01, 0x00, 0x2A] (opcode 1, reg 1, imm 42)
  // 2. MOV x2, #10    -> [0x01, 0x02, 0x00, 0x0A] (opcode 1, reg 2, imm 10)
  // 3. ADD x3, x1, x2 -> [0x02, 0x03, 0x01, 0x02] (opcode 2, rd 3, rn 1, rm 2)
  let program1 = vec![
    0x01, 0x01, 0x2A, 0x00,
    0x01, 0x02, 0x0A, 0x00,
    0x02, 0x03, 0x01, 0x02,
  ];

  my_cpu.execute_program(&program1);

  // Check results
  println!("x1 = {}", my_cpu.registers.read(1)); // Should be 42
  println!("x2 = {}", my_cpu.registers.read(2)); // Should be 10
  println!("x3 = {}", my_cpu.registers.read(3)); // Should be 52

  my_cpu.reset();
  
  // Define program2: A loop that counts down from 3 to 0 using SUB, CMP, and B.EQ
  let program2 = vec![
    0x01, 0x00, 0x00, 0x00, // 0x00: MOV x0, #0  (comparison stop = 0)
    0x01, 0x01, 0x15, 0x00, // 0x04: MOV x1, #21 (step value)
    0x01, 0x02, 0x07, 0x00, // 0x08: MOV x2, #7  (counter = 7)
    0x01, 0x03, 0x00, 0x00, // 0x0C: MOV x3, #0  (accumulator = 0)
    0x01, 0x04, 0x01, 0x00, // 0x10: MOV x4, #1  (decrementer constant = 1)
    
    // --- Loop Start (Address 16 / 0x10) ---
    0x02, 0x03, 0x03, 0x01, // 0x14: ADD x3, x3, x1 (x3 += 21)
    0x03, 0x02, 0x02, 0x04, // 0x18: SUB x2, x2, x4 (x2 -= 1)
    0x09, 0x02, 0x00, 0x00, // 0x1C: CMP x2, x0     (Compare x2 with x0/zero, sets flags)
    0x06, 0x00, 0x00, 0x28, // 0x20: B.EQ 0x24      (If zero flag is true, jump OUT to 0x28 & exit program)
    0x05, 0x10, 0x00, 0x00, // 0x24: B 0x10         (Else, jump back to loop start at 0x10 & re-iterate)
    
    // --- Loop Exit / End (Address 40 / 0x28) ---
  ];

  my_cpu.execute_program(&program2);
  println!("x3 = {}", my_cpu.registers.read(3)); // Should output 21 × 7 = 147
  my_cpu.reset();

  // program3: demonstrate STR + LDR round trip.
  //   MOV x1, #0xAB        -> x1 = 171
  //   MOV x2, #0           -> base address = 0
  //   STR x1, [x2, #64]    -> memory[64..72] = x1 (little-endian)
  //   LDR x3, [x2, #64]    -> x3 = memory[64..72]
  let program3 = vec![
    0x01, 0x01, 0xAB, 0x00, // 0x00: MOV x1, #0xAB
    0x01, 0x02, 0x00, 0x00, // 0x04: MOV x2, #0
    0x08, 0x01, 0x02, 0x40, // 0x08: STR x1, [x2, #64]
    0x07, 0x03, 0x02, 0x40, // 0x0C: LDR x3, [x2, #64]
  ];

  my_cpu.execute_program(&program3);

  println!("x1 = {}", my_cpu.registers.read(1)); // Should be 171
  println!("x3 = {}", my_cpu.registers.read(3)); // Should be 171 (round-tripped via memory)

  // Show the raw bytes at address 64 to prove the store happened LE.
  let stored = &my_cpu.memory[64..72];
  println!("memory[64..72] = {:?}", stored); // Should be [0xAB, 0, 0, 0, 0, 0, 0, 0]
  my_cpu.reset();

  let program4 = vec![
    0x01, 0x01, 0x11, 0x00, // 0x00: MOV x1, #0x11
    0x01, 0x02, 0x22, 0x00, // 0x04: MOV x2, #0x22
    0x01, 0x03, 0x00, 0x00, // 0x08: MOV x3, #0      (base = 0)
    0x08, 0x01, 0x03, 0x40, // 0x0C: STR x1, [x3, #64]  -> mem[64]  = 0x11
    0x08, 0x02, 0x03, 0x48, // 0x10: STR x2, [x3, #72]  -> mem[72]  = 0x22
    0x07, 0x04, 0x03, 0x40, // 0x14: LDR x4, [x3, #64]  -> x4 = 0x11
    0x07, 0x05, 0x03, 0x48, // 0x18: LDR x5, [x3, #72]  -> x5 = 0x22
  ];

  my_cpu.execute_program(&program4);
  println!("x4: {:#?}.", my_cpu.registers.read(4));  // should be 17
  println!("x5: {:#?}.", my_cpu.registers.read(5));  // should be 34
  my_cpu.reset();
}



pub fn run_addition_test(max: u8) {
  // ---- Rust baseline ---- \\
  let start_rust = Instant::now();
  let mut sum_rust: u64 = black_box(0);
  for i in 1..=max as u64 {
    sum_rust += black_box(i);
  }
  let time_rust = start_rust.elapsed().as_nanos();
  println!("Time for Rust to add up to {}: {}ns.", max, time_rust);
  println!("Sum up to {} by Rust: {}.", max, sum_rust);

  let program = vec![
    0x01, 0x00, 0x00, 0x00,       // 0x00: MOV  x0, #0
    0x01, 0x01, 0x00, 0x00,       // 0x04: MOV  x1, #0
    0x01, 0x02,  max, 0x00,       // 0x08: MOV  x2, #max
    0x0F, 0x01, 0x01, 0x01,       // 0x0C: ADDI x1, x1,   #1
    0x02, 0x00, 0x00, 0x01,       // 0x10: ADD  x0, x0,   x1
    0x09, 0x02, 0x01, 0x00,       // 0x14: CMP  x2, x1
    0x0A, 0x0C, 0x00, 0x00,       // 0x18: B.NE #12
  ];

  let mut my_cpu = CPU::new(1024);
  let start_vm = Instant::now();
  my_cpu.execute_program(&program);
  let time_vm = start_vm.elapsed().as_nanos();
  let gap = (time_vm as f32) / (time_rust as f32);
  println!("Time for my custom VM to add up to {}: {}ns.", max, time_vm);
  println!("Sum up to {} by my custom VM: {}.", max, my_cpu.registers.read(0));
  println!("Gap between my VM & Rust: {}x.", gap);


  // << Sanity check: VM and Rust agree? >> \\
  assert_eq!(
    my_cpu.registers.read(0),
    sum_rust,
    "VM sum {} != Rust sum {}",
    my_cpu.registers.read(0),
    sum_rust
  );
}


pub fn run_square_addition_test(max: u8) {
  // ---- Rust baseline ---- \\
  let start_rust = Instant::now();
  let mut sum_rust: u64 = black_box(0);  // to prevent LLVM doing compile-time cheating
  for i in 1..=max as u64 {
    sum_rust += black_box(i * i);
  }
  let time_rust = start_rust.elapsed().as_nanos();
  println!("Time for Rust to add squares up to {}: {}ns.", max, time_rust);
  println!("Squares sum up to {} by Rust: {}.", max, sum_rust);

  let program = vec![
    0x01, 0x00, 0x00, 0x00,       // 0x00: MOV  x0,   #0
    0x01, 0x01, 0x00, 0x00,       // 0x04: MOV  x1,   #0
    0x01, 0x02,  max, 0x00,       // 0x08: MOV  x2,   #max
    0x0F, 0x01, 0x01, 0x01,       // 0x0C: ADDI x1,   x1,   #1
    0x04, 0x03, 0x01, 0x01,       // 0x10: MUL  x3,   x1,   x1
    0x02, 0x00, 0x00, 0x03,       // 0x14: ADD  x0,   x0,   x3
    0x09, 0x02, 0x01, 0x00,       // 0x18: CMP  x2,   x1
    0x0A, 0x0C, 0x00, 0x00,       // 0x1C: B.NE #12
  ];

  let mut my_cpu = CPU::new(1024);
  let start_vm = Instant::now();
  my_cpu.execute_program(&program);
  let time_vm = start_vm.elapsed().as_nanos();
  let gap = (time_vm as f32) / (time_rust as f32);
  println!("Time for my custom VM to add squares up to {}: {}ns.", max, time_vm);
  println!("Squares sum up to {} by my custom VM: {}.", max, my_cpu.registers.read(0));
  println!("Gap between my VM & Rust: {}x.", gap);

  // << Sanity check: VM and Rust agree? >> \\
  assert_eq!(
    my_cpu.registers.read(0),
    sum_rust,
    "VM sum {} != Rust sum {}",
    my_cpu.registers.read(0),
    sum_rust
  );
}

pub fn run_fibonacci_test(n: u8) {
  // ---- Rust baseline ---- \\
  let start_rust = Instant::now();
  let mut a: u64 = 0;
  let mut b: u64 = 1;
  let mut fib: u64 = 0;

  if n == 0 {
    fib = 0;
  } else if n == 1 {
    fib = 1;
  } else {
    for _ in 2..=n {
      fib = black_box(a + b);
      a = b;
      b = fib;
    }
  }
  let time_rust = start_rust.elapsed().as_nanos();
  println!("Time for Rust to compute Fib({}): {}ns.", n, time_rust);
  println!("Fib({}) by Rust: {}.", n, fib);

  // ---- VM Bytecode Program ---- \\
  // Registers:
  // x0 = a (n-2)
  // x1 = b (n-1)
  // x2 = fib result accumulator
  // x3 = target n
  // x4 = loop counter
  // x5 = constant 1
  let program = vec![
    0x01, 0x00, 0x00, 0x00,       // 0x00: MOV  x0, #0      (a = 0)
    0x01, 0x01, 0x01, 0x00,       // 0x04: MOV  x1, #1      (b = 1)
    0x01, 0x03,    n, 0x00,       // 0x08: MOV  x3, #n      (target n)
    0x01, 0x04, 0x01, 0x00,       // 0x0C: MOV  x4, #1      (counter = 1)
    0x01, 0x05, 0x01, 0x00,       // 0x10: MOV  x5, #1      (constant 1)

    // --- Loop Start (Address 20 / 0x14) ---
    0x09, 0x03, 0x04, 0x00,       // 0x14: CMP  x3, x4      (Compare target n with counter)
    0x06, 0x00, 0x00, 0x30,       // 0x18: B.EQ 0x30        (If equal, exit loop)
    0x02, 0x02, 0x00, 0x01,       // 0x1C: ADD  x2, x0, x1  (fib = a + b)
    0x26, 0x00, 0x01, 0x00,       // 0x20: MOVR x0, x1      (a = b)
    0x26, 0x01, 0x02, 0x00,       // 0x24: MOVR x1, x2      (b = fib)
    0x02, 0x04, 0x04, 0x05,       // 0x28: ADD  x4, x4, x5  (counter += 1)
    0x05, 0x14, 0x00, 0x00,       // 0x2C: B 0x14           (Loop back)
    // --- Loop Exit (Address 48 / 0x30) ---
  ];

  let mut my_cpu = CPU::new(1024);
  let start_vm = Instant::now();
  my_cpu.execute_program(&program);
  let time_vm = start_vm.elapsed().as_nanos();
  let gap = (time_vm as f32) / (time_rust.max(1) as f32);

  println!("Time for my custom VM to compute Fib({}): {}ns.", n, time_vm);
  println!("Fib({}) by my custom VM: {}.", n, my_cpu.registers.read(1));
  println!("Gap between my VM & Rust: {}x.", gap);

  assert_eq!(
    my_cpu.registers.read(1),
    fib,
    "VM Fib {} != Rust Fib {}",
    my_cpu.registers.read(1),
    fib
  );
}

pub fn run_pell_test(n: u8) {
  // ---- Rust baseline ---- \\
  let start_rust = Instant::now();
  let mut p0: u64 = 0;
  let mut p1: u64 = 1;
  let mut p_curr: u64 = 0;
  
  if n == 0 {
    p_curr = 0;
  } else if n == 1 {
    p_curr = 1;
  } else {
    for _ in 2..=n {
      p_curr = black_box(2 * p1 + p0);
      p0 = p1;
      p1 = p_curr;
    }
  }
  let time_rust = start_rust.elapsed().as_nanos();
  println!("Time for Rust to compute Pell({}) : {}ns.", n, time_rust);
  println!("Pell({}) by Rust: {}.", n, p_curr);

  let program = vec![
    0x01, 0x03, 0x00, 0x00,       // 0x00: MOV  x3, #0      (p0 = 0)
    0x01, 0x04, 0x01, 0x00,       // 0x04: MOV  x4, #1      (p1 = 1)
    0x01, 0x05, 0x02, 0x00,       // 0x08: MOV  x5, #2      (multiplier 2)
    0x01, 0x06, 0x01, 0x00,       // 0x0C: MOV  x6, #1      (constant 1)
    0x01, 0x02, 0x01, 0x00,       // 0x10: MOV  x2, #1      (counter = 1)
    0x01, 0x01,    n, 0x00,       // 0x14: MOV  x1, #n      (target n)
    
    // --- Loop Start (Address 24 / 0x18) ---
    0x09, 0x01, 0x02, 0x00,       // 0x18: CMP  x1, x2      (Compare n with counter)
    0x06, 0x00, 0x00, 0x38,       // 0x1C: B.EQ 0x38        (If equal, jump out to exit)
    0x04, 0x00, 0x04, 0x05,       // 0x20: MUL  x0, x4, x5  (x0 = p1 * 2)  <-- FIXED BYTES HERE
    0x02, 0x00, 0x00, 0x03,       // 0x24: ADD  x0, x0, x3  (x0 = 2*p1 + p0 -> new p_curr)
    0x26, 0x03, 0x04, 0x00,       // 0x28: MOVR x3, x4      (p0 = p1)
    0x26, 0x04, 0x00, 0x00,       // 0x2C: MOVR x4, x0      (p1 = p_curr)
    0x02, 0x02, 0x02, 0x06,       // 0x30: ADD  x2, x2, x6  (counter += 1)
    0x05, 0x18, 0x00, 0x00,       // 0x34: B 0x18           (Loop back)
    // --- Loop Exit (Address 56 / 0x38) ---
  ];

  let mut my_cpu = CPU::new(1024);
  let start_vm = Instant::now();
  my_cpu.execute_program(&program);
  let time_vm = start_vm.elapsed().as_nanos();
  let gap = (time_vm as f32) / (time_rust.max(1) as f32);
  
  println!("Time for my custom VM to compute Pell({}): {}ns.", n, time_vm);
  println!("Pell({}) by my custom VM: {}.", n, my_cpu.registers.read(4));
  println!("Gap between my VM & Rust: {}x.", gap);

  assert_eq!(
    my_cpu.registers.read(4),
    p_curr,
    "VM Pell {} != Rust Pell {}",
    my_cpu.registers.read(4),
    p_curr
  );
}

pub fn run_simple_tests() {
  run_rough_test();

  run_addition_test(100);
  run_addition_test(150);
  run_addition_test(200);
  
  run_square_addition_test(100);
  run_square_addition_test(150);
  run_square_addition_test(200);

  run_fibonacci_test(10); 
  run_fibonacci_test(5);
  run_fibonacci_test(71);

  run_pell_test(7);
  run_pell_test(17);
  run_pell_test(21);
}

pub fn run_heavy_addition_test(max: u8, times: u32) {
  // -- Rust baseline -- \\
  let mut total_rust_time: u128 = 0;
  let mut start_rust = Instant::now();
  let mut time_rust = 0;
  let mut total_rust_sum = black_box(0);
  for _ in 0..times {
    total_rust_sum = 0;
    start_rust = Instant::now();
    for i in 1..=max {
      total_rust_sum += black_box(i);
    }
    time_rust = start_rust.elapsed().as_nanos();
    total_rust_time += time_rust;
  }
  let average_rust_time = (total_rust_time) / (times as u128);
  println!("Average time for Rust to add upto {} across {} runs: {}ns.", max, times, average_rust_time);

  // -- My VM -- \\
  let program = vec![
    0x01, 0x00, 0x00, 0x00,    // 0x00, mov x0 #0
    0x01, 0x01, 0x00, 0x00,    // 0x04, mov x1, #0
    0x0F, 0x01, 0x01, 0x01,    // 0x08, addi x1, x1, #1
    0x02, 0x00, 0x00, 0x01,    // 0x0C, add x0, x0, x1
    0x27, 0x01,  max, 0x00,    // 0x10, cmpi x1, #max
    0x0A, 0x08, 0x00, 0x00,    // 0x14, bne, #8
  ];
  let mut my_cpu = CPU::new(1024);

  let mut total_vm_time: u128 = 0;
  let mut start_vm = Instant::now();
  let mut time_vm = 0;
  for _ in 0..times {
    my_cpu.reset();
    start_vm = Instant::now();
    
    my_cpu.execute_program(&program);
    
    time_vm = start_vm.elapsed().as_nanos();
    total_vm_time += time_vm;
  }
  let average_vm_time = (total_vm_time) / (times as u128);
  let gap = (average_vm_time as f64) / (average_rust_time as f64);
  println!("Average time for my VM to add upto {} across {} runs: {}ns.", max, times, average_vm_time);
  println!("Gap between my VM & Rust: {}x.", gap);

  /*
    Average time for Rust to add upto 100 across 1000 runs: 259ns.
    Average time for my VM to add upto 100 across 1000 runs: 1939ns.
    Gap between my VM & Rust: 7.486486486486487x.
  */
}

pub fn run_heavy_tests() {
  run_heavy_addition_test(100, 1000);
}

/*
Time for Rust to add up to 100: 313ns.
Sum up to 100 by Rust: 5050.
Time for my custom VM to add up to 100: 2136ns.
Sum up to 100 by my custom VM: 5050.
Gap between my VM & Rust: 6.824281x.
Time for Rust to add up to 150: 417ns.
Sum up to 150 by Rust: 11325.
Time for my custom VM to add up to 150: 2864ns.
Sum up to 150 by my custom VM: 11325.
Gap between my VM & Rust: 6.8681054x.
Time for Rust to add up to 200: 469ns.
Sum up to 200 by Rust: 20100.
Time for my custom VM to add up to 200: 3750ns.
Sum up to 200 by my custom VM: 20100.
Gap between my VM & Rust: 7.9957356x.
Time for Rust to add squares up to 100: 261ns.
Squares sum up to 100 by Rust: 338350.
Time for my custom VM to add squares up to 100: 2604ns.
Squares sum up to 100 by my custom VM: 338350.
Gap between my VM & Rust: 9.977012x.
Time for Rust to add squares up to 150: 365ns.
Squares sum up to 150 by Rust: 1136275.
Time for my custom VM to add squares up to 150: 3489ns.
Squares sum up to 150 by my custom VM: 1136275.
Gap between my VM & Rust: 9.558904x.
Time for Rust to add squares up to 200: 521ns.
Squares sum up to 200 by Rust: 2686700.
Time for my custom VM to add squares up to 200: 4583ns.
Squares sum up to 200 by my custom VM: 2686700.
Gap between my VM & Rust: 8.796545x.
Time for Rust to compute Fib(10): 104ns.
Fib(10) by Rust: 55.
Time for my custom VM to compute Fib(10): 885ns.
Fib(10) by my custom VM: 55.
Gap between my VM & Rust: 8.509615x.
Time for Rust to compute Fib(5): 52ns.
Fib(5) by Rust: 5.
Time for my custom VM to compute Fib(5): 260ns.
Fib(5) by my custom VM: 5.
Gap between my VM & Rust: 5x.
Time for Rust to compute Fib(71): 208ns.
Fib(71) by Rust: 308061521170129.
Time for my custom VM to compute Fib(71): 2292ns.
Fib(71) by my custom VM: 308061521170129.
Gap between my VM & Rust: 11.019231x.
Time for Rust to compute Pell(7) : 52ns.
Pell(7) by Rust: 169.
Time for my custom VM to compute Pell(7): 521ns.
Pell(7) by my custom VM: 169.
Gap between my VM & Rust: 10.019231x.
Time for Rust to compute Pell(17) : 104ns.
Pell(17) by Rust: 1136689.
Time for my custom VM to compute Pell(17): 781ns.
Pell(17) by my custom VM: 1136689.
Gap between my VM & Rust: 7.5096154x.
Time for Rust to compute Pell(21) : 52ns.
Pell(21) by Rust: 38613965.
Time for my custom VM to compute Pell(21): 781ns.
Pell(21) by my custom VM: 38613965.
Gap between my VM & Rust: 15.019231x.
*/