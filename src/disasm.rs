use bitvec::prelude::*;

use crate::instruction::Instruction;
use core::fmt::{self, Formatter, Write};

use crate::symbol::{
    Address, Condition, Offset, Offset8, Rd, RegList, Rs, D, F, FE, FS, IL, IW, K, N, PC, Z,
};

pub fn disassemble_stage1(bytebuf: &[u8], start_addr: usize) -> Vec<(usize, Instruction, Vec<u16>)> {
    let mut inst_vec = vec![];
    let bitslice = bytebuf.view_bits::<Lsb0>();
    let mut word_iter = bitslice.chunks(16).enumerate();
    loop {
        match word_iter.next() {
            Some((pc, word)) => {
                let upper7 = word.get(9..=15).unwrap().load::<u8>();
                let subop = word.get(5..=8).unwrap().load::<u8>();
                let rs = word.get(5..=8).unwrap().load::<u8>();
                let rf = *word.get(4).unwrap();
                let rd = word.get(0..=3).unwrap().load::<u8>();
                let f = *word.get(9).unwrap();
                let fs = word.get(0..=4).unwrap().load::<u8>();
                let n = word.get(0..=4).unwrap().load::<u8>();
                let fe = *word.get(5).unwrap();
                let k = word.get(5..=9).unwrap().load::<u8>();
                let d = *word.get(10).unwrap();
                let z = *word.get(7).unwrap();
                let cc = word.get(8..=11).unwrap().load::<u8>();
                match upper7 {
                    0b0000000 => {
                        match subop {
                            1 => {
                                inst_vec.push((
                                    pc + start_addr,
                                    Instruction::Rev(Rd((rf as u8) << 4 | rd)),
                                    vec![word.load::<u16>()],
                                ));
                            }
                            8 => {
                                inst_vec.push((
                                    pc + start_addr,
                                    Instruction::Emu,
                                    vec![word.load::<u16>()],
                                ));
                            }
                            9 => {
                                inst_vec.push((
                                    pc + start_addr,
                                    Instruction::Exgpc(Rd((rf as u8) << 4 | rd)),
                                    vec![word.load::<u16>()],
                                ));
                            }
                            10 => {
                                inst_vec.push((
                                    pc + start_addr,
                                    Instruction::Getpc(Rd((rf as u8) << 4 | rd)),
                                    vec![word.load::<u16>()],
                                ));
                            }
                            11 => {
                                // this is kind of strange that we assign rd to Rs, but it's in the
                                // position of rd, and the manual calls it Rs because it's the
                                // source of the argument to jump to
                                inst_vec.push((
                                    pc + start_addr,
                                    Instruction::Jump(Rs((rf as u8) << 4 | rd)),
                                    vec![word.load::<u16>()],
                                ));
                            }
                            12 => {
                                inst_vec.push((
                                    pc + start_addr,
                                    Instruction::Getst(Rd((rf as u8) << 4 | rd)),
                                    vec![word.load::<u16>()],
                                ));
                            }
                            13 => {
                                inst_vec.push((
                                    pc + start_addr,
                                    Instruction::Putst(Rs((rf as u8) << 4 | rd)),
                                    vec![word.load::<u16>()],
                                ));
                            }
                            14 => {
                                inst_vec.push((
                                    pc + start_addr,
                                    Instruction::Popst,
                                    vec![word.load::<u16>()],
                                ));
                            }
                            15 => {
                                inst_vec.push((
                                    pc + start_addr,
                                    Instruction::Pushst,
                                    vec![word.load::<u16>()],
                                ));
                            }
                            _ => {}
                        }
                    }
                    0b0000001 => match subop {
                        8 => {
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Nop,
                                vec![word.load::<u16>()],
                            ));
                        }
                        9 => {
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Clrc,
                                vec![word.load::<u16>()],
                            ));
                        }
                        10 => {
                            let mut src_addr: u32 = 0;
                            let mut dst_addr: u32 = 0;
                            let src_lsb = word_iter.next().unwrap().1.load::<u16>();
                            let src_msb = word_iter.next().unwrap().1.load::<u16>();
                            let dst_lsb = word_iter.next().unwrap().1.load::<u16>();
                            let dst_msb = word_iter.next().unwrap().1.load::<u16>();
                            src_addr.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(src_lsb);
                            src_addr.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(src_msb);
                            dst_addr.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(dst_lsb);
                            dst_addr.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(dst_msb);
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::MovbAbsoluteToAbsolute(
                                    Address(src_addr),
                                    Address(dst_addr),
                                ),
                                vec![word.load::<u16>(), src_lsb, src_msb, dst_lsb, dst_msb],
                            ));
                        }
                        11 => {
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Dint,
                                vec![word.load::<u16>()],
                            ));
                        }
                        12 => {
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Abs(Rd((rf as u8) << 4 | rd)),
                                vec![word.load::<u16>()],
                            ));
                        }
                        13 => {
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Neg(Rd((rf as u8) << 4 | rd)),
                                vec![word.load::<u16>()],
                            ));
                        }
                        14 => {
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Negb(Rd((rf as u8) << 4 | rd)),
                                vec![word.load::<u16>()],
                            ));
                        }
                        15 => {
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Not(Rd((rf as u8) << 4 | rd)),
                                vec![word.load::<u16>()],
                            ));
                        }
                        _ => {}
                    },
                    0b0000010 | 0b0000011 => {
                        match subop {
                            8 => {
                                inst_vec.push((
                                    pc + start_addr,
                                    Instruction::Sext(Rd((rf as u8) << 4 | rd), F(f)),
                                    vec![word.load::<u16>()],
                                ));
                            }
                            9 => {
                                inst_vec.push((
                                    pc + start_addr,
                                    Instruction::Zext(Rd((rf as u8) << 4 | rd), F(f)),
                                    vec![word.load::<u16>()],
                                ));
                            }
                            10 | 11 => {
                                inst_vec.push((
                                    pc + start_addr,
                                    Instruction::Setf(FS(fs), FE(fe), Some(F(f))),
                                    vec![word.load::<u16>()],
                                ));
                            }
                            12 => {
                                let mut address: u32 = 0;
                                let lsb = word_iter.next().unwrap().1.load::<u16>();
                                let msb = word_iter.next().unwrap().1.load::<u16>();
                                address.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                                address.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                                inst_vec.push((
                                    pc + start_addr,
                                    Instruction::MoveFieldRegToAbsolute(
                                        Rs((rf as u8) << 4 | rd),
                                        Address(address),
                                        Some(F(f)),
                                    ),
                                    vec![word.load::<u16>(), lsb, msb],
                                ));
                            }
                            13 => {
                                let mut address: u32 = 0;
                                let lsb = word_iter.next().unwrap().1.load::<u16>();
                                let msb = word_iter.next().unwrap().1.load::<u16>();
                                address.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                                address.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                                inst_vec.push((
                                    pc + start_addr,
                                    Instruction::MoveFieldAbsoluteToReg(
                                        Address(address),
                                        Rd((rf as u8) << 4 | rd),
                                        Some(F(f)),
                                    ),
                                    vec![word.load::<u16>(), lsb, msb],
                                ));
                            }
                            14 => {
                                let mut src_addr: u32 = 0;
                                let mut dst_addr: u32 = 0;
                                let src_lsb = word_iter.next().unwrap().1.load::<u16>();
                                let src_msb = word_iter.next().unwrap().1.load::<u16>();
                                let dst_lsb = word_iter.next().unwrap().1.load::<u16>();
                                let dst_msb = word_iter.next().unwrap().1.load::<u16>();
                                src_addr.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(src_lsb);
                                src_addr.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(src_msb);
                                dst_addr.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(dst_lsb);
                                dst_addr.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(dst_msb);
                                inst_vec.push((
                                    pc + start_addr,
                                    Instruction::MoveFieldAbsoluteToAbsolute(
                                        Address(src_addr),
                                        Address(dst_addr),
                                        Some(F(f)),
                                    ),
                                    vec![word.load::<u16>(), src_lsb, src_msb, dst_lsb, dst_msb],
                                ));
                            }
                            15 => {
                                if f {
                                    // this has nothing to do with fields I was just too lazy to
                                    // make an alias for bit 9
                                    let mut address: u32 = 0;
                                    let lsb = word_iter.next().unwrap().1.load::<u16>();
                                    let msb = word_iter.next().unwrap().1.load::<u16>();
                                    address.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                                    address.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                                    inst_vec.push((
                                        pc + start_addr,
                                        Instruction::MovbAbsoluteToReg(
                                            Address(address),
                                            Rd((rf as u8) << 4 | rd),
                                        ),
                                        vec![word.load::<u16>(), lsb, msb],
                                    ));
                                } else {
                                    let mut address: u32 = 0;
                                    let lsb = word_iter.next().unwrap().1.load::<u16>();
                                    let msb = word_iter.next().unwrap().1.load::<u16>();
                                    address.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                                    address.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                                    inst_vec.push((
                                        pc + start_addr,
                                        Instruction::MovbRegToAbsolute(
                                            Rs((rf as u8) << 4 | rd),
                                            Address(address),
                                        ),
                                        vec![word.load::<u16>(), lsb, msb],
                                    ));
                                }
                            }
                            _ => {}
                        }
                    }
                    0b0000100 => match subop {
                        8 => {
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Trap(N(n)),
                                vec![word.load::<u16>()],
                            ));
                        }
                        9 => {
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Call(Rs(rd)),
                                vec![word.load::<u16>()],
                            ));
                        }
                        10 => {
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Reti,
                                vec![word.load::<u16>()],
                            ));
                        }
                        11 => {
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Rets(N(n)),
                                vec![word.load::<u16>()],
                            ));
                        }
                        12 => {
                            let reglist = word_iter.next().unwrap().1.load::<u16>();
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Mmtm(Rd((rf as u8) << 4 | rd), RegList(reglist)),
                                vec![word.load::<u16>(), reglist],
                            ));
                        }
                        13 => {
                            let reglist = word_iter.next().unwrap().1.load::<u16>();
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Mmfm(Rs((rf as u8) << 4 | rd), RegList(reglist)),
                                vec![word.load::<u16>(), reglist],
                            ));
                        }
                        14 => {
                            let iw = word_iter.next().unwrap().1.load::<u16>();
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Moviw(IW(iw), Rd((rf as u8) << 4 | rd)),
                                vec![word.load::<u16>(), iw],
                            ));
                        }
                        15 => {
                            let mut il: u32 = 0;
                            let lsb = word_iter.next().unwrap().1.load::<u16>();
                            let msb = word_iter.next().unwrap().1.load::<u16>();
                            il.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                            il.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Movil(IL(il), Rd((rf as u8) << 4 | rd)),
                                vec![word.load::<u16>(), lsb, msb],
                            ));
                        }
                        _ => {}
                    },
                    0b0000101 => match subop {
                        8 => {
                            let iw = word_iter.next().unwrap().1.load::<u16>();
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Addiw(IW(iw), Rd((rf as u8) << 4 | rd)),
                                vec![word.load::<u16>(), iw],
                            ));
                        }
                        9 => {
                            let mut il: u32 = 0;
                            let lsb = word_iter.next().unwrap().1.load::<u16>();
                            let msb = word_iter.next().unwrap().1.load::<u16>();
                            il.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                            il.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Addil(IL(il), Rd((rf as u8) << 4 | rd)),
                                vec![word.load::<u16>(), lsb, msb],
                            ));
                        }
                        10 => {
                            let iw = word_iter.next().unwrap().1.load::<u16>();
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Cmpiw(IW(iw), Rd((rf as u8) << 4 | rd)),
                                vec![word.load::<u16>(), iw],
                            ));
                        }
                        11 => {
                            let mut il: u32 = 0;
                            let lsb = word_iter.next().unwrap().1.load::<u16>();
                            let msb = word_iter.next().unwrap().1.load::<u16>();
                            il.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                            il.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Cmpil(IL(il), Rd((rf as u8) << 4 | rd)),
                                vec![word.load::<u16>(), lsb, msb],
                            ));
                        }
                        12 => {
                            let mut il: u32 = 0;
                            let lsb = word_iter.next().unwrap().1.load::<u16>();
                            let msb = word_iter.next().unwrap().1.load::<u16>();
                            il.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                            il.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Andi(IL(il), Rd((rf as u8) << 4 | rd)),
                                vec![word.load::<u16>(), lsb, msb],
                            ));
                        }
                        13 => {
                            let mut il: u32 = 0;
                            let lsb = word_iter.next().unwrap().1.load::<u16>();
                            let msb = word_iter.next().unwrap().1.load::<u16>();
                            il.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                            il.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Ori(IL(il), Rd((rf as u8) << 4 | rd)),
                                vec![word.load::<u16>(), lsb, msb],
                            ));
                        }
                        14 => {
                            let mut il: u32 = 0;
                            let lsb = word_iter.next().unwrap().1.load::<u16>();
                            let msb = word_iter.next().unwrap().1.load::<u16>();
                            il.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                            il.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Xori(IL(il), Rd((rf as u8) << 4 | rd)),
                                vec![word.load::<u16>(), lsb, msb],
                            ));
                        }
                        15 => {
                            let iw = word_iter.next().unwrap().1.load::<u16>();
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Subiw(IW(iw), Rd((rf as u8) << 4 | rd)),
                                vec![word.load::<u16>(), iw],
                            ));
                        }
                        _ => {}
                    },
                    0b0000110 => match subop {
                        8 => {
                            let mut il: u32 = 0;
                            let lsb = word_iter.next().unwrap().1.load::<u16>();
                            let msb = word_iter.next().unwrap().1.load::<u16>();
                            il.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                            il.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Subil(IL(il), Rd((rf as u8) << 4 | rd)),
                                vec![word.load::<u16>(), lsb, msb],
                            ));
                        }
                        9 => {
                            let offset = word_iter.next().unwrap().1.load::<u16>();
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Callr(
                                    Offset(offset),
                                    PC(pc as u32 + start_addr as u32),
                                ),
                                vec![word.load::<u16>(), offset],
                            ));
                        }
                        10 => {
                            let mut address: u32 = 0;
                            let lsb = word_iter.next().unwrap().1.load::<u16>();
                            let msb = word_iter.next().unwrap().1.load::<u16>();
                            address.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                            address.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Calla(Address(address)),
                                vec![word.load::<u16>(), lsb, msb],
                            ));
                        }
                        11 => {
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Eint,
                                vec![word.load::<u16>()],
                            ));
                        }
                        12 => {
                            let offset = word_iter.next().unwrap().1.load::<u16>();
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Dsj(Rd((rf as u8) << 4 | rd), Offset(offset)),
                                vec![word.load::<u16>(), offset],
                            ));
                        }
                        13 => {
                            let offset = word_iter.next().unwrap().1.load::<u16>();
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Dsjeq(Rd((rf as u8) << 4 | rd), Offset(offset)),
                                vec![word.load::<u16>(), offset],
                            ));
                        }
                        14 => {
                            let offset = word_iter.next().unwrap().1.load::<u16>();
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Dsjne(Rd((rf as u8) << 4 | rd), Offset(offset)),
                                vec![word.load::<u16>(), offset],
                            ));
                        }
                        15 => {
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Setc,
                                vec![word.load::<u16>()],
                            ));
                        }
                        _ => {}
                    },
                    0b0000111 => match subop {
                        8 => {
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Pixbltll,
                                vec![word.load::<u16>()],
                            ));
                        }
                        9 => {
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Pixbltlxy,
                                vec![word.load::<u16>()],
                            ));
                        }
                        10 => {
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Pixbltxyl,
                                vec![word.load::<u16>()],
                            ));
                        }
                        11 => {
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Pixbltxyxy,
                                vec![word.load::<u16>()],
                            ));
                        }
                        12 => {
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Pixbltbl,
                                vec![word.load::<u16>()],
                            ));
                        }
                        13 => {
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Pixbltbxy,
                                vec![word.load::<u16>()],
                            ));
                        }
                        14 => {
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Filll,
                                vec![word.load::<u16>()],
                            ));
                        }
                        15 => {
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Fillxy,
                                vec![word.load::<u16>()],
                            ));
                        }
                        _ => {}
                    },
                    0b0001000 | 0b0001001 => {
                        if k == 1 {
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Inc(Rd((rf as u8) << 4 | rd)),
                                vec![word.load::<u16>()],
                            ));
                        } else {
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Addk(K(k), Rd((rf as u8) << 4 | rd)),
                                vec![word.load::<u16>()],
                            ));
                        }
                    }
                    0b0001010 | 0b0001011 => {
                        if k == 1 {
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Dec(Rd((rf as u8) << 4 | rd)),
                                vec![word.load::<u16>()],
                            ));
                        } else {
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Subk(K(k), Rd((rf as u8) << 4 | rd)),
                                vec![word.load::<u16>()],
                            ));
                        }
                    }
                    0b0001100 | 0b0001101 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Movk(K(k), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0001110 | 0b0001111 => {
                        // reminder to deal with 1's complement when formatting and assembling
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Btstk(K(k), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0010000 | 0b0010001 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Slak(K(k), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0010010 | 0b0010011 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Sllk(K(k), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0010100 | 0b0010101 => {
                        // reminder to deal with 2's complement when formatting and assembling
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Srak(K(k), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0010110 | 0b0010111 => {
                        // reminder to deal with 2's complement when formatting and assembling
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Srlk(K(k), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0011000 | 0b0011001 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Rlk(K(k), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0011100..=0b0011111 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Dsjs(D(d), Rd((rf as u8) << 4 | rd), K(k), PC(pc as u32 +start_addr as u32)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0100000 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Add(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0100001 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Addc(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0100010 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Sub(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0100011 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Subb(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0100100 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Cmp(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0100101 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Btst(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0100110 | 0b0100111 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::MoveReg(
                                Rs((rf as u8) << 4 | rs),
                                Rd(if f == false { (rf as u8) << 4 } else { 0 } | rd),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0101000 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::And(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0101001 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Andn(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0101010 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Or(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0101011 => {
                        if rs == rd {
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Clr(Rd((rf as u8) << 4 | rd)),
                                vec![word.load::<u16>()],
                            ));
                        } else {
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Xor(
                                    Rs((rf as u8) << 4 | rs),
                                    Rd((rf as u8) << 4 | rd),
                                ),
                                vec![word.load::<u16>()],
                            ));
                        }
                    }
                    0b0101100 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Divs(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0101101 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Divu(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0101110 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Mpys(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0101111 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Mpyu(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0110000 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Sla(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0110001 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Sll(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0110010 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Sra(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0110011 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Srl(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0110100 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Rl(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0110101 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Lmo(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0110110 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Mods(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0110111 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Modu(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1000000 | 0b1000001 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::MoveFieldRegToIndirect(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                                Some(F(f)),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1000010 | 0b1000011 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::MoveFieldIndirectToReg(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                                Some(F(f)),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1000100 | 0b1000101 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::MoveFieldIndirectToIndirect(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                                Some(F(f)),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1000110 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::MovbRegToIndirect(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1000111 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::MovbIndirectToReg(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1001000 | 0b1001001 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::MoveFieldRegToIndirectPostinc(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                                Some(F(f)),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1001010 | 0b1001011 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::MoveFieldIndirectPostincToReg(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                                Some(F(f)),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1001100 | 0b1001101 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::MoveFieldIndirectToIndirectPostinc(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                                Some(F(f)),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1001110 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::MovbIndirectToIndirect(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1010000 | 0b1010001 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::MoveFieldRegToIndirectPredec(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                                Some(F(f)),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1010010 | 0b1010011 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::MoveFieldIndirectPredecToReg(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                                Some(F(f)),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1010100 | 0b1010101 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::MoveFieldIndirectToIndirectPredec(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                                Some(F(f)),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1010110 => {
                        let offset = word_iter.next().unwrap().1.load::<u16>();
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::MovbRegToIndirectOffset(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                                Offset(offset),
                            ),
                            vec![word.load::<u16>(), offset],
                        ));
                    }
                    0b1010111 => {
                        let offset = word_iter.next().unwrap().1.load::<u16>();
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::MovbIndirectOffsetToReg(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                                Offset(offset),
                            ),
                            vec![word.load::<u16>(), offset],
                        ));
                    }
                    0b1011000 | 0b1011001 => {
                        let offset = word_iter.next().unwrap().1.load::<u16>();
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::MoveFieldRegToIndirectOffset(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                                Some(F(f)),
                                Offset(offset),
                            ),
                            vec![word.load::<u16>(), offset],
                        ));
                    }
                    0b1011010 | 0b1011011 => {
                        let offset = word_iter.next().unwrap().1.load::<u16>();
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::MoveFieldIndirectOffsetToReg(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                                Some(F(f)),
                                Offset(offset),
                            ),
                            vec![word.load::<u16>(), offset],
                        ));
                    }
                    0b1011100 | 0b1011101 => {
                        let src_offset = word_iter.next().unwrap().1.load::<u16>();
                        let dst_offset = word_iter.next().unwrap().1.load::<u16>();
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::MoveFieldIndirectOffsetToIndirectOffset(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                                Some(F(f)),
                                Offset(src_offset),
                                Offset(dst_offset),
                            ),
                            vec![word.load::<u16>(), src_offset, dst_offset],
                        ));
                    }
                    0b1011110 => {
                        let src_offset = word_iter.next().unwrap().1.load::<u16>();
                        let dst_offset = word_iter.next().unwrap().1.load::<u16>();
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::MovbIndirectOffsetToIndirectOffset(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                                Offset(src_offset),
                                Offset(dst_offset),
                            ),
                            vec![word.load::<u16>(), src_offset, dst_offset],
                        ));
                    }
                    0b1100000..=0b1100111 => {
                        let lower8 = word.get(0..=7).unwrap().load::<u8>();
                        if lower8 == 0x80 {
                            let mut address: u32 = 0;
                            let lsb = word_iter.next().unwrap().1.load::<u16>();
                            let msb = word_iter.next().unwrap().1.load::<u16>();
                            address.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                            address.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Ja(Condition(cc), Address(address)),
                                vec![word.load::<u16>(), lsb, msb],
                            ));
                        } else if lower8 == 0x00 {
                            let offset = word_iter.next().unwrap().1.load::<u16>();
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Jrs(Condition(cc), Offset(offset)),
                                vec![word.load::<u16>(), offset],
                            ));
                        } else {
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Jr(
                                    Condition(cc),
                                    Offset8(lower8),
                                    PC(pc as u32 + start_addr as u32),
                                ),
                                vec![word.load::<u16>()],
                            ));
                        }
                    }
                    0b1101000 | 0b1101001 => {
                        let offset = word_iter.next().unwrap().1.load::<u16>();
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::MoveFieldIndirectOffsetToIndirectPostinc(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                                Some(F(f)),
                                Offset(offset),
                            ),
                            vec![word.load::<u16>(), offset],
                        ));
                    }
                    0b1101010 | 0b1101011 => match subop {
                        0 => {
                            let mut address: u32 = 0;
                            let lsb = word_iter.next().unwrap().1.load::<u16>();
                            let msb = word_iter.next().unwrap().1.load::<u16>();
                            address.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                            address.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::MoveFieldAbsoluteToIndirectPostinc(
                                    Address(address),
                                    Rd((rf as u8) << 4 | rd),
                                    Some(F(f)),
                                ),
                                vec![word.load::<u16>(), lsb, msb],
                            ));
                        }
                        8 => {
                            inst_vec.push((
                                pc + start_addr,
                                Instruction::Exgf(Rd((rf as u8) << 4 | rd), Some(F(f))),
                                vec![word.load::<u16>()],
                            ));
                        }
                        _ => {}
                    },
                    0b1101111 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Line(Z(z)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1110000 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Addxy(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1110001 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Subxy(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1110010 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Cmpxy(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1110011 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Cpw(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1110100 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Cvxyl(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1110110 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Movx(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1110111 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Movy(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1111000 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::PixtRegToIndirectxy(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1111001 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::PixtIndirectxyToReg(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1111010 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::PixtIndirectxytoIndirectxy(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1111011 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::Drav(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1111100 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::PixtRegToIndirect(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1111101 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::PixtIndirectToReg(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1111110 => {
                        inst_vec.push((
                            pc + start_addr,
                            Instruction::PixtIndirectToIndirect(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    _ => {}
                }
            }
            None => {
                break;
            }
        }
    }
    inst_vec
}

impl fmt::Display for Instruction {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Instruction::Setf(fs, fe, Some(f)) => {
                write!(fmt, "SETF      {},{},{}", fs, fe, f)
            }
            Instruction::Movk(k, rd) => {
                let special_k = if  k.0 == 0 { K(32) } else { *k };
                write!(fmt, "MOVK      {},{}", special_k, rd)
            }
            Instruction::MoveFieldRegToAbsolute(rs, addr, Some(f)) => {
                write!(fmt, "MOVE      {},@{},{}", rs, addr, f)
            }
            Instruction::Moviw(iw, rd) => {
                write!(fmt, "MOVI      {},{}", iw, rd)
            }
            Instruction::Movil(il, rd) => {
                write!(fmt, "MOVI      {},{}", il, rd)
            }
            Instruction::MoveReg(rs, rd) => {
                write!(fmt, "MOVE      {},{}", rs, rd)
            }
            Instruction::Lmo(rs, rd) => {
                write!(fmt, "LMO       {},{}", rs, rd)
            }
            Instruction::Callr(offset, pc) => {
                write!(fmt, "CALLR     {:X}h", pc.0 * 16 + offset.0 as u32 * 16 + 32)
            }
            Instruction::Trap(n) => {
                write!(fmt, "TRAP      {}", n)
            }
            Instruction::Calla(addr) => {
                write!(fmt, "CALLA     {}", addr)
            }
            Instruction::Jr(condition, off8, pc) => {
                write!(fmt, "JR{}{}      {:X}h",
                    condition,
                    if condition.to_string().len() == 1 { " " } else { "" },
                    (pc.0 + (off8.0 as i8) as u32) * 16 + 16)
            }
            Instruction::Pixbltbxy => {
                write!(fmt, "PIXBLT    B,XY")
            }
            Instruction::Clr(rd) => {
                write!(fmt, "CLR       {}", rd)
            }
            Instruction::Rets(n) => {
                write!(fmt, "RETS      {}",
                    if n.0 > 0 {
                        n.to_string()
                    } else {
                        String::new()
                    }
                )
            }
            Instruction::Fillxy => {
                write!(fmt, "FILL      XY")
            }
            Instruction::PixtRegToIndirectxy(rs, rd) => {
                write!(fmt, "PIXT      {},*{},XY", rs, rd)
            }
            Instruction::PixtIndirectxyToReg(rs, rd) => {
                write!(fmt, "PIXT      *{},XY,{}", rs, rd)
            }
            Instruction::PixtRegToIndirect(rs, rd) => {
                write!(fmt, "PIXT      {},*{}", rs, rd)
            }
            Instruction::PixtIndirectxytoIndirectxy(rs, rd) => {
                write!(fmt, "PIXT      *{},XY,*{},XY", rs, rd)
            }
            Instruction::Drav(rs, rd) => {
                write!(fmt, "DRAV      {},{}", rs, rd)
            }
            Instruction::Dsjs(d, rd, k, pc) => {
                if d.0 {
                    write!(fmt, "DSJS      {},{:X}h", rd, (pc.0-k.0 as u32)*16+16 )
                }
                else {
                    write!(fmt, "DSJS      {},{:X}h", rd, (pc.0+k.0 as u32)*16+16 )
                }
            }
            Instruction::MoveFieldRegToIndirectPredec(rs, rd, Some(f)) => {
                write!(fmt, "MOVE      {},-*{},{}", rs, rd, f) 
            }
            Instruction::Pixbltbl => {
                write!(fmt, "PIXBLT    B,L")
            }
            Instruction::MoveFieldIndirectPostincToReg(rs, rd, Some(f)) => {
                write!(fmt, "MOVE      *{}+,{},{}", rs, rd, f)
            }
            Instruction::Pixbltlxy => {
                write!(fmt, "PIXBLT    L,XY")
            }
            Instruction::Pixbltxyxy => {
                write!(fmt, "PIXBLT    XY,XY")
            }
            Instruction::Pixbltxyl => {
                write!(fmt, "PIXBLT    XY,L")
            }
            Instruction::Pixbltll => {
                write!(fmt, "PIXBLT    L,L")
            }
            Instruction::MoveFieldAbsoluteToReg(addr, rd, Some(f)) =>
            {
                write!(fmt, "MOVE      @{},{},{}", addr, rd, f)
            }
            Instruction::Ori(il, rd) => {
                write!(fmt, "ORI       {},{}", il, rd) 
            }
            Instruction::Andi(il, rd) => {
                let ones_comp = IL(!il.0);
                write!(fmt, "ANDI      {},{}", ones_comp, rd) 
            }
            Instruction::Cmp(rs, rd) => {
                write!(fmt, "CMP       {},{}", rs, rd)
            }
            Instruction::And(rs, rd) => {
                write!(fmt, "AND       {},{}", rs, rd) 
            }
            Instruction::Or(rs, rd) => {
                write!(fmt, "OR        {},{}", rs, rd)
            }
            Instruction::Inc(rd) => {
                write!(fmt, "INC       {}", rd)
            }
            Instruction::Subxy(rs, rd) => {
                write!(fmt, "SUBXY     {},{}", rs, rd)
            }
            Instruction::Sllk(k, rd) => {
                write!(fmt, "SLL       {},{}", k, rd)
            }
            Instruction::Addil(il, rd) => {
                write!(fmt, "ADDIL     {},{}", il, rd) 
            }
            Instruction::Filll => {
                write!(fmt, "FILL      L")
            }
            _ => write!(fmt, "")
        }
    }
}

pub fn disassemble_stage2(stage1_output: Vec<(usize, Instruction, Vec<u16>)>) -> String {
    let mut disassembly = String::new();

    for (pc, inst, words) in stage1_output {
        let mut hexdump = String::new();
        write!(hexdump, "{:04X?}", words).unwrap();
        hexdump = hexdump.replace("[", "");
        hexdump = hexdump.replace("]", "");
        hexdump = hexdump.replace(",", " ");

        writeln!(
            disassembly,
            "{:08X}:\t{}{}{}",
            pc * 16,
            inst,
            " ".repeat(32 - inst.to_string().len()),
            hexdump
        )
        .unwrap();
    }
    disassembly
}

pub fn disassemble(bytebuf: &[u8], start_addr: usize) {
    println!(
        "{}",
        disassemble_stage2(disassemble_stage1(bytebuf, start_addr))
    );
}
