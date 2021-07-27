use bitvec::prelude::*;

use crate::instruction::Instruction;

#[allow(unused)]
use crate::symbol::{Rs, Rd, IW, IL, K, F, Address, FS, FE, N, Offset, Z, Condition};

pub fn disassemble_stage1(bytebuf: &[u8]) -> Vec<Instruction> {
    let mut inst_vec = vec![];
    let bitslice = bytebuf.view_bits::<Lsb0>();
    let mut word_iter = bitslice.chunks(16);
    loop {
        match word_iter.next() {
            Some(word) => {
                let upper7 = word.get(9..=15).unwrap().load::<u8>();
                let subop = word.get(5..=8).unwrap().load::<u8>();
                let rs = word.get(5..=8).unwrap().load::<u8>();
                let rf = *word.get(4).unwrap();
                let rd = word.get(0..=3).unwrap().load::<u8>();
                match upper7 {
                    0b0000000 => {
                        match subop {
                            1 => {
                                inst_vec.push(Instruction::Rev(Rd((rf as u8) << 4 | rd)));
                            },
                            8 => {
                                inst_vec.push(Instruction::Emu);
                            },
                            9 => {
                                inst_vec.push(Instruction::Exgpc(Rd((rf as u8) << 4 | rd)));
                            },
                            10 => {
                                inst_vec.push(Instruction::Getpc(Rd((rf as u8) << 4 | rd)));
                            },
                            11 => {
                                // this is kind of strange that we assign rd to Rs, but it's in the
                                // position of rd, and the manual calls it Rs because it's the
                                // source of the argument to jump to
                                inst_vec.push(Instruction::Jump(Rs((rf as u8) << 4 | rd)));
                            },
                            12 => {
                                inst_vec.push(Instruction::Getst(Rd((rf as u8) << 4 | rd)));
                            },
                            13 => {
                                inst_vec.push(Instruction::Putst(Rs((rf as u8) << 4 | rd)));
                            },
                            14 => {
                                inst_vec.push(Instruction::Popst);
                            },
                            15 => {
                                inst_vec.push(Instruction::Pushst);
                            },
                            _ => {}
                        }
                    },
                    0b0000001 => {
                        match subop {
                            8 => {
                                inst_vec.push(Instruction::Nop);
                            },
                            9 =>
                            {
                                inst_vec.push(Instruction::Clrc);
                            },
                            10 => {
                                //Instruction::MovbAbsoluteToAbsolute,
                                todo!("MovbAbsoluteToAbsolute");
                            },
                            11 => {
                                inst_vec.push(Instruction::Dint);
                            },
                            12 => {
                                inst_vec.push(Instruction::Abs(Rd((rf as u8) << 4 | rd)));
                            },
                            _ => {},
                        }
                    },
                    0b0101011 => {
                        if rs == rd {
                            inst_vec.push(Instruction::Clr(Rd((rf as u8) << 4 | rd)));
                        }
                        else {
                            inst_vec.push(Instruction::Xor(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                        }
                    },
                    _ => {}
                }
            },
            None  => {
                break;
            }
        }
    }
    inst_vec
}
