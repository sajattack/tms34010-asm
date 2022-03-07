use bitvec::prelude::*;

use crate::instruction::Instruction;
use core::fmt::{self, Formatter, Write};

use crate::symbol::{
    Address, Condition, Offset, Offset8, Rd, RegList, Rs, D, F, FE, FS, IL, IW, K, N, PC, Z,
};

pub fn disassemble_stage1(bytebuf: &[u8], start_addr: usize) -> Vec<Dis> {
    let mut inst_vec = vec![];
    let bitslice = bytebuf.view_bits::<Lsb0>();
    let mut word_iter = bitslice.chunks(16).enumerate();
    loop {
        match word_iter.next() {
            Some((pc, word)) => {
                let mut bad = false;
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
                                inst_vec.push(Dis(
                                    pc + start_addr,
                                    Instruction::Rev(Rd((rf as u8) << 4 | rd)),
                                    vec![word.load::<u16>()],
                                ));
                            }
                            8 => {
                                inst_vec.push(Dis(
                                    pc + start_addr,
                                    Instruction::Emu,
                                    vec![word.load::<u16>()],
                                ));
                            }
                            9 => {
                                inst_vec.push(Dis(
                                    pc + start_addr,
                                    Instruction::Exgpc(Rd((rf as u8) << 4 | rd), F(f)),
                                    vec![word.load::<u16>()],
                                ));
                            }
                            10 => {
                                inst_vec.push(Dis(
                                    pc + start_addr,
                                    Instruction::Getpc(Rd((rf as u8) << 4 | rd)),
                                    vec![word.load::<u16>()],
                                ));
                            }
                            11 => {
                                // this is kind of strange that we assign rd to Rs, but it's in the
                                // position of rd, and the manual calls it Rs because it's the
                                // source of the argument to jump to
                                inst_vec.push(Dis(
                                    pc + start_addr,
                                    Instruction::Jump(Rs((rf as u8) << 4 | rd)),
                                    vec![word.load::<u16>()],
                                ));
                            }
                            12 => {
                                inst_vec.push(Dis(
                                    pc + start_addr,
                                    Instruction::Getst(Rd((rf as u8) << 4 | rd)),
                                    vec![word.load::<u16>()],
                                ));
                            }
                            13 => {
                                inst_vec.push(Dis(
                                    pc + start_addr,
                                    Instruction::Putst(Rs((rf as u8) << 4 | rd)),
                                    vec![word.load::<u16>()],
                                ));
                            }
                            14 => {
                                inst_vec.push(Dis(
                                    pc + start_addr,
                                    Instruction::Popst,
                                    vec![word.load::<u16>()],
                                ));
                            }
                            15 => {
                                inst_vec.push(Dis(
                                    pc + start_addr,
                                    Instruction::Pushst,
                                    vec![word.load::<u16>()],
                                ));
                            }
                            _ => {
                                bad = true;
                            }
                        }
                    }
                    0b0000001 => match subop {
                        8 => {
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Nop,
                                vec![word.load::<u16>()],
                            ));
                        }
                        9 => {
                            inst_vec.push(Dis(
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
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::MovbAbsoluteToAbsolute(
                                    Address(src_addr),
                                    Address(dst_addr),
                                ),
                                vec![word.load::<u16>(), src_lsb, src_msb, dst_lsb, dst_msb],
                            ));
                        }
                        11 => {
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Dint,
                                vec![word.load::<u16>()],
                            ));
                        }
                        12 => {
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Abs(Rd((rf as u8) << 4 | rd)),
                                vec![word.load::<u16>()],
                            ));
                        }
                        13 => {
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Neg(Rd((rf as u8) << 4 | rd)),
                                vec![word.load::<u16>()],
                            ));
                        }
                        14 => {
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Negb(Rd((rf as u8) << 4 | rd)),
                                vec![word.load::<u16>()],
                            ));
                        }
                        15 => {
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Not(Rd((rf as u8) << 4 | rd)),
                                vec![word.load::<u16>()],
                            ));
                        }
                        _ => {
                            bad = true;
                        }
                    },
                    0b0000010 | 0b0000011 => {
                        match subop {
                            8 => {
                                inst_vec.push(Dis(
                                    pc + start_addr,
                                    Instruction::Sext(Rd((rf as u8) << 4 | rd), F(f)),
                                    vec![word.load::<u16>()],
                                ));
                            }
                            9 => {
                                inst_vec.push(Dis(
                                    pc + start_addr,
                                    Instruction::Zext(Rd((rf as u8) << 4 | rd), F(f)),
                                    vec![word.load::<u16>()],
                                ));
                            }
                            10 | 11 => {
                                inst_vec.push(Dis(
                                    pc + start_addr,
                                    Instruction::Setf(FS(fs), FE(fe), F(f)),
                                    vec![word.load::<u16>()],
                                ));
                            }
                            12 => {
                                let mut address: u32 = 0;
                                let lsb = word_iter.next().unwrap().1.load::<u16>();
                                let msb = word_iter.next().unwrap().1.load::<u16>();
                                address.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                                address.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                                inst_vec.push(Dis(
                                    pc + start_addr,
                                    Instruction::MoveFieldRegToAbsolute(
                                        Rs((rf as u8) << 4 | rd),
                                        Address(address),
                                        F(f),
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
                                inst_vec.push(Dis(
                                    pc + start_addr,
                                    Instruction::MoveFieldAbsoluteToReg(
                                        Address(address),
                                        Rd((rf as u8) << 4 | rd),
                                        F(f),
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
                                inst_vec.push(Dis(
                                    pc + start_addr,
                                    Instruction::MoveFieldAbsoluteToAbsolute(
                                        Address(src_addr),
                                        Address(dst_addr),
                                        F(f),
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
                                    inst_vec.push(Dis(
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
                                    inst_vec.push(Dis(
                                        pc + start_addr,
                                        Instruction::MovbRegToAbsolute(
                                            Rs((rf as u8) << 4 | rd),
                                            Address(address),
                                        ),
                                        vec![word.load::<u16>(), lsb, msb],
                                    ));
                                }
                            }
                            _ => {
                                bad = true;
                            }
                        }
                    }
                    0b0000100 => match subop {
                        8 => {
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Trap(N(n)),
                                vec![word.load::<u16>()],
                            ));
                        }
                        9 => {
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Call(Rs(rd)),
                                vec![word.load::<u16>()],
                            ));
                        }
                        10 => {
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Reti,
                                vec![word.load::<u16>()],
                            ));
                        }
                        11 => {
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Rets(N(n)),
                                vec![word.load::<u16>()],
                            ));
                        }
                        12 => {
                            let reglist = word_iter.next().unwrap().1.load::<u16>();
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Mmtm(Rd((rf as u8) << 4 | rd), RegList(reglist)),
                                vec![word.load::<u16>(), reglist],
                            ));
                        }
                        13 => {
                            let reglist = word_iter.next().unwrap().1.load::<u16>();
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Mmfm(Rs((rf as u8) << 4 | rd), RegList(reglist)),
                                vec![word.load::<u16>(), reglist],
                            ));
                        }
                        14 => {
                            let iw = word_iter.next().unwrap().1.load::<u16>();
                            inst_vec.push(Dis(
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
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Movil(IL(il), Rd((rf as u8) << 4 | rd)),
                                vec![word.load::<u16>(), lsb, msb],
                            ));
                        }
                        _ => {
                            bad = true;
                        }
                    },
                    0b0000101 => match subop {
                        8 => {
                            let iw = word_iter.next().unwrap().1.load::<u16>();
                            inst_vec.push(Dis(
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
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Addil(IL(il), Rd((rf as u8) << 4 | rd)),
                                vec![word.load::<u16>(), lsb, msb],
                            ));
                        }
                        10 => {
                            let iw = word_iter.next().unwrap().1.load::<u16>();
                            inst_vec.push(Dis(
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
                            inst_vec.push(Dis(
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
                            inst_vec.push(Dis(
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
                            inst_vec.push(Dis(
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
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Xori(IL(il), Rd((rf as u8) << 4 | rd)),
                                vec![word.load::<u16>(), lsb, msb],
                            ));
                        }
                        15 => {
                            let iw = word_iter.next().unwrap().1.load::<u16>();
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Subiw(IW(iw), Rd((rf as u8) << 4 | rd)),
                                vec![word.load::<u16>(), iw],
                            ));
                        }
                        _ => {
                            bad = true;
                        }
                    },
                    0b0000110 => match subop {
                        8 => {
                            let mut il: u32 = 0;
                            let lsb = word_iter.next().unwrap().1.load::<u16>();
                            let msb = word_iter.next().unwrap().1.load::<u16>();
                            il.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                            il.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Subil(IL(il), Rd((rf as u8) << 4 | rd)),
                                vec![word.load::<u16>(), lsb, msb],
                            ));
                        }
                        9 => {
                            let offset = word_iter.next().unwrap().1.load::<u16>();
                            inst_vec.push(Dis(
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
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Calla(Address(address)),
                                vec![word.load::<u16>(), lsb, msb],
                            ));
                        }
                        11 => {
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Eint,
                                vec![word.load::<u16>()],
                            ));
                        }
                        12 => {
                            let offset = word_iter.next().unwrap().1.load::<u16>();
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Dsj(Rd((rf as u8) << 4 | rd), Offset(offset)),
                                vec![word.load::<u16>(), offset],
                            ));
                        }
                        13 => {
                            let offset = word_iter.next().unwrap().1.load::<u16>();
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Dsjeq(Rd((rf as u8) << 4 | rd), Offset(offset)),
                                vec![word.load::<u16>(), offset],
                            ));
                        }
                        14 => {
                            let offset = word_iter.next().unwrap().1.load::<u16>();
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Dsjne(Rd((rf as u8) << 4 | rd), Offset(offset)),
                                vec![word.load::<u16>(), offset],
                            ));
                        }
                        15 => {
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Setc,
                                vec![word.load::<u16>()],
                            ));
                        }
                        _ => {
                            bad = true;
                        }
                    },
                    0b0000111 => match subop {
                        8 => {
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Pixbltll,
                                vec![word.load::<u16>()],
                            ));
                        }
                        9 => {
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Pixbltlxy,
                                vec![word.load::<u16>()],
                            ));
                        }
                        10 => {
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Pixbltxyl,
                                vec![word.load::<u16>()],
                            ));
                        }
                        11 => {
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Pixbltxyxy,
                                vec![word.load::<u16>()],
                            ));
                        }
                        12 => {
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Pixbltbl,
                                vec![word.load::<u16>()],
                            ));
                        }
                        13 => {
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Pixbltbxy,
                                vec![word.load::<u16>()],
                            ));
                        }
                        14 => {
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Filll,
                                vec![word.load::<u16>()],
                            ));
                        }
                        15 => {
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Fillxy,
                                vec![word.load::<u16>()],
                            ));
                        }
                        _ => {
                            bad = true;
                        }
                    },
                    0b0001000 | 0b0001001 => {
                        if k == 1 {
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Inc(Rd((rf as u8) << 4 | rd)),
                                vec![word.load::<u16>()],
                            ));
                        } else {
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Addk(K(k), Rd((rf as u8) << 4 | rd)),
                                vec![word.load::<u16>()],
                            ));
                        }
                    }
                    0b0001010 | 0b0001011 => {
                        if k == 1 {
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Dec(Rd((rf as u8) << 4 | rd)),
                                vec![word.load::<u16>()],
                            ));
                        } else {
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Subk(K(k), Rd((rf as u8) << 4 | rd)),
                                vec![word.load::<u16>()],
                            ));
                        }
                    }
                    0b0001100 | 0b0001101 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Movk(K(k), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0001110 | 0b0001111 => {
                        // reminder to deal with 1's complement when formatting and assembling
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Btstk(K(k), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0010000 | 0b0010001 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Slak(K(k), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0010010 | 0b0010011 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Sllk(K(k), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0010100 | 0b0010101 => {
                        // reminder to deal with 2's complement when formatting and assembling
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Srak(K(k), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0010110 | 0b0010111 => {
                        // reminder to deal with 2's complement when formatting and assembling
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Srlk(K(k), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0011000 | 0b0011001 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Rlk(K(k), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0011100..=0b0011111 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Dsjs(
                                D(d),
                                Rd((rf as u8) << 4 | rd),
                                K(k),
                                PC(pc as u32 + start_addr as u32),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0100000 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Add(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0100001 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Addc(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0100010 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Sub(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0100011 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Subb(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0100100 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Cmp(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0100101 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Btst(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0100110 | 0b0100111 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::MoveReg(
                                Rs((rf as u8) << 4 | rs),
                                Rd(if f == false { (rf as u8) << 4 } else { 0 } | rd),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0101000 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::And(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0101001 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Andn(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0101010 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Or(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0101011 => {
                        if rs == rd {
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Clr(Rd((rf as u8) << 4 | rd)),
                                vec![word.load::<u16>()],
                            ));
                        } else {
                            inst_vec.push(Dis(
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
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Divs(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0101101 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Divu(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0101110 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Mpys(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0101111 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Mpyu(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0110000 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Sla(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0110001 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Sll(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0110010 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Sra(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0110011 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Srl(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0110100 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Rl(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0110101 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Lmo(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0110110 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Mods(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b0110111 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Modu(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1000000 | 0b1000001 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::MoveFieldRegToIndirect(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                                F(f),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1000010 | 0b1000011 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::MoveFieldIndirectToReg(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                                F(f),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1000100 | 0b1000101 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::MoveFieldIndirectToIndirect(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                                F(f),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1000110 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::MovbRegToIndirect(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1000111 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::MovbIndirectToReg(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1001000 | 0b1001001 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::MoveFieldRegToIndirectPostinc(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                                F(f),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1001010 | 0b1001011 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::MoveFieldIndirectPostincToReg(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                                F(f),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1001100 | 0b1001101 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::MoveFieldIndirectToIndirectPostinc(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                                F(f),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1001110 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::MovbIndirectToIndirect(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1010000 | 0b1010001 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::MoveFieldRegToIndirectPredec(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                                F(f),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1010010 | 0b1010011 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::MoveFieldIndirectPredecToReg(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                                F(f),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1010100 | 0b1010101 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::MoveFieldIndirectToIndirectPredec(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                                F(f),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1010110 => {
                        let offset = word_iter.next().unwrap().1.load::<u16>();
                        inst_vec.push(Dis(
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
                        inst_vec.push(Dis(
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
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::MoveFieldRegToIndirectOffset(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                                F(f),
                                Offset(offset),
                            ),
                            vec![word.load::<u16>(), offset],
                        ));
                    }
                    0b1011010 | 0b1011011 => {
                        let offset = word_iter.next().unwrap().1.load::<u16>();
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::MoveFieldIndirectOffsetToReg(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                                F(f),
                                Offset(offset),
                            ),
                            vec![word.load::<u16>(), offset],
                        ));
                    }
                    0b1011100 | 0b1011101 => {
                        let src_offset = word_iter.next().unwrap().1.load::<u16>();
                        let dst_offset = word_iter.next().unwrap().1.load::<u16>();
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::MoveFieldIndirectOffsetToIndirectOffset(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                                F(f),
                                Offset(src_offset),
                                Offset(dst_offset),
                            ),
                            vec![word.load::<u16>(), src_offset, dst_offset],
                        ));
                    }
                    0b1011110 => {
                        let src_offset = word_iter.next().unwrap().1.load::<u16>();
                        let dst_offset = word_iter.next().unwrap().1.load::<u16>();
                        inst_vec.push(Dis(
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
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Ja(Condition(cc), Address(address)),
                                vec![word.load::<u16>(), lsb, msb],
                            ));
                        } else if lower8 == 0x00 {
                            let offset = word_iter.next().unwrap().1.load::<u16>();
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Jr(
                                    Condition(cc),
                                    Offset(offset),
                                    PC(pc as u32 + start_addr as u32),
                                ),
                                vec![word.load::<u16>(), offset],
                            ));
                        } else {
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Jrs(
                                    Condition(cc),
                                    Offset8(lower8),
                                    PC((pc + start_addr) as u32),
                                ),
                                vec![word.load::<u16>()],
                            ));
                        }
                    }
                    0b1101000 | 0b1101001 => {
                        let offset = word_iter.next().unwrap().1.load::<u16>();
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::MoveFieldIndirectOffsetToIndirectPostinc(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                                F(f),
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
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::MoveFieldAbsoluteToIndirectPostinc(
                                    Address(address),
                                    Rd((rf as u8) << 4 | rd),
                                    F(f),
                                ),
                                vec![word.load::<u16>(), lsb, msb],
                            ));
                        }
                        8 => {
                            inst_vec.push(Dis(
                                pc + start_addr,
                                Instruction::Exgf(Rd((rf as u8) << 4 | rd), F(f)),
                                vec![word.load::<u16>()],
                            ));
                        }
                        _ => {
                            bad = true;
                        }
                    },
                    0b1101111 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Line(Z(z)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1110000 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Addxy(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1110001 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Subxy(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1110010 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Cmpxy(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1110011 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Cpw(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1110100 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Cvxyl(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1110110 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Movx(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1110111 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Movy(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1111000 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::PixtRegToIndirectxy(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1111001 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::PixtIndirectxyToReg(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1111010 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::PixtIndirectxyToIndirectxy(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1111011 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::Drav(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1111100 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::PixtRegToIndirect(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1111101 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::PixtIndirectToReg(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    0b1111110 => {
                        inst_vec.push(Dis(
                            pc + start_addr,
                            Instruction::PixtIndirectToIndirect(
                                Rs((rf as u8) << 4 | rs),
                                Rd((rf as u8) << 4 | rd),
                            ),
                            vec![word.load::<u16>()],
                        ));
                    }
                    _ => {
                        bad = true;
                    }
                }
                if bad == true {
                    inst_vec.push(Dis(
                        pc + start_addr,
                        Instruction::Dw(IW(word.load::<u16>())),
                        vec![word.load::<u16>()],
                    ));
                }
            }
            None => {
                break;
            }
        }
    }
    inst_vec
}

#[derive(Debug, PartialEq)]
pub struct Dis(pub usize, pub Instruction, pub Vec<u16>);

impl fmt::Display for Instruction {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Instruction::Lmo(rs, rd)
            | Instruction::Xor(rs, rd)
            | Instruction::Cmp(rs, rd)
            | Instruction::Or(rs, rd)
            | Instruction::Sub(rs, rd)
            | Instruction::Subb(rs, rd)
            | Instruction::Subxy(rs, rd)
            | Instruction::Add(rs, rd)
            | Instruction::Addc(rs, rd)
            | Instruction::Addxy(rs, rd)
            | Instruction::And(rs, rd)
            | Instruction::Andn(rs, rd)
            | Instruction::Btst(rs, rd)
            | Instruction::Cmpxy(rs, rd)
            | Instruction::Divs(rs, rd)
            | Instruction::Divu(rs, rd)
            | Instruction::Mods(rs, rd)
            | Instruction::Modu(rs, rd)
            | Instruction::Mpys(rs, rd)
            | Instruction::Mpyu(rs, rd)
            | Instruction::MoveReg(rs, rd)
            | Instruction::Movx(rs, rd)
            | Instruction::Movy(rs, rd)
            | Instruction::Cpw(rs, rd)
            | Instruction::Cvxyl(rs, rd)
            | Instruction::Rl(rs, rd)
            | Instruction::Sla(rs, rd)
            | Instruction::Sll(rs, rd)
            | Instruction::Sra(rs, rd)
            | Instruction::Srl(rs, rd)
            | Instruction::Drav(rs, rd) => {
                write!(fmt, "{} {}, {}", self.get_mnemonic(), rs, rd)
            }
            Instruction::Pixbltbxy
            | Instruction::Pixbltbl
            | Instruction::Pixbltlxy
            | Instruction::Pixbltxyxy
            | Instruction::Pixbltxyl
            | Instruction::Pixbltll
            | Instruction::Fillxy
            | Instruction::Filll
            | Instruction::Emu
            | Instruction::Popst
            | Instruction::Pushst
            | Instruction::Nop
            | Instruction::Clrc
            | Instruction::Setc
            | Instruction::Dint
            | Instruction::Eint
            | Instruction::Reti => {
                write!(fmt, "{}", self.get_mnemonic())
            }
            Instruction::Getpc(rd)
            | Instruction::Getst(rd)
            | Instruction::Neg(rd)
            | Instruction::Negb(rd)
            | Instruction::Not(rd)
            | Instruction::Inc(rd)
            | Instruction::Dec(rd)
            | Instruction::Abs(rd)
            | Instruction::Rev(rd)
            | Instruction::Clr(rd) => {
                write!(fmt, "{} {}", self.get_mnemonic(), rd)
            }
            Instruction::Jump(rs) | Instruction::Call(rs) | Instruction::Putst(rs) => {
                write!(fmt, "{} {}", self.get_mnemonic(), rs)
            }
            Instruction::Moviw(iw, rd)
            | Instruction::Subiw(iw, rd)
            | Instruction::Addiw(iw, rd) => {
                write!(fmt, "{} {}, {}", self.get_mnemonic(), iw, rd)
            }
            Instruction::Addil(il, rd)
            | Instruction::Subil(il, rd)
            | Instruction::Movil(il, rd)
            | Instruction::Xori(il, rd)
            | Instruction::Ori(il, rd) => {
                write!(fmt, "{} {}, {}", self.get_mnemonic(), il, rd)
            }
            Instruction::Setf(fs, fe, f) => {
                write!(fmt, "{} {}, {}, {}", self.get_mnemonic(), fs, fe, f)
            }
            Instruction::Sext(rd, f) | Instruction::Zext(rd, f) | Instruction::Exgf(rd, f) => {
                write!(fmt, "{} {}, {}", self.get_mnemonic(), rd, f)
            }
            Instruction::Trap(n) => {
                write!(fmt, "{} {}", self.get_mnemonic(), n)
            }
            Instruction::Calla(addr) => {
                write!(fmt, "{} {}", self.get_mnemonic(), addr)
            }
            Instruction::Callr(offset, pc) => {
                write!(
                    fmt,
                    "{} {:X}h",
                    self.get_mnemonic(),
                    ((pc.0 as i64 + offset.0 as i16 as i64) * 16) as u32 + 32
                )
            }
            Instruction::Jrs(condition, off8, pc) => {
                write!(
                    fmt,
                    "{}{} {:X}h",
                    self.get_mnemonic(),
                    condition,
                    (((pc.0 as i64 + off8.0 as i8 as i64) * 16) + 16) as u32
                )
            }
            Instruction::Ja(condition, address) => {
                write!(
                    fmt,
                    "{}{}, {:X}h",
                    self.get_mnemonic(),
                    condition,
                    address.0 as u32
                )
            }
            Instruction::Rets(n) => {
                write!(
                    fmt,
                    "{} {}",
                    self.get_mnemonic(),
                    if n.0 > 0 {
                        n.to_string()
                    } else {
                        String::new()
                    }
                )
            }
            Instruction::PixtRegToIndirect(rs, rd) => {
                write!(fmt, "{} {}, *{}", self.get_mnemonic(), rs, rd)
            }
            Instruction::PixtRegToIndirectxy(rs, rd) => {
                write!(fmt, "{} {}, *{}, XY", self.get_mnemonic(), rs, rd)
            }
            Instruction::PixtIndirectToReg(rs, rd) => {
                write!(fmt, "{} *{}, {}", self.get_mnemonic(), rs, rd)
            }
            Instruction::PixtIndirectToIndirect(rs, rd) => {
                write!(fmt, "{} *{}, *{}", self.get_mnemonic(), rs, rd)
            }
            Instruction::PixtIndirectxyToReg(rs, rd) => {
                write!(fmt, "{} *{}, XY, {}", self.get_mnemonic(), rs, rd)
            }
            Instruction::PixtIndirectxyToIndirectxy(rs, rd) => {
                write!(fmt, "{} *{} ,XY, *{}, XY", self.get_mnemonic(), rs, rd)
            }
            Instruction::Dsjs(d, rd, k, pc) => {
                if d.0 {
                    write!(
                        fmt,
                        "{} {}, {:X}h",
                        self.get_mnemonic(),
                        rd,
                        (pc.0 - k.0 as u32) * 16 + 16
                    )
                } else {
                    write!(
                        fmt,
                        "{} {}, {:X}h",
                        self.get_mnemonic(),
                        rd,
                        (pc.0 + k.0 as u32) * 16 + 16
                    )
                }
            }
            Instruction::Cmpil(il, rd) => {
                write!(fmt, "{} {:08X}h, {}", self.get_mnemonic(), !il.0, rd)
            }
            Instruction::MovbRegToIndirect(rs, rd) => {
                write!(fmt, "{} {}, *{}", self.get_mnemonic(), rs, rd)
            }
            Instruction::MovbIndirectToReg(rs, rd) => {
                write!(fmt, "{} *{}, {}", self.get_mnemonic(), rs, rd)
            }
            Instruction::MovbIndirectToIndirect(rs, rd) => {
                write!(fmt, "{} *{}, *{}", self.get_mnemonic(), rs, rd)
            }
            Instruction::MovbRegToIndirectOffset(rs, rd, offset) => {
                write!(
                    fmt,
                    "{} {}, *{}({})",
                    self.get_mnemonic(),
                    rs,
                    rd,
                    offset.0 as i16
                )
            }
            Instruction::MovbIndirectOffsetToReg(rs, rd, offset) => {
                write!(
                    fmt,
                    "{} *{}({}), {}",
                    self.get_mnemonic(),
                    rs,
                    rd,
                    offset.0 as i16
                )
            }
            Instruction::MovbIndirectOffsetToIndirectOffset(rs, rd, offset, offset2) => {
                write!(
                    fmt,
                    "{} *{}({}), *{}({})",
                    self.get_mnemonic(),
                    rs,
                    rd,
                    offset.0 as i16,
                    offset2.0 as i16
                )
            }
            Instruction::MovbAbsoluteToReg(addr, rd) => {
                write!(fmt, "{} @{}, {}", self.get_mnemonic(), addr, rd)
            }
            Instruction::MovbRegToAbsolute(rs, addr) => {
                write!(fmt, "{} {}, @{}", self.get_mnemonic(), rs, addr)
            }
            Instruction::MovbAbsoluteToAbsolute(src_addr, dst_addr) => {
                write!(fmt, "{} @{}, @{}", self.get_mnemonic(), src_addr, dst_addr)
            }
            Instruction::MoveFieldRegToAbsolute(rs, addr, f) => {
                write!(fmt, "{} {}, @{}, {}", self.get_mnemonic(), rs, addr, f)
            }
            Instruction::MoveFieldAbsoluteToReg(addr, rd, f) => {
                write!(fmt, "{} @{}, {}, {}", self.get_mnemonic(), addr, rd, f)
            }
            Instruction::MoveFieldAbsoluteToAbsolute(src_addr, dst_addr, f) => {
                write!(
                    fmt,
                    "{} @{}, @{}, {}",
                    self.get_mnemonic(),
                    src_addr,
                    dst_addr,
                    f
                )
            }
            Instruction::MoveFieldAbsoluteToIndirectPostinc(addr, rd, f) => {
                write!(fmt, "{} @{}, *{}+, {}", self.get_mnemonic(), addr, rd, f)
            }
            Instruction::MoveFieldRegToIndirect(rs, rd, f) => {
                write!(fmt, "{} *{}, {}, {}", self.get_mnemonic(), rs, rd, f)
            }
            Instruction::MoveFieldRegToIndirectOffset(rs, rd, f, offset) => {
                write!(
                    fmt,
                    "{} {}, *{}({:X}h), {}",
                    self.get_mnemonic(),
                    rs,
                    rd,
                    offset.0 as u16,
                    f
                )
            }
            Instruction::MoveFieldRegToIndirectPredec(rs, rd, f) => {
                write!(fmt, "{} {}, -*{}, {}", self.get_mnemonic(), rs, rd, f)
            }
            Instruction::MoveFieldRegToIndirectPostinc(rs, rd, f) => {
                write!(fmt, "{} {}, *{}+, {}", self.get_mnemonic(), rs, rd, f)
            }
            Instruction::MoveFieldIndirectToReg(rs, rd, f) => {
                write!(fmt, "{} *{}, {}, {}", self.get_mnemonic(), rs, rd, f)
            }
            Instruction::MoveFieldIndirectPredecToReg(rs, rd, f) => {
                write!(fmt, "{} -*{}, {}, {}", self.get_mnemonic(), rs, rd, f)
            }
            Instruction::MoveFieldIndirectPostincToReg(rs, rd, f) => {
                write!(fmt, "{} *{}+, {}, {}", self.get_mnemonic(), rs, rd, f)
            }
            Instruction::MoveFieldIndirectToIndirect(rs, rd, f) => {
                write!(fmt, "{} *{}, *{}, {}", self.get_mnemonic(), rs, rd, f)
            }
            Instruction::MoveFieldIndirectToIndirectPredec(rs, rd, f) => {
                write!(fmt, "{} -*{}, -*{}, {}", self.get_mnemonic(), rs, rd, f)
            }
            Instruction::MoveFieldIndirectToIndirectPostinc(rs, rd, f) => {
                write!(fmt, "{} *{}+, *{}+, {}", self.get_mnemonic(), rs, rd, f)
            }
            Instruction::MoveFieldIndirectOffsetToReg(rs, rd, f, offset) => {
                write!(
                    fmt,
                    "{} *{}({:X}h), {}, {}",
                    self.get_mnemonic(),
                    rs,
                    offset.0 as u16,
                    rd,
                    f
                )
            }
            Instruction::MoveFieldIndirectOffsetToIndirectPostinc(rs, rd, f, offset) => {
                write!(
                    fmt,
                    "{} *{}({:X}h), *{}+, {}",
                    self.get_mnemonic(),
                    rs,
                    offset.0 as u16,
                    rd,
                    f
                )
            }
            Instruction::MoveFieldIndirectOffsetToIndirectOffset(rs, rd, f, offset1, offset2) => {
                write!(
                    fmt,
                    "{} *{}({:X}h), *{}({:X}h), {}",
                    self.get_mnemonic(),
                    rs,
                    offset1.0 as u16,
                    rd,
                    offset2.0 as u16,
                    f
                )
            }
            Instruction::Andi(il, rd) => {
                let ones_comp = IL(!il.0);
                write!(fmt, "{}, {}, {}", self.get_mnemonic(), ones_comp, rd)
            }
            Instruction::Cmpiw(iw, rd) => {
                write!(fmt, "{} {:08X}h, {}", self.get_mnemonic(), !iw.0, rd)
            }
            Instruction::Sllk(k, rd)
            | Instruction::Rlk(k, rd)
            | Instruction::Srlk(k, rd)
            | Instruction::Srak(k, rd)
            | Instruction::Slak(k, rd) => {
                write!(fmt, "{} {}, {}", self.get_mnemonic(), k, rd)
            }
            Instruction::Addk(k, rd) | Instruction::Subk(k, rd) | Instruction::Movk(k, rd) => {
                let special_k = if k.0 == 0 { K(32) } else { *k };
                write!(fmt, "{} {}, {}", self.get_mnemonic(), special_k, rd)
            }
            Instruction::Exgpc(rd, f) => {
                write!(fmt, "{} {}, {}", self.get_mnemonic(), rd, f)
            }
            Instruction::Line(z) => {
                write!(fmt, "{} {}", self.get_mnemonic(), z)
            }
            Instruction::Dw(word) => {
                write!(fmt, "{} {:04X}h", self.get_mnemonic(), word.0)
            }
            Instruction::Jr(condition, offset, pc) => {
                write!(
                    fmt,
                    "{}{} {:X}h",
                    self.get_mnemonic(),
                    condition,
                    (((pc.0 as i64 + offset.0 as i16 as i64) * 16) + 16) as u32
                )
            }

            Instruction::Mmtm(rd, reglist) => {
                let rf = rd.0.view_bits::<Lsb0>()[4];
                let mut reg_letter = 'A';
                if rf {
                    reg_letter = 'B';
                }
                let mut reglist_str = String::new();
                let iter = reglist.0.view_bits::<Lsb0>().iter_ones().peekable();
                for reg_number in iter {
                    if reg_number == 15 {
                        write!(reglist_str, "SP").unwrap();
                    } else {
                        write!(reglist_str, "{}{}", reg_letter, reg_number).unwrap();
                    }
                    write!(reglist_str, ", ").unwrap();
                }
                // can't be arsed to come up with logic to avoid writing an extraneous comma
                // fix it in post!
                reglist_str = reglist_str[0..reglist_str.len() - 2].to_string();
                write!(fmt, "{} {}, {}", self.get_mnemonic(), rd, reglist_str)
            }

            // The type system has betrayed me!
            // Rd and Rs are different types so MMTM and MMFM can't be in the same match block
            // YOLO copy-pasta
            Instruction::Mmfm(rs, reglist) => {
                let rf = rs.0.view_bits::<Lsb0>()[4];
                let mut reg_letter = 'A';
                if rf {
                    reg_letter = 'B';
                }
                let mut reglist_str = String::new();
                let iter = reglist.0.view_bits::<Lsb0>().iter_ones().peekable();
                for reg_number in iter {
                    if reg_number == 15 {
                        write!(reglist_str, "SP").unwrap();
                    } else {
                        write!(reglist_str, "{}{}", reg_letter, reg_number).unwrap();
                    }
                    write!(reglist_str, ", ").unwrap();
                }
                // can't be arsed to come up with logic to avoid writing an extraneous comma
                // fix it in post!
                reglist_str = reglist_str[0..reglist_str.len() - 2].to_string();

                write!(fmt, "{} {}, {}", self.get_mnemonic(), rs, reglist_str)
            }

            Instruction::Btstk(_, _)
            | Instruction::Dsj(_, _)
            | Instruction::Dsjne(_, _)
            | Instruction::Dsjeq(_, _) => {
                write!(fmt, "UNIMPL {:?}", self)
            }
        }
    }
}

pub fn disassemble_stage2(stage1_output: Vec<Dis>) -> String {
    let mut disassembly = String::new();

    for Dis(pc, inst, words) in stage1_output {
        let mut hexdump = String::new();
        write!(hexdump, "{:04X?}", words).unwrap();
        hexdump = hexdump.replace("[", "");
        hexdump = hexdump.replace("]", "");
        hexdump = hexdump.replace(",", "");

        writeln!(
            disassembly,
            "{:08X}:\t{}{}{}",
            pc * 16,
            inst,
            " ".repeat(60 - inst.to_string().len()),
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
