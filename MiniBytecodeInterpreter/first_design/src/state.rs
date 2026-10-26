// ।। ॐ नमः शिवाय ।। \\
// @date_start 19th October, 2026

///// ++++++++++++++++++++ \\\\\
// ++== Registers struct ==++ \\
///// ++++++++++++++++++++ \\\\\

pub struct Registers {
  pub x: [u64; 31],
}

impl Registers {
  pub fn new() -> Self {
    Self {
      x: [0; 31],
    }
  }
}

impl Registers {
  pub fn read(&self, index: usize) -> u64 {
    if index == 31 {
      return 0; // free XZR, always return p
    }
    // << my first time writing unsafe Rust ☠️☠️😎😎 >> \\
    // << @optimization: safety is gauranteed by compiler >> \\
    unsafe {
      *self.x.get_unchecked(index)
    }
  }

  pub fn write(&mut self, index: usize, new_value: u64) {
    if index == 31 {
      return; // free XZR, writes are discarded
    }
    // << @optimization: safety is gauranteed by compiler >> \\
    unsafe {
      *self.x.get_unchecked_mut(index) = new_value;
    }
  }
}


///// ++++++++++++++++ \\\\\
// ++== Flags struct ==++ \\
///// ++++++++++++++++ \\\\\

pub struct Flags {
  pub zero_flag: bool,     // 'Z': result was zero
  pub negative_flag: bool, // 'N': result's top bit was 1 (signed negative)
  pub carry_flag: bool,    // 'C': for SUB, 1 = no borrow; for ADD, 1 = carry out
  pub overflow_flag: bool, // 'V': signed overflow (result sign != expected)
}

impl Flags {
  pub fn new() -> Self {
    Self {
      zero_flag: false,
      negative_flag: false,
      carry_flag: false,
      overflow_flag: false,
    }
  }
}

pub struct State {
  pub is_running: bool,
}

impl State {
  pub fn new() -> Self {
    Self {
      is_running: false,
    }
  }
}