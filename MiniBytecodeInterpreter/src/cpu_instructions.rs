// ।। ॐ नमः शिवाय ।। \\
// @date_start 20th October, 2026

use crate::cpu::CPU;

// -- EXIT instruction -- \\
// -- opcode: 0x00 -- \\
// -- signature: [ opcode:8 | unused:8 | unused:8 | unused:8 ] -- \\
pub fn exit(cpu: &mut CPU, _: &u32) {
  cpu.state.is_running = false;
}
  
// -- MOV instruction -- \\
// -- opcode: 0x01 -- \\
// -- signature: [ opcode:8 | rd:8 | imm:16 ] -- \\
pub fn mov(cpu: &mut CPU, instruction: &u32) {
  // << setup >> \\
  let rd = ((instruction >> 8) & 0xFF) as usize;
  let imm = ((instruction >> 16) & 0xFFFF) as u64;

  cpu.registers.write(rd, imm);
}

// -- ADD instruction -- \\
// -- opcode: 0x02 -- \\
// -- signature: [ opcode:8 | rd:8 | rn:8 | rm:8 ] -- \\
pub fn add(cpu: &mut CPU, instruction: &u32) {
  // << setup >> \\
  let rd = ((instruction >> 8) & 0xFF) as usize;
  let rn = ((instruction >> 16) & 0xFF) as usize;
  let rm = ((instruction >> 24) & 0xFF) as usize;

  let sum = cpu.registers.read(rn).wrapping_add(cpu.registers.read(rm));
  cpu.registers.write(rd, sum);
}

// -- SUB instruction -- \\
// -- opcode: 0x03 -- \\
// -- signature: [ opcode:8 | rd:8 | rn:8 | rm:8 ] -- \\
pub fn sub(cpu: &mut CPU, instruction: &u32) {
  // << setup >> \\
  let rd = ((instruction >> 8) & 0xFF) as usize;
  let rn = ((instruction >> 16) & 0xFF) as usize;
  let rm = ((instruction >> 24) & 0xFF) as usize;

  let diff = cpu.registers.read(rn).wrapping_sub(cpu.registers.read(rm));
  cpu.registers.write(rd, diff);
}

// -- MUL instruction -- \\
// -- opcode: 0x04 -- \\
// -- signature: [ opcode:8 | rd:8 | rn:8 | rm:8 ] -- \\
pub fn mul(cpu: &mut CPU, instruction: &u32) {
  // << setup >> \\
  let rd = ((instruction >> 8) & 0xFF) as usize;
  let rn = ((instruction >> 16) & 0xFF) as usize;
  let rm = ((instruction >> 24) & 0xFF) as usize;

  let prod = cpu.registers.read(rn).wrapping_mul(cpu.registers.read(rm));

  cpu.registers.write(rd, prod);
}

// -- B instruction -- \\
// -- opcode: 0x05 -- \\
// -- signature: [ opcode:8 | target_address:24 ] -- \\
pub fn b(cpu: &mut CPU, instruction: &u32) {
  // << setup >> \\
  let target_address = ((instruction >> 8) & 0xFFFFFF) as u64;
  cpu.pc = target_address;
}

// -- B.EQ instruction -- \\
// -- opcode: 0x06 -- \\
// -- signature: [ opcode:8 | target_address:24 ] -- \\
pub fn beq(cpu: &mut CPU, instruction: &u32) {
  // << setup >> \\
  let target_address = ((instruction >> 8) & 0xFFFFFF) as u64;
  if cpu.flags.zero_flag {
    cpu.pc = target_address;
  }
}

// -- LDR instruction -- \\
// -- opcode: 0x07 -- \\
// -- signature: [ opcode:8 | rd:8 | rn:8 | imm: 8 ] -- \\
pub fn ldr(cpu: &mut CPU, instruction: &u32) {
  // << setup >> \\
  let rd = ((instruction >> 8) & 0xFF) as usize;
  let rn = ((instruction >> 16) & 0xFF) as usize;
  let imm = ((instruction >> 24) & 0xFF) as u64;

  // << target address = base address + imm >> \\
  let base_address = cpu.registers.read(rn);
  let target_address = (base_address + imm) as usize;

  // << read safely >> \\
  let bytes: [u8; 8] = cpu.memory[target_address..target_address + 8].try_into().expect("Out of bounds error caught at `ldr`.");
  let value = u64::from_le_bytes(bytes);

  cpu.registers.write(rd, value);
}

// -- STR instruction -- \\
// -- opcode: 0x08 -- \\
// -- signature: [ opcode:8 | rt:8 | rn:8 | imm:8 ] -- \\
pub fn str(cpu: &mut CPU, instruction: &u32) {
  let rt  = ((instruction >> 8)  & 0xFF) as usize;
  let rn  = ((instruction >> 16) & 0xFF) as usize;
  let imm = ((instruction >> 24) & 0xFF) as u64;

  let base = cpu.registers.read(rn);
  let addr = base.checked_add(imm).expect("STR: address overflow") as usize;
  let end  = addr.checked_add(8).expect("STR: address overflow");

  cpu.memory[addr..end].copy_from_slice(&u64::to_le_bytes(cpu.registers.read(rt)));
}

// -- CMP instruction (Register-Register) -- \\
// -- opcode: 0x09 -- \\
// -- signature: [ opcode:8 | rn:8 | rm:8 | unused:8 ] -- \\
pub fn cmp(cpu: &mut CPU, instruction: &u32) {
  // << setup >> \\
  let rn = ((instruction >> 8) & 0xFF) as usize;
  let rm = ((instruction >> 16) & 0xFF) as usize;

  let val1 = cpu.registers.read(rn);
  let val2 = cpu.registers.read(rm);

  // << Perform subtraction with overflow tracking >> \\
  let (diff, did_borrow) = val1.overflowing_sub(val2);

  // << Zero flag: result is 0 >> \\
  cpu.flags.zero_flag = diff == 0;

  // << Negative flag: top bit is 1 (signed interpretation) >> \\
  cpu.flags.negative_flag = (diff as i64) < 0;

  // << Carry flag: In ARM, C is 1 if there is NO borrow (!borrow) >> \\
  cpu.flags.carry_flag = !did_borrow;

  // << Overflow flag: Signed overflow occurs if operands had different signs >> \\
  // << and the result sign differs from the first operand (val1) >> \\
  let val1_i = val1 as i64;
  let val2_i = val2 as i64;
  let diff_i = diff as i64;
  cpu.flags.overflow_flag = (val1_i ^ val2_i) >= 0 && (val1_i ^ diff_i) < 0;
}

/*
 * When subtracting b from a (a - b):
 *   - Zero (Z): diff == 0
 *   - Negative (N): The most significant bit of diff is 1 (i.e., (diff as i64) < 0 or checking the top bit).
 *   - Carry / Borrow (C): In ARM, for subtraction, Carry acts as a NOT-borrow flag. C = 1 if a >= b (no borrow occurred),
 *     and C = 0 if a < b (borrow occurred). You can get this directly from .overflowing_sub(): overflow tells you if a
 *     borrow happened, so carry_flag = !overflow.
 *   - Overflow (V): Occurs if subtracting positive from negative (or vice versa) yields an unexpected sign change.
 */

// -- B.NE instruction -- \\
// -- opcode: 0x0A -- \\
// -- signature: [ opcode:8 | target_address:24 ] -- \\
pub fn bne(cpu: &mut CPU, instruction: &u32) {
  let target_address = ((instruction >> 8) & 0xFFFFFF) as u64;
  if !cpu.flags.zero_flag {
    cpu.pc = target_address;
  }
}

// -- B.LT instruction -- \\
// -- opcode: 0x0B -- \\
// -- [ opcode:8 | target_address:24 ] -- \\
pub fn blt(cpu: &mut CPU, instruction: &u32) {
  let target_address = ((instruction >> 8) & 0xFFFFFF) as u64;
  if cpu.flags.negative_flag != cpu.flags.overflow_flag {
    cpu.pc = target_address;
  }
}

// -- B.LE -- instruction -- \\
// -- opcode: 0x0C -- \\
// -- [ opcode:8 | target_address:24 ] -- \\
pub fn ble(cpu: &mut CPU, instruction: &u32) {
  let target_address = ((instruction >> 8) & 0xFFFFFF) as u64;
  if cpu.flags.zero_flag || cpu.flags.negative_flag != cpu.flags.overflow_flag {
    cpu.pc = target_address;
  }
}

// -- B.GT instruction -- \\
// -- opcode: 0x0D -- \\
// -- [ opcode:8 | target_address:24 ] -- \\
pub fn bgt(cpu: &mut CPU, instruction: &u32) {
  let target_address = ((instruction >> 8) & 0xFFFFFF) as u64;
  if !cpu.flags.zero_flag && cpu.flags.negative_flag == cpu.flags.overflow_flag {
    cpu.pc = target_address;
  }
}


// -- B.GE instruction -- \\
// -- opcode: 0x0E -- \\
// -- [ opcode:8 | target_address:24 ] -- \\
pub fn bge(cpu: &mut CPU, instruction: &u32) {
  let target_address = ((instruction >> 8) & 0xFFFFFF) as u64;
  if cpu.flags.negative_flag == cpu.flags.overflow_flag {
    cpu.pc = target_address;
  }
}

// -- ADDI instruction -- \\
// -- opcode: 0x0F -- \\
// -- signature: [ opcode:8 | rd:8 | rn:8 | imm:8 ] -- \\
pub fn addi(cpu: &mut CPU, instruction: &u32) {
  // << setup >> \\
  let rd = ((instruction >> 8) & 0xFF) as usize;
  let rn = ((instruction >> 16) & 0xFF) as usize;
  let imm = ((instruction >> 24) & 0xFF) as u64;

  let sum = cpu.registers.read(rn).wrapping_add(imm);
  cpu.registers.write(rd, sum);
}

// -- SUBI instruction -- \\
// -- opcode: 0x10 -- \\
// -- signature: [ opcode:8 | rd:8 | rn:8 | imm:8 ] -- \\
pub fn subi(cpu: &mut CPU, instruction: &u32) {
  // << setup >> \\
  let rd = ((instruction >> 8) & 0xFF) as usize;
  let rn = ((instruction >> 16) & 0xFF) as usize;
  let imm = ((instruction >> 24) & 0xFF) as u64;

  let diff = cpu.registers.read(rn).wrapping_sub(imm);
  cpu.registers.write(rd, diff);
}

// -- MULI instruction -- \\
// -- opcode: 0x11 -- \\
// -- signature: [ opcode:8 | rd:8 | rn:8 | imm:8 ] -- \\
pub fn muli(cpu: &mut CPU, instruction: &u32) {
  // << setup >> \\
  let rd = ((instruction >> 8) & 0xFF) as usize;
  let rn = ((instruction >> 16) & 0xFF) as usize;
  let imm = ((instruction >> 24) & 0xFF) as u64;

  let prod = cpu.registers.read(rn).wrapping_mul(imm);
  cpu.registers.write(rd, prod);
}

// -- DIV instruction -- \\
// -- opcode: 0x12 -- \\
// -- signature: [ opcode:8 | rd:8 | rn:8 | rm:8 ] -- \\
pub fn div(cpu: &mut CPU, instruction: &u32) {
  // << setup >> \\
  let rd = ((instruction >> 8) & 0xFF) as usize;
  let rn = ((instruction >> 16) & 0xFF) as usize;
  let rm = ((instruction >> 24) & 0xFF) as usize;

  let divisor = cpu.registers.read(rm);
  if divisor == 0 {
    panic!("ZeroDivisionError: caught at div.");
  }
  let quo = cpu.registers.read(rn).wrapping_div(divisor);
  cpu.registers.write(rd, quo);
}

// -- DIVI instruction -- \\
// -- opcode: 0x13 -- \\
// -- signature: [ opcode:8 | rd:8 | rn:8 | imm:8 ] -- \\
pub fn divi(cpu: &mut CPU, instruction: &u32) {
  // << setup >> \\
  let rd = ((instruction >> 8) & 0xFF) as usize;
  let rn = ((instruction >> 16) & 0xFF) as usize;
  let imm = ((instruction >> 24) & 0xFF) as u64;

  if imm == 0 {
    panic!("ZeroDivisionError: caught at divi.");
  }
  let quo = cpu.registers.read(rn).wrapping_div(imm);
  cpu.registers.write(rd, quo);
}


// -- REM instruction (Register-Register) -- \\
// -- opcode: 0x14 -- \\
// -- signature: [ opcode:8 | rd:8 | rn:8 | rm:8 ] -- \\
pub fn rem(cpu: &mut CPU, instruction: &u32) {
  // << setup >> \\
  let rd = ((instruction >> 8) & 0xFF) as usize;
  let rn = ((instruction >> 16) & 0xFF) as usize;
  let rm = ((instruction >> 24) & 0xFF) as usize;

  let divisor = cpu.registers.read(rm);
  if divisor == 0 {
    panic!("ZeroDivisionError: caught at rem.");
  }
  let rem = cpu.registers.read(rn).wrapping_rem(divisor);
  cpu.registers.write(rd, rem);
}

// -- REMI instruction (Register-Immediate) -- \\
// -- opcode: 0x15 -- \\
// -- signature: [ opcode:8 | rd:8 | rn:8 | imm:8 ] -- \\
pub fn remi(cpu: &mut CPU, instruction: &u32) {
  // << setup >> \\
  let rd = ((instruction >> 8) & 0xFF) as usize;
  let rn = ((instruction >> 16) & 0xFF) as usize;
  let imm = ((instruction >> 24) & 0xFF) as u64;

  if imm == 0 {
    panic!("ZeroDivisionError: caught at remi.");
  }
  let rem = cpu.registers.read(rn).wrapping_rem(imm);
  cpu.registers.write(rd, rem);
}

// -- AND instruction -- \\
// -- opcode: 0x16 -- \\
// -- signature: [ opcode:8 | rd:8 | rn:8 | rm:8 ] -- \\
pub fn and(cpu: &mut CPU, instruction: &u32) {
  // << setup >> \\
  let rd = ((instruction >> 8) & 0xFF) as usize;
  let rn = ((instruction >> 16) & 0xFF) as usize;
  let rm = ((instruction >> 24) & 0xFF) as usize;

  let val = cpu.registers.read(rn) & cpu.registers.read(rm);
  cpu.registers.write(rd, val);
}

// -- ANDI instruction -- \\
// -- opcode: 0x17 -- \\
// -- signature: [ opcode:8 | rd:8 | rn:8 | imm:8 ] -- \\
pub fn andi(cpu: &mut CPU, instruction: &u32) {
  // << setup >> \\
  let rd = ((instruction >> 8) & 0xFF) as usize;
  let rn = ((instruction >> 16) & 0xFF) as usize;
  let imm = ((instruction >> 24) & 0xFF) as u64;

  let val = cpu.registers.read(rn) & imm;
  cpu.registers.write(rd, val);
}

// -- ORR instruction -- \\
// -- opcode: 0x18 -- \\
// -- signature: [ opcode:8 | rd:8 | rn:8 | rm:8 ] -- \\
pub fn orr(cpu: &mut CPU, instruction: &u32) {
  // << setup >> \\
  let rd = ((instruction >> 8) & 0xFF) as usize;
  let rn = ((instruction >> 16) & 0xFF) as usize;
  let rm = ((instruction >> 24) & 0xFF) as usize;

  let val = cpu.registers.read(rn) | cpu.registers.read(rm);
  cpu.registers.write(rd, val);
}

// -- ORRI instruction -- \\
// -- opcode: 0x19 -- \\
// -- signature: [ opcode:8 | rd:8 | rn:8 | imm:8 ] -- \\
pub fn orri(cpu: &mut CPU, instruction: &u32) {
  // << setup >> \\
  let rd = ((instruction >> 8) & 0xFF) as usize;
  let rn = ((instruction >> 16) & 0xFF) as usize;
  let imm = ((instruction >> 24) & 0xFF) as u64;

  let val = cpu.registers.read(rn) | imm;
  cpu.registers.write(rd, val);
}

// -- EOR instruction -- \\
// -- opcode: 0x1A -- \\
// -- signature: [ opcode:8 | rd:8 | rn:8 | rm:8 ] -- \\
pub fn eor(cpu: &mut CPU, instruction: &u32) {
  // << setup >> \\
  let rd = ((instruction >> 8) & 0xFF) as usize;
  let rn = ((instruction >> 16) & 0xFF) as usize;
  let rm = ((instruction >> 24) & 0xFF) as usize;

  let val = cpu.registers.read(rn) ^ cpu.registers.read(rm);
  cpu.registers.write(rd, val);
}

// -- EORI instruction -- \\
// -- opcode: 0x1B -- \\
// -- signature: [ opcode:8 | rd:8 | rn:8 | imm:8 ] -- \\
pub fn eori(cpu: &mut CPU, instruction: &u32) {
  // << setup >> \\
  let rd = ((instruction >> 8) & 0xFF) as usize;
  let rn = ((instruction >> 16) & 0xFF) as usize;
  let imm = ((instruction >> 24) & 0xFF) as u64;

  let val = cpu.registers.read(rn) ^ imm;
  cpu.registers.write(rd, val);
}

// -- LSL instruction (Logical Shift Left) -- \\
// -- opcode: 0x1C -- \\
// -- signature: [ opcode:8 | rd:8 | rn:8 | rm:8 ] -- \\
pub fn lsl(cpu: &mut CPU, instruction: &u32) {
  // << setup >> \\
  let rd = ((instruction >> 8) & 0xFF) as usize;
  let rn = ((instruction >> 16) & 0xFF) as usize;
  let rm = ((instruction >> 24) & 0xFF) as usize;

  let shift = (cpu.registers.read(rm) & 0x3F) as u32;
  let val = cpu.registers.read(rn) << shift;
  cpu.registers.write(rd, val);
}

// -- LSLI instruction (Logical Shift Left Immediate) -- \\
// -- opcode: 0x1D -- \\
// -- signature: [ opcode:8 | rd:8 | rn:8 | imm:8 ] -- \\
pub fn lsli(cpu: &mut CPU, instruction: &u32) {
  // << setup >> \\
  let rd = ((instruction >> 8) & 0xFF) as usize;
  let rn = ((instruction >> 16) & 0xFF) as usize;
  let imm_shift = ((instruction >> 24) & 0x3F) as u32;

  let val = cpu.registers.read(rn) << imm_shift;
  cpu.registers.write(rd, val);
}

// -- LSR instruction (Logical Shift Right) -- \\
// -- opcode: 0x1E -- \\
// -- signature: [ opcode:8 | rd:8 | rn:8 | rm:8 ] -- \\
pub fn lsr(cpu: &mut CPU, instruction: &u32) {
  // << setup >> \\
  let rd = ((instruction >> 8) & 0xFF) as usize;
  let rn = ((instruction >> 16) & 0xFF) as usize;
  let rm = ((instruction >> 24) & 0xFF) as usize;

  let shift = (cpu.registers.read(rm) & 0x3F) as u32;
  let val = cpu.registers.read(rn) >> shift;
  cpu.registers.write(rd, val);
}

// -- LSRI instruction (Logical Shift Right Immediate) -- \\
// -- opcode: 0x1F -- \\
// -- signature: [ opcode:8 | rd:8 | rn:8 | imm:8 ] -- \\
pub fn lsri(cpu: &mut CPU, instruction: &u32) {
  // << setup >> \\
  let rd = ((instruction >> 8) & 0xFF) as usize;
  let rn = ((instruction >> 16) & 0xFF) as usize;
  let imm_shift = ((instruction >> 24) & 0x3F) as u32;

  let val = cpu.registers.read(rn) >> imm_shift;
  cpu.registers.write(rd, val);
}

// -- ASR instruction (Arithmetic Shift Right - preserves sign bit) -- \\
// -- opcode: 0x20 -- \\
// -- signature: [ opcode:8 | rd:8 | rn:8 | rm:8 ] -- \\
pub fn asr(cpu: &mut CPU, instruction: &u32) {
  // << setup >> \\
  let rd = ((instruction >> 8) & 0xFF) as usize;
  let rn = ((instruction >> 16) & 0xFF) as usize;
  let rm = ((instruction >> 24) & 0xFF) as usize;

  let shift = (cpu.registers.read(rm) & 0x3F) as u32;
  // Cast to i64 to perform an arithmetic (signed) shift in Rust
  let val = (cpu.registers.read(rn) as i64 >> shift) as u64;
  cpu.registers.write(rd, val);
}

// -- ASRI instruction (Arithmetic Shift Right Immediate) -- \\
// -- opcode: 0x21 -- \\
// -- signature: [ opcode:8 | rd:8 | rn:8 | imm:8 ] -- \\
pub fn asri(cpu: &mut CPU, instruction: &u32) {
  // << setup >> \\
  let rd = ((instruction >> 8) & 0xFF) as usize;
  let rn = ((instruction >> 16) & 0xFF) as usize;
  let imm_shift = ((instruction >> 24) & 0x3F) as u32;

  // Cast to i64 to perform an arithmetic (signed) shift in Rust
  let val = (cpu.registers.read(rn) as i64 >> imm_shift) as u64;
  cpu.registers.write(rd, val);
}

// -- BL instruction (Branch with Link) -- \\
// -- opcode: 0x22 -- \\
// -- signature: [ opcode:8 | target_address:24 ] -- \\
pub fn bl(cpu: &mut CPU, instruction: &u32) {
  let target_address = ((instruction >> 8) & 0xFFFFFF) as u64;
  // << save return address (current PC + 4) into register x30 (standard ARM link register) >> \\
  cpu.registers.write(30, cpu.pc + 4);
  cpu.pc = target_address;
}

// -- RET instruction (Return from subroutine) -- \\
// -- opcode: 0x23 -- \\
// -- signature: [ opcode:8 | unused:24 ] -- \\
pub fn ret(cpu: &mut CPU, _: &u32) {
  // << jump back to the address stored in x30 >> \\
  cpu.pc = cpu.registers.read(30);
}

// -- LDRB instruction (Load Register Byte) -- \\
// -- opcode: 0x24 -- \\
// -- signature: [ opcode:8 | rd:8 | rn:8 | imm:8 ] -- \\
pub fn ldrb(cpu: &mut CPU, instruction: &u32) {
  // << setup >> \\
  let rd = ((instruction >> 8) & 0xFF) as usize;
  let rn = ((instruction >> 16) & 0xFF) as usize;
  let imm = ((instruction >> 24) & 0xFF) as u64;

  let target_address = (cpu.registers.read(rn) + imm) as usize;
  let value = cpu.memory[target_address] as u64;
  cpu.registers.write(rd, value);
}

// -- STRB instruction -- \\
// -- opcode: 0x25 -- \\
// -- signature: [ opcode:8 | rt:8 | rn:8 | imm:8 ] -- \\
pub fn strb(cpu: &mut CPU, instruction: &u32) {
  // << setup >> \\
  let rt = ((instruction >> 8) & 0xFF) as usize;
  let rn = ((instruction >> 16) & 0xFF) as usize;
  let imm = ((instruction >> 24) & 0xFF) as u64;

  let value = cpu.registers.read(rt) as u8;
  let target_address = (cpu.registers.read(rn) + imm) as usize;
  cpu.memory[target_address] = value;
}

// -- MOVR instruction (Move Register-to-Register) -- \\
// -- opcode: 0x26 -- \\
// -- signature: [ opcode:8 | rd:8 | rn:8 | unused:8 ] -- \\
pub fn movr(cpu: &mut CPU, instruction: &u32) {
  // << setup >> \\
  let rd = ((instruction >> 8) & 0xFF) as usize;
  let rn = ((instruction >> 16) & 0xFF) as usize;

  cpu.registers.write(rd, cpu.registers.read(rn));
}

// -- CMPI instruction (Register-Immediate) -- \\
// -- opcode: 0x27 -- \\
// -- signature: [ opcode:8 | rn:8 | imm:16 ] -- \\
pub fn cmpi(cpu: &mut CPU, instruction: &u32) {
  // << setup >> \\
  let rn = ((instruction >> 8) & 0xFF) as usize;
  let imm = ((instruction >> 16) & 0xFFFF) as u64;

  let val1 = cpu.registers.read(rn);

  // << Perform subtraction with overflow tracking >> \\
  let (diff, did_borrow) = val1.overflowing_sub(imm);

  // << Zero flag: result is 0 >> \\
  cpu.flags.zero_flag = diff == 0;

  // << Negative flag: top bit is 1 (signed interpretation) >> \\
  cpu.flags.negative_flag = (diff as i64) < 0;

  // << Carry flag: In ARM, C is 1 if there is NO borrow (!borrow) >> \\
  cpu.flags.carry_flag = !did_borrow;

  // << Overflow flag: Signed overflow occurs if operands had different signs >> \\
  // << and the result sign differs from the first operand (val1) >> \\
  let val1_i = val1 as i64;
  let imm_i = imm as i64;
  let diff_i = diff as i64;
  cpu.flags.overflow_flag = (val1_i ^ imm_i) >= 0 && (val1_i ^ diff_i) < 0;
}

// -- ADDS instruction (Add Register-Register & Set Flags) -- \\
// -- opcode: 0x28 -- \\
// -- signature: [ opcode:8 | rd:8 | rn:8 | rm:8 ] -- \\
pub fn adds(cpu: &mut CPU, instruction: &u32) {
  // << setup >> \\
  let rd = ((instruction >> 8) & 0xFF) as usize;
  let rn = ((instruction >> 16) & 0xFF) as usize;
  let rm = ((instruction >> 24) & 0xFF) as usize;

  let val1 = cpu.registers.read(rn);
  let val2 = cpu.registers.read(rm);

  // << Perform addition with carry tracking >> \\
  let (sum, did_carry) = val1.overflowing_add(val2);

  // << Zero flag: result is 0 >> \\
  cpu.flags.zero_flag = sum == 0;

  // << Negative flag: top bit is 1 (signed interpretation) >> \\
  cpu.flags.negative_flag = (sum as i64) < 0;

  // << Carry flag: 1 if an unsigned carry out occurred >> \\
  cpu.flags.carry_flag = did_carry;

  // << Overflow flag: Signed overflow occurs if operands had the same sign >> \\
  // << and the result sign differs from the operands >> \\
  let val1_i = val1 as i64;
  let val2_i = val2 as i64;
  let sum_i = sum as i64;
  cpu.flags.overflow_flag = (val1_i ^ val2_i) < 0 && (val1_i ^ sum_i) >= 0;

  // << Write sum to destination register >> \\
  cpu.registers.write(rd, sum);
}

// -- SUBS instruction (Subtract Register-Register & Set Flags)-- \\
// -- opcode: 0x29 -- \\
// -- signature: [ opcode:8 | rd:8 | rn:8 | rm:8 ] -- \\
pub fn subs(cpu: &mut CPU, instruction: &u32) {
  // << setup >> \\
  let rd = ((instruction >> 8) & 0xFF) as usize;
  let rn = ((instruction >> 16) & 0xFF) as usize;
  let rm = ((instruction >> 24) & 0xFF) as usize;

  let val1 = cpu.registers.read(rn);
  let val2 = cpu.registers.read(rm);

  // << Perform subtraction with borrow tracking >> \\
  let (diff, did_borrow) = val1.overflowing_sub(val2);

  // << Zero flag: result is 0 >> \\
  cpu.flags.zero_flag = diff == 0;

  // << Negative flag: top bit is 1 (signed interpretation) >> \\
  cpu.flags.negative_flag = (diff as i64) < 0;

  // << Carry flag: In ARM, C is 1 if there is NO borrow (!borrow) >> \\
  cpu.flags.carry_flag = !did_borrow;

  // << Overflow flag: Signed overflow tracking >> \\
  let val1_i = val1 as i64;
  let val2_i = val2 as i64;
  let diff_i = diff as i64;
  cpu.flags.overflow_flag = (val1_i ^ val2_i) >= 0 && (val1_i ^ diff_i) < 0;

  // << Write difference to destination register >> \\
  cpu.registers.write(rd, diff);
}

// -- ADDIS instruction (Add Register-Immediate & Set Flags)-- \\
// -- opcode: 0x2A -- \\
// -- signature: [ opcode:8 | rd:8 | rn:8 | imm:8 ] -- \\
pub fn addis(cpu: &mut CPU, instruction: &u32) {
  // << setup >> \\
  let rd = ((instruction >> 8) & 0xFF) as usize;
  let rn = ((instruction >> 16) & 0xFF) as usize;
  let imm = ((instruction >> 24) & 0xFF) as u64;

  let val1 = cpu.registers.read(rn);

  // << Perform addition with carry tracking >> \\
  let (sum, did_carry) = val1.overflowing_add(imm);

  // << Zero flag: result is 0 >> \\
  cpu.flags.zero_flag = sum == 0;

  // << Negative flag: top bit is 1 (signed interpretation) >> \\
  cpu.flags.negative_flag = (sum as i64) < 0;

  // << Carry flag: 1 if an unsigned carry out occurred >> \\
  cpu.flags.carry_flag = did_carry;

  // << Overflow flag: Signed overflow occurs if operands had the same sign >> \\
  // << and the result sign differs from the operands >> \\
  let val1_i = val1 as i64;
  let imm_i = imm as i64;
  let sum_i = sum as i64;
  cpu.flags.overflow_flag = (val1_i ^ imm_i) < 0 && (val1_i ^ sum_i) >= 0;

  // << Write sum to destination register >> \\
  cpu.registers.write(rd, sum);
}

// -- SUBIS instruction (Subtract Register-Immediate & Set Flags) -- \\
// -- opcode: 0x2B -- \\
// -- signature: [ opcode:8 | rd:8 | rn:8 | imm:8 ] -- \\
pub fn subis(cpu: &mut CPU, instruction: &u32) {
  // << setup >> \\
  let rd = ((instruction >> 8) & 0xFF) as usize;
  let rn = ((instruction >> 16) & 0xFF) as usize;
  let imm = ((instruction >> 24) & 0xFF) as u64;

  let val1 = cpu.registers.read(rn);

  // << Perform subtraction with borrow tracking >> \\
  let (diff, did_borrow) = val1.overflowing_sub(imm);

  // << Zero flag: result is 0 >> \\
  cpu.flags.zero_flag = diff == 0;

  // << Negative flag: top bit is 1 (signed interpretation) >> \\
  cpu.flags.negative_flag = (diff as i64) < 0;

  // << Carry flag: In ARM, C is 1 if there is NO borrow (!borrow) >> \\
  cpu.flags.carry_flag = !did_borrow;

  // << Overflow flag: Signed overflow tracking >> \\
  let val1_i = val1 as i64;
  let imm_i = imm as i64;
  let diff_i = diff as i64;
  cpu.flags.overflow_flag = (val1_i ^ imm_i) >= 0 && (val1_i ^ diff_i) < 0;

  // << Write difference to destination register >> \\
  cpu.registers.write(rd, diff);
}

// -- Define the function pointer type for instructions -- \\
pub type InstructionFn = fn(&mut CPU, &u32);

// -- Complete dispatch table mapped sequentially for all opcodes -- \\
pub const INSTRUCTION_HANDLERS: [InstructionFn; 44] = [
  exit,    // 0x00
  mov,     // 0x01
  add,     // 0x02
  sub,     // 0x03
  mul,     // 0x04
  b,       // 0x05
  beq,     // 0x06
  ldr,     // 0x07
  str,     // 0x08
  cmp,     // 0x09
  bne,     // 0x0A
  blt,     // 0x0B
  ble,     // 0x0C
  bgt,     // 0x0D
  bge,     // 0x0E
  addi,    // 0x0F
  subi,    // 0x10
  muli,    // 0x11
  div,     // 0x12
  divi,    // 0x13
  rem,     // 0x14
  remi,    // 0x15
  and,     // 0x16
  andi,    // 0x17
  orr,     // 0x18
  orri,    // 0x19
  eor,     // 0x1A
  eori,    // 0x1B
  lsl,     // 0x1C
  lsli  ,  // 0x1D
  lsr,     // 0x1E
  lsri,    // 0x1F
  asr,     // 0x20
  asri,    // 0x21
  bl,      // 0x22
  ret,     // 0x23
  ldrb,    // 0x24
  strb,    // 0x25
  movr,    // 0x26
  cmpi,    // 0x27
  adds,    // 0x28
  subs,    // 0x29
  addis,   // 0x2A
  subis,   // 0x2B
];
