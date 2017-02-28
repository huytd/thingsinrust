use std::io::prelude::*;
use std::fs::File;

// Instruction Set: http://e-tradition.net/bytes/6502/6502_instruction_set.html

#[allow(unused_variables)]
#[allow(unused_mut)]
fn main() {
    let mut f = File::open("code.hex").unwrap();
    let mut program = Vec::new();
    let length = f.read_to_end(&mut program).unwrap() as u16;
    let mut pc = 0u16;
    let mut ac = 0u8;
    let mut x = 0u8;
    let mut y = 0u8;
    let mut sr = 0u8;
    let mut sp = 0u8; // 0xNV0BDIZC
    println!("PC  \t\t OP \t\tASM");
    println!("--  \t\t -- \t\t---------");
    while pc < length {
        let low = program[pc as usize] & 0x000F;
        let high = (program[pc as usize] & 0x00F0) >> 4;
        print!("{:04X}\t\t {:X}{:X}", pc, high, low);
        match (high, low) {
            (0xA, 0x9) => {
                let param = program[(pc as usize) + 1];
                println!("\t\t LDA #${:02X}", param);
                pc += 2;
            },
            (0x8, 0xD) => {
                let params = (program[(pc as usize) + 1], program[(pc as usize) + 2]);
                println!("\t\t STA ${:02X}{:02X}", params.1, params.0);
                pc += 3;
            },
            (_, _) => {}
        }
    }
}
