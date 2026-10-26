// ।। ॐ नमः शिवाय ।। \\
// @date_start 19th October, 2026

use crate::state::{Registers, Flags, State};
use crate::cpu_instructions;

pub struct CPU {
  pub registers: Registers,
  pub memory: Vec<u8>,
  pub pc: u64,
  pub flags: Flags,
  pub state: State,
}

impl CPU {
  pub fn new(memory_size: usize) -> Self {
    Self {
      registers: Registers::new(),
      memory: vec![0; memory_size],
      pc: 0,
      flags: Flags::new(),
      state: State::new(),
    }
  }
}

impl CPU {
  pub fn load_program_into_memory(&mut self, program: &Vec<u8>) {
    for (i, &byte) in program.iter().enumerate() {
      self.memory[i] = byte;
    }
  }

  pub fn clear_memory(&mut self) {
    self.registers.x.fill(0);
    self.memory.fill(0);
  }

  pub fn reset(&mut self) {
    self.registers.x.fill(0);
    self.memory.fill(0);
    self.pc = 0;
    self.flags.zero_flag = false;
    self.flags.negative_flag = false;
    self.flags.carry_flag = false;
    self.flags.overflow_flag = false;
    self.state.is_running = false;
  }
}

impl CPU {
  pub fn fetch_instruction(&self) -> u32 {
    let p = self.pc as usize;
    unsafe {
      u32::from_le_bytes([
        *self.memory.get_unchecked(p),
        *self.memory.get_unchecked(p + 1),
        *self.memory.get_unchecked(p + 2),
        *self.memory.get_unchecked(p + 3),
      ])
    }
  }
}

impl CPU {
  pub fn execute_instruction(&mut self) {
    // << fetch >> \\
    let instruction = self.fetch_instruction();

    // << decode: extract opcode (top 8 bits) >> \\
    let opcode = (instruction & 0xFF) as usize;

    // << execute >> \\
    if opcode < cpu_instructions::INSTRUCTION_HANDLERS.len() {
      cpu_instructions::INSTRUCTION_HANDLERS[opcode](self, &instruction)
    } else {
      panic!("Unknown or unimplemented instruction: {:#?}.", opcode);
    }
  }

  pub fn execute_program(&mut self, program: &Vec<u8>) {
    // << load + setup >> \\
    self.load_program_into_memory(program);

    // << cached pc copy to compare branching >> \\
    let mut old_pc = self.pc;
    self.state.is_running = true;
    while self.state.is_running && (self.pc as usize) < program.len() {
      // << execute >> \\
      old_pc = self.pc;
      self.execute_instruction();

      // << execute normally if branch didnt change pc >> \\
      if old_pc == self.pc {
        self.pc += 4;
      }
    }
  }
}