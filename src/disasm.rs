use bitvec::prelude::*;

use crate::instruction::Instruction;
use core::fmt::{self, Formatter, Write};

use crate::symbol::{Rs, Rd, IW, IL, K, F, D, Address, FS, FE, N, M, Offset, Offset8, Z, Condition, RegList};

pub fn disassemble_stage1(bytebuf: &[u8]) -> Vec<(usize, Instruction)> {
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
                                inst_vec.push((pc, Instruction::Rev(Rd((rf as u8) << 4 | rd))));
                            },
                            8 => {
                                inst_vec.push((pc, Instruction::Emu));
                            },
                            9 => {
                                inst_vec.push((pc, Instruction::Exgpc(Rd((rf as u8) << 4 | rd))));
                            },
                            10 => {
                                inst_vec.push((pc, Instruction::Getpc(Rd((rf as u8) << 4 | rd))));
                            },
                            11 => {
                                // this is kind of strange that we assign rd to Rs, but it's in the
                                // position of rd, and the manual calls it Rs because it's the
                                // source of the argument to jump to
                                inst_vec.push((pc, Instruction::Jump(Rs((rf as u8) << 4 | rd))));
                            },
                            12 => {
                                inst_vec.push((pc, Instruction::Getst(Rd((rf as u8) << 4 | rd))));
                            },
                            13 => {
                                inst_vec.push((pc, Instruction::Putst(Rs((rf as u8) << 4 | rd))));
                            },
                            14 => {
                                inst_vec.push((pc, Instruction::Popst));
                            },
                            15 => {
                                inst_vec.push((pc, Instruction::Pushst));
                            },
                            _ => {}
                        }
                    },
                    0b0000001 => {
                        match subop {
                            8 => {
                                inst_vec.push((pc, Instruction::Nop));
                            },
                            9 =>
                            {
                                inst_vec.push((pc, Instruction::Clrc));
                            },
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
                                inst_vec.push((pc, Instruction::MovbAbsoluteToAbsolute(Address(src_addr), Address(dst_addr))));
                            },
                            11 => {
                                inst_vec.push((pc, Instruction::Dint));
                            },
                            12 => {
                                inst_vec.push((pc, Instruction::Abs(Rd((rf as u8) << 4 | rd))));
                            },
                            13 => {
                                inst_vec.push((pc, Instruction::Neg(Rd((rf as u8) << 4 | rd))));
                            },
                            14 => {
                                inst_vec.push((pc, Instruction::Negb(Rd((rf as u8) << 4 | rd))));
                            },
                            15 => {
                                inst_vec.push((pc, Instruction::Not(Rd((rf as u8) << 4 | rd))));
                            },
                            _ => {},
                        }
                    },
                    0b0000010 |
                    0b0000011 => {
                        match subop {
                            8 => {
                                inst_vec.push((pc, Instruction::Sext(Rd((rf as u8) << 4 | rd), F(f)))); 
                            },
                            9 => {
                                inst_vec.push((pc, Instruction::Zext(Rd((rf as u8) << 4 | rd), F(f)))); 
                            },
                            10 | 11 => {
                                inst_vec.push((pc, Instruction::Setf(FS(fs), FE(fe), Some(F(f)))));
                            }
                            12 => {
                                let mut address: u32 = 0;
                                let lsb = word_iter.next().unwrap().1.load::<u16>();
                                let msb = word_iter.next().unwrap().1.load::<u16>();
                                address.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                                address.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                                inst_vec.push((pc, Instruction::MoveFieldRegToAbsolute(Rs((rf as u8) << 4 | rd), Address(address), Some(F(f)))));
                            },
                            13 => {
                                let mut address: u32 = 0;
                                let lsb = word_iter.next().unwrap().1.load::<u16>();
                                let msb = word_iter.next().unwrap().1.load::<u16>();
                                address.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                                address.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                                inst_vec.push((pc, Instruction::MoveFieldAbsoluteToReg(Address(address), Rd((rf as u8) << 4 | rd), Some(F(f)))));
                            },
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
                                inst_vec.push((pc, Instruction::MoveFieldAbsoluteToAbsolute(Address(src_addr), Address(dst_addr), Some(F(f)))));
                            },
                            15 => {
                                if f {
                                    // this has nothing to do with fields I was just too lazy to
                                    // make an alias for bit 9
                                    let mut address: u32 = 0;
                                    let lsb = word_iter.next().unwrap().1.load::<u16>();
                                    let msb = word_iter.next().unwrap().1.load::<u16>();
                                    address.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                                    address.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                                    inst_vec.push((pc, Instruction::MovbAbsoluteToReg(Address(address), Rd((rf as u8) << 4 | rd))));
                                } 
                                else {
                                    let mut address: u32 = 0;
                                    let lsb = word_iter.next().unwrap().1.load::<u16>();
                                    let msb = word_iter.next().unwrap().1.load::<u16>();
                                    address.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                                    address.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                                    inst_vec.push((pc, Instruction::MovbRegToAbsolute(Rs((rf as u8) << 4 | rd), Address(address))));
                                }
                            },
                            _ => {}
                        }
                    },
                    0b0000100 => {
                        match subop {
                            8 => {
                                inst_vec.push((pc, Instruction::Trap(N(n))));
                            },
                            9 => {
                                inst_vec.push((pc, Instruction::Call(Rs(rd))));
                            },
                            10 => {
                                inst_vec.push((pc, Instruction::Reti));
                            },
                            11 => {
                                inst_vec.push((pc, Instruction::Rets(N(n))));
                            },
                            12 => {
                                let reglist = word_iter.next().unwrap().1.load::<u16>();
                                inst_vec.push((pc, Instruction::Mmtm(Rd((rf as u8) << 4 | rd), RegList(reglist))));
                            },
                            13 => {
                                let reglist = word_iter.next().unwrap().1.load::<u16>();
                                inst_vec.push((pc, Instruction::Mmfm(Rs((rf as u8) << 4 | rd), RegList(reglist))));
                            },
                            14 => {
                                let iw = word_iter.next().unwrap().1.load::<u16>();
                                inst_vec.push((pc, Instruction::Moviw(IW(iw),Rd((rf as u8) << 4 | rd))));
                            },
                            15 => {
                                let mut il: u32 = 0;
                                let lsb = word_iter.next().unwrap().1.load::<u16>();
                                let msb = word_iter.next().unwrap().1.load::<u16>();
                                il.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                                il.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                                inst_vec.push((pc, Instruction::Movil(IL(il),Rd((rf as u8) << 4 | rd))));
                            },
                            _ => {}
                        }
                    },
                    0b0000101 => {
                        match subop {
                            8 => {
                                let iw = word_iter.next().unwrap().1.load::<u16>();
                                inst_vec.push((pc, Instruction::Addiw(IW(iw), Rd((rf as u8) << 4 | rd))));
                            },
                            9 => {
                                let mut il: u32 = 0;
                                let lsb = word_iter.next().unwrap().1.load::<u16>();
                                let msb = word_iter.next().unwrap().1.load::<u16>();
                                il.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                                il.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                                inst_vec.push((pc, Instruction::Addil(IL(il), Rd((rf as u8) << 4 | rd))));
                            },
                            10 => {
                                let iw = word_iter.next().unwrap().1.load::<u16>();
                                inst_vec.push((pc, Instruction::Cmpiw(IW(iw), Rd((rf as u8) << 4 | rd))));
                            },
                            11 => {
                                let mut il: u32 = 0;
                                let lsb = word_iter.next().unwrap().1.load::<u16>();
                                let msb = word_iter.next().unwrap().1.load::<u16>();
                                il.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                                il.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                                inst_vec.push((pc, Instruction::Cmpil(IL(il), Rd((rf as u8) << 4 | rd))));
                            },
                            12 => {
                                let mut il: u32 = 0;
                                let lsb = word_iter.next().unwrap().1.load::<u16>();
                                let msb = word_iter.next().unwrap().1.load::<u16>();
                                il.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                                il.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                                inst_vec.push((pc, Instruction::Andi(IL(il), Rd((rf as u8) << 4 | rd))));
                            }
                            13 => {
                                let mut il: u32 = 0;
                                let lsb = word_iter.next().unwrap().1.load::<u16>();
                                let msb = word_iter.next().unwrap().1.load::<u16>();
                                il.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                                il.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                                inst_vec.push((pc, Instruction::Ori(IL(il), Rd((rf as u8) << 4 | rd))));
                            },
                            14 => {
                                let mut il: u32 = 0;
                                let lsb = word_iter.next().unwrap().1.load::<u16>();
                                let msb = word_iter.next().unwrap().1.load::<u16>();
                                il.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                                il.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                                inst_vec.push((pc, Instruction::Xori(IL(il), Rd((rf as u8) << 4 | rd))));
                            },
                            15 => {
                                let iw = word_iter.next().unwrap().1.load::<u16>();
                                inst_vec.push((pc, Instruction::Subiw(IW(iw), Rd((rf as u8) << 4 | rd))));
                            },
                            _ => {}
                        }
                    },
                    0b0000110 => {
                        match subop {
                            8 => {
                                let mut il: u32 = 0;
                                let lsb = word_iter.next().unwrap().1.load::<u16>();
                                let msb = word_iter.next().unwrap().1.load::<u16>();
                                il.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                                il.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                                inst_vec.push((pc, Instruction::Subil(IL(il), Rd((rf as u8) << 4 | rd))));
                            },
                            9 => {
                                let offset = word_iter.next().unwrap().1.load::<u16>();
                                inst_vec.push((pc, Instruction::Callr(Offset(offset))));
                            },
                            10 => {
                                let mut address: u32 = 0;
                                let lsb = word_iter.next().unwrap().1.load::<u16>();
                                let msb = word_iter.next().unwrap().1.load::<u16>();
                                address.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                                address.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                                inst_vec.push((pc, Instruction::Calla(Address(address))));
                            },
                            11 => {
                                inst_vec.push((pc, Instruction::Eint));
                            },
                            12 => {
                                let offset = word_iter.next().unwrap().1.load::<u16>();
                                inst_vec.push((pc, Instruction::Dsj(Rd((rf as u8) << 4 | rd), Offset(offset))));
                            },
                            13 => {
                                let offset = word_iter.next().unwrap().1.load::<u16>();
                                inst_vec.push((pc, Instruction::Dsjeq(Rd((rf as u8) << 4 | rd), Offset(offset))));
                            },
                            14 => {
                                let offset = word_iter.next().unwrap().1.load::<u16>();
                                inst_vec.push((pc, Instruction::Dsjne(Rd((rf as u8) << 4 | rd), Offset(offset))));
                            },
                            15 => {
                                inst_vec.push((pc, Instruction::Setc));
                            },
                            _ => {}
                        }
                    },
                    0b0000111 => {
                        match subop {
                            8 => {
                                inst_vec.push((pc, Instruction::Pixbltll));
                            },
                            9 => {
                                inst_vec.push((pc, Instruction::Pixbltlxy));
                            },
                            10 => {
                                inst_vec.push((pc, Instruction::Pixbltxyl));
                            },
                            11 => {
                                inst_vec.push((pc, Instruction::Pixbltxyxy));
                            },
                            12 => {
                                inst_vec.push((pc, Instruction::Pixbltbl));
                            },
                            13 => {
                                inst_vec.push((pc, Instruction::Pixbltbxy));
                            },
                            14 => {
                                inst_vec.push((pc, Instruction::Filll));
                            },
                            15 => {
                                inst_vec.push((pc, Instruction::Fillxy));
                            },
                            _ => {},
                        }
                    },
                    0b0001000 | 0b0001001 => {
                        if k == 1 {
                            inst_vec.push((pc, Instruction::Inc(Rd((rf as u8) << 4 | rd))));
                        }
                        else {
                            inst_vec.push((pc, Instruction::Addk(K(k), Rd((rf as u8) << 4 | rd))));
                        }
                    },
                    0b0001010 | 0b0001011 => {
                        if k == 1 {
                            inst_vec.push((pc, Instruction::Dec(Rd((rf as u8) << 4 | rd))));
                        }
                        else {
                            inst_vec.push((pc, Instruction::Subk(K(k), Rd((rf as u8) << 4 | rd))));
                        }
                    },
                    0b0001100 | 0b0001101 => {
                        inst_vec.push((pc, Instruction::Movk(K(k), Rd((rf as u8) << 4 | rd)))); 
                    },
                    0b0001110 | 0b0001111 => {
                        // reminder to deal with 1's complement when formatting and assembling
                        inst_vec.push((pc, Instruction::Btstk(K(k), Rd((rf as u8) << 4 | rd)))); 
                    },
                    0b0010000 | 0b0010001 => {
                        inst_vec.push((pc, Instruction::Slak(K(k), Rd((rf as u8) << 4 | rd))));
                    },
                    0b0010010 | 0b0010011 => {
                        inst_vec.push((pc, Instruction::Sllk(K(k), Rd((rf as u8) << 4 | rd))));
                    },
                    0b0010100 | 0b0010101 => {
                        // reminder to deal with 2's complement when formatting and assembling
                        inst_vec.push((pc, Instruction::Srak(K(k), Rd((rf as u8) << 4 | rd))));
                    },
                    0b0010110 | 0b0010111 => {
                        // reminder to deal with 2's complement when formatting and assembling
                        inst_vec.push((pc, Instruction::Srlk(K(k), Rd((rf as u8) << 4 | rd))));
                    },
                    0b0011000 | 0b0011001 => {
                        inst_vec.push((pc, Instruction::Rlk(K(k), Rd((rf as u8) << 4 | rd))));
                    },
                    0b0011100..=0b0011111 => {
                        inst_vec.push((pc, Instruction::Dsjs(D(d), Rd((rf as u8) << 4 | rd), K(k)))); 
                    },
                    0b0100000 => {
                        inst_vec.push((pc, Instruction::Add(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b0100001 => {
                        inst_vec.push((pc, Instruction::Addc(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b0100010 => {
                        inst_vec.push((pc, Instruction::Sub(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b0100011 => {
                        inst_vec.push((pc, Instruction::Subb(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b0100100 => {
                        inst_vec.push((pc, Instruction::Cmp(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b0100101 => {
                        inst_vec.push((pc, Instruction::Btst(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b0100110 | 0b0100111 => {
                        inst_vec.push((pc, Instruction::MoveReg(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd), M(f))));
                    },
                    0b0101000 => {
                        inst_vec.push((pc, Instruction::And(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b0101001 => {
                        inst_vec.push((pc, Instruction::Andn(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b0101010 => {
                        inst_vec.push((pc, Instruction::Or(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b0101011 => {
                        if rs == rd {
                            inst_vec.push((pc, Instruction::Clr(Rd((rf as u8) << 4 | rd))));
                        }
                        else {
                            inst_vec.push((pc, Instruction::Xor(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                        }
                    },
                    0b0101100 => {
                        inst_vec.push((pc, Instruction::Divs(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b0101101 => {
                        inst_vec.push((pc, Instruction::Divu(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b0101110 => {
                        inst_vec.push((pc, Instruction::Mpys(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b0101111 => {
                        inst_vec.push((pc, Instruction::Mpyu(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b0110001 => {
                        inst_vec.push((pc, Instruction::Sll(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b0110010 => {
                        inst_vec.push((pc, Instruction::Sra(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b0110011 => {
                        inst_vec.push((pc, Instruction::Srl(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b0110100 => {
                        inst_vec.push((pc, Instruction::Rl(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b0110101 => {
                        inst_vec.push((pc, Instruction::Lmo(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b0110110 => {
                        inst_vec.push((pc, Instruction::Mods(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b0110111 => {
                        inst_vec.push((pc, Instruction::Modu(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b1000000 | 0b1000001 => {
                        inst_vec.push((pc, Instruction::MoveFieldRegToIndirect(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd), Some(F(f)))));
                    },
                    0b1000010 | 0b1000011 => {
                        inst_vec.push((pc, Instruction::MoveFieldIndirectToReg(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd), Some(F(f)))));
                    },
                    0b1000100 | 0b1000101 => {
                        inst_vec.push((pc, Instruction::MoveFieldIndirectToIndirect(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd), Some(F(f)))));
                    },
                    0b1000110 => {
                        inst_vec.push((pc, Instruction::MovbRegToIndirect(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b1000111 => {
                        inst_vec.push((pc, Instruction::MovbIndirectToReg(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b1001000 | 0b1001001 => {
                        inst_vec.push((pc, Instruction::MoveFieldRegToIndirectPostinc(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd), Some(F(f)))));
                    },
                    0b1001010 | 0b1001011 => {
                        inst_vec.push((pc, Instruction::MoveFieldIndirectPostincToReg(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd), Some(F(f)))));
                    },
                    0b1001100 | 0b1001101 => {
                        inst_vec.push((pc, Instruction::MoveFieldIndirectToIndirectPostinc(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd), Some(F(f)))));
                    },
                    0b1001110 => {
                        inst_vec.push((pc, Instruction::MovbIndirectToIndirect(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b1010000 | 0b1010001 => {
                        inst_vec.push((pc, Instruction::MoveFieldRegToIndirectPredec(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd), Some(F(f)))));
                    },
                    0b1010010 | 0b1010011 => {
                        inst_vec.push((pc, Instruction::MoveFieldIndirectPredecToReg(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd), Some(F(f)))));
                    },
                    0b1010110 => {
                        let offset = word_iter.next().unwrap().1.load::<u16>();
                        inst_vec.push((pc, Instruction::MovbRegToIndirectOffset(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd), Offset(offset))));
                    },
                    0b1010111 => {
                        let offset = word_iter.next().unwrap().1.load::<u16>();
                        inst_vec.push((pc, Instruction::MovbIndirectOffsetToReg(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd), Offset(offset))));
                    },
                    0b1011000 | 0b1011001 => {
                        let offset = word_iter.next().unwrap().1.load::<u16>();
                        inst_vec.push((pc, Instruction::MoveFieldRegToIndirectOffset(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd), Some(F(f)), Offset(offset))));
                    },
                    0b1011010 | 0b1011011 => {
                        let offset = word_iter.next().unwrap().1.load::<u16>();
                        inst_vec.push((pc, Instruction::MoveFieldIndirectOffsetToReg(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd), Some(F(f)), Offset(offset))));
                    },
                    0b1011100 | 0b1011101 => {
                        let src_offset = word_iter.next().unwrap().1.load::<u16>();
                        let dst_offset = word_iter.next().unwrap().1.load::<u16>();
                        inst_vec.push((pc, Instruction::MoveFieldIndirectOffsetToIndirectOffset(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd), Some(F(f)), Offset(src_offset), Offset(dst_offset))));
                    },
                    0b1011110 => {
                        let src_offset = word_iter.next().unwrap().1.load::<u16>();
                        let dst_offset = word_iter.next().unwrap().1.load::<u16>();
                        inst_vec.push((pc, Instruction::MovbIndirectOffsetToIndirectOffset(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd), Offset(src_offset), Offset(dst_offset))));
                    },
                    0b1100000..=0b1100111 => {
                        let lower8 = word.get(0..=7).unwrap().load::<u8>();
                        if lower8 == 0x80 {
                            let mut address: u32 = 0;
                            let lsb = word_iter.next().unwrap().1.load::<u16>();
                            let msb = word_iter.next().unwrap().1.load::<u16>();
                            address.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                            address.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                            inst_vec.push((pc, Instruction::Ja(Condition(cc), Address(address))));
                        } else if lower8 == 0x00 {
                            let offset = word_iter.next().unwrap().1.load::<u16>();
                            inst_vec.push((pc, Instruction::Jrs(Condition(cc), Offset(offset))));
                        }
                        else {
                            inst_vec.push((pc, Instruction::Jr(Condition(cc), Offset8(lower8))));
                        }
                    },
                    0b1101000 | 0b1101001 => {
                        let offset = word_iter.next().unwrap().1.load::<u16>();
                        inst_vec.push((pc, Instruction::MoveFieldIndirectOffsetToIndirectPostinc(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd), Some(F(f)), Offset(offset))));
                    },
                    0b1101010 | 0b1101011 => {
                        match subop {
                            0 => {
                                let mut address: u32 = 0;
                                let lsb = word_iter.next().unwrap().1.load::<u16>();
                                let msb = word_iter.next().unwrap().1.load::<u16>();
                                address.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                                address.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                                inst_vec.push((pc, Instruction::MoveFieldAbsoluteToIndirectPostinc(Address(address), Rd((rf as u8) << 4 | rd), Some(F(f)))));
                            },
                            8 => {
                                inst_vec.push((pc, Instruction::Exgf(Rd((rf as u8) << 4 | rd), Some(F(f)))));
                            },
                            _ => {}
                        }
                    },
                    0b1101111 => {
                        inst_vec.push((pc, Instruction::Line(Z(z))));
                    },
                    0b1110000 => {
                        inst_vec.push((pc, Instruction::Addxy(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b1110001 => {
                        inst_vec.push((pc, Instruction::Subxy(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b1110010 => {
                        inst_vec.push((pc, Instruction::Cmpxy(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b1110011 => {
                        inst_vec.push((pc, Instruction::Cpw(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b1110100 => {
                        inst_vec.push((pc, Instruction::Cvxyl(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b1110110 => {
                        inst_vec.push((pc, Instruction::Movx(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b1110111 => {
                        inst_vec.push((pc, Instruction::Movy(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b1111000 => {
                        inst_vec.push((pc, Instruction::PixtRegToIndirectxy(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b1111001 => {
                        inst_vec.push((pc, Instruction::PixtIndirectxyToReg(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b1111010 => {
                        inst_vec.push((pc, Instruction::PixtIndirectxytoIndirectxy(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b1111011 => {
                        inst_vec.push((pc, Instruction::Drav(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b1111100 => {
                        inst_vec.push((pc, Instruction::PixtRegToIndirect(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b1111101 => {
                        inst_vec.push((pc, Instruction::PixtIndirectToReg(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
                    },
                    0b1111110 => {
                        inst_vec.push((pc, Instruction::PixtIndirectToIndirect(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd))));
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

impl fmt::Display for Instruction
{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Instruction::Setf(fs, fe, Some(f)) => {
                write!(fmt, "SETF {},{},{}", fs, fe, f)
            }
            Instruction::Movk(k, rd) => {
                write!(fmt, "MOVK {},{}", k, rd)
            }
            _ => {write!(fmt, "")}
        }
    }
}

pub fn disassemble_stage2(stage1_output: Vec<(usize, Instruction)>) -> String {
    let mut disassembly = String::new();
    for (pc, inst) in stage1_output {
        writeln!(disassembly, "{:08x}\t{}", pc*16, inst).unwrap();
    }
    disassembly
}


pub fn disassemble(bytebuf: &[u8]) {
    println!("{}", disassemble_stage2(disassemble_stage1(bytebuf)));
}
