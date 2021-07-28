/*use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;*/

pub mod instruction;
pub mod symbol;
pub mod disasm;

fn main() {
    println!("{:?}", disasm::disassemble_stage1(&[
            // REV A1
            0x21, 0x00,
            // EMU
            0x00, 0x01,
            // EXGPC A7
            0x27, 0x01,
            // GETPC B8
            0x56, 0x01,
            // JUMP A6
            0x66, 0x01,
            // GETST A3
            0x83, 0x01,
            // PUTST A3
            0xA3, 0x01,
            // POPST
            0xC0, 0x01,
            // PUSHST
            0xE0, 0x01,
            // NOP
            0x00, 0x03,        
            // CLRC
            0x20, 0x03,
            // DINT
            0x60, 0x03,
            // ABS A2
            0x82,  0x03,
            // SETF 0x10, 0, 0
            0x50, 0x05,
            // ADDI 0x1000, A1
            0x01, 0x0b, 0x00, 0x01,
            // ADDI 0x5555AAAA, A2
            0x22, 0x0b, 0xAA, 0xAA, 0x55, 0x55,
            // ADDI 0x1000_0000, A1
            0x21, 0x0b, 0x00, 0x00, 0x00, 0x01,
            // XOR B10, B7
            0x57, 0x57,
            // CLR B9
            0x39, 0x57,
    ]));

    /*let mut file = File::open("/home/paul/Downloads/gspa-dos/tutor_c.out").unwrap();
    file.seek(SeekFrom::Start(0x146)).unwrap();
    let mut bytes = vec![0u8; 100];
    file.read(&mut bytes).unwrap();
    println!("{:?}", disasm::disassemble_stage1(&bytes));*/
}
