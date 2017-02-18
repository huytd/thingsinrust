use std::io::prelude::*;
use std::fs::File;

macro_rules! opcodes {
  (
    $hival:expr, $loval:expr;
    [ $(($high:expr, $low:expr) => $expression:block),* ]
  ) => {
    $(
      if $high == $hival && $low == $loval {
        $expression();
      }
    )*
  }
}

// 6502 Disassembler
// Instruction Set: http://e-tradition.net/bytes/6502/6502_instruction_set.html
fn main() {
  let mut f = File::open("code.hex").unwrap();
  let mut program = Vec::new();
  let length = f.read_to_end(&mut program).unwrap();
  let mut pc = 0;
  println!("PC  \t\t OP \t\tASM");
  println!("--  \t\t -- \t\t---------");
  while pc < length {
    let low = program[pc] & 0x000F;
    let high = (program[pc] & 0x00F0) >> 4;
    print!("{:04X}\t\t {:X}{:X}", pc, high, low);
    opcodes!(high, low; 
        [
          (0xA, 0x9) => {
            let param = program[pc + 1];
            println!("\t\t LDA #${:02X}", param);
            pc += 2;
          },
          (0x8, 0xD) => {
            let params = (program[pc + 1], program[pc + 2]);
            println!("\t\t STA ${:02X}{:02X}", params.1, params.0);
            pc += 3;
          }
        ]
    );
  }
}
