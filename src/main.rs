use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;

mod disasm;
mod instruction;
mod symbol;

/// Stage 1 test, byte buffer
#[allow(unused)]
fn test1() {
    println!(
        "{:?}",
        disasm::disassemble_stage1(
            &[

                0x21, 0x00, // REV A1
                0x00, 0x01, // EMU
                0x27, 0x01, // EXGPC A7
                0x56, 0x01, // GETPC B8
                0x66, 0x01, // JUMP A6
                0x83, 0x01, // GETST A3
                0xA3, 0x01, // PUTST A3
                0xC0, 0x01, // POPST
                0xE0, 0x01, // PUSHST
                0x00, 0x03, // NOP
                0x20, 0x03, // CLRC
                0x60, 0x03, // DINT
                0x82, 0x03, // ABS A2
                0x50, 0x05, // SETF 0x10, 0, 0
                0x01, 0x0b, 0x00, 0x01,  // ADDI 0x1000, A1
                0x22, 0x0b, 0xAA, 0xAA, 0x55, 0x55, // ADDI 0x5555AAAA, A2
                0x21, 0x0b, 0x00, 0x00, 0x00, 0x01, // ADDI 0x1000_0000, A1
                0x57, 0x57, // XOR B10, B7 
                0x39, 0x57, // CLR B9
            ],
            0
        )
    );
}

/// Stage 1 test, TUTOR_C program
#[allow(unused)]
fn test2() {
    let mut file = File::open("/home/paul/Downloads/gspa-dos/tutor_c.out").unwrap();
    file.seek(SeekFrom::Start(0x148)).unwrap();
    let mut bytes = vec![0u8; 4682];
    file.read(&mut bytes).unwrap();
    println!("{:?}", disasm::disassemble_stage1(&bytes, 0x2000));
}

/// Stage 2 test, byte buffer
#[allow(unused)]
fn test3() {
    disasm::disassemble(&[0x91, 0x18], 0);
}

/// Stage 2 test, TUTOR_C program
#[allow(unused)]
fn test4() {
    let mut file = File::open("/home/paul/Downloads/gspa-dos/tutor_c.out").unwrap();
    file.seek(SeekFrom::Start(0x148)).unwrap();
    let mut bytes = vec![0u8; 4682];
    file.read(&mut bytes).unwrap();
    disasm::disassemble(&bytes, 0x2000);
}

fn main() {
    // super hacky, just for quick testing
    let mut iter = std::env::args().into_iter();
    iter.next();
    match iter.next() {
        Some(num) => match num.parse::<u8>().unwrap() {
            1 => test1(),
            2 => test2(),
            3 => test3(),
            4 => test4(),
            _ => (),
        },
        None => (),
    }
}
