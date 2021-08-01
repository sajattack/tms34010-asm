use bitvec::prelude::*;

use crate::instruction::Instruction;

#[allow(unused)]
use crate::symbol::{Rs, Rd, IW, IL, K, F, D, Address, FS, FE, N, M, Offset, Z, Condition};

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
                let f = *word.get(9).unwrap();
                let fs = word.get(0..=4).unwrap().load::<u8>();
                let n = word.get(0..=4).unwrap().load::<u8>();
                let fe = *word.get(5).unwrap();
                let k = word.get(5..=9).unwrap().load::<u8>();
                let d = *word.get(10).unwrap();
                let z = *word.get(7).unwrap();
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
                                let mut src_addr: u32 = 0;
                                let mut dst_addr: u32 = 0;
                                let src_lsb = word_iter.next().unwrap().load::<u16>();
                                let src_msb = word_iter.next().unwrap().load::<u16>();
                                let dst_lsb = word_iter.next().unwrap().load::<u16>();
                                let dst_msb = word_iter.next().unwrap().load::<u16>();
                                src_addr.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(src_lsb);
                                src_addr.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(src_msb);
                                dst_addr.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(dst_lsb);
                                dst_addr.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(dst_msb);
                                inst_vec.push(Instruction::MovbAbsoluteToAbsolute(Address(src_addr), Address(dst_addr)));
                            },
                            11 => {
                                inst_vec.push(Instruction::Dint);
                            },
                            12 => {
                                inst_vec.push(Instruction::Abs(Rd((rf as u8) << 4 | rd)));
                            },
                            13 => {
                                inst_vec.push(Instruction::Neg(Rd((rf as u8) << 4 | rd)));
                            },
                            14 => {
                                inst_vec.push(Instruction::Negb(Rd((rf as u8) << 4 | rd)));
                            },
                            15 => {
                                inst_vec.push(Instruction::Not(Rd((rf as u8) << 4 | rd)));
                            },
                            _ => {},
                        }
                    },
                    0b0000010 |
                    0b0000011 => {
                        match subop {
                            8 => {
                                inst_vec.push(Instruction::Sext(Rd((rf as u8) << 4 | rd), F(f))); 
                            },
                            9 => {
                                inst_vec.push(Instruction::Zext(Rd((rf as u8) << 4 | rd), F(f))); 
                            },
                            10 | 11 => {
                                inst_vec.push(Instruction::Setf(FS(fs), FE(fe), Some(F(f))));
                            }
                            12 => {
                                let mut address: u32 = 0;
                                let lsb = word_iter.next().unwrap().load::<u16>();
                                let msb = word_iter.next().unwrap().load::<u16>();
                                address.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                                address.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                                inst_vec.push(Instruction::MoveFieldRegToAbsolute(Rs((rf as u8) << 4 | rd), Address(address), Some(F(f))));
                            },
                            13 => {
                                let mut address: u32 = 0;
                                let lsb = word_iter.next().unwrap().load::<u16>();
                                let msb = word_iter.next().unwrap().load::<u16>();
                                address.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                                address.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                                inst_vec.push(Instruction::MoveFieldAbsoluteToReg(Address(address), Rd((rf as u8) << 4 | rd), Some(F(f))));
                            },
                            14 => {
                                let mut src_addr: u32 = 0;
                                let mut dst_addr: u32 = 0;
                                let src_lsb = word_iter.next().unwrap().load::<u16>();
                                let src_msb = word_iter.next().unwrap().load::<u16>();
                                let dst_lsb = word_iter.next().unwrap().load::<u16>();
                                let dst_msb = word_iter.next().unwrap().load::<u16>();
                                src_addr.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(src_lsb);
                                src_addr.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(src_msb);
                                dst_addr.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(dst_lsb);
                                dst_addr.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(dst_msb);
                                inst_vec.push(Instruction::MoveFieldAbsoluteToAbsolute(Address(src_addr), Address(dst_addr), Some(F(f))));
                            },
                            15 => {
                                if f {
                                    // this has nothing to do with fields I was just too lazy to
                                    // make an alias for bit 9
                                    let mut address: u32 = 0;
                                    let lsb = word_iter.next().unwrap().load::<u16>();
                                    let msb = word_iter.next().unwrap().load::<u16>();
                                    address.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                                    address.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                                    inst_vec.push(Instruction::MovbAbsoluteToReg(Address(address), Rd((rf as u8) << 4 | rd)));
                                } 
                                else {
                                    let mut address: u32 = 0;
                                    let lsb = word_iter.next().unwrap().load::<u16>();
                                    let msb = word_iter.next().unwrap().load::<u16>();
                                    address.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                                    address.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                                    inst_vec.push(Instruction::MovbRegToAbsolute(Rs((rf as u8) << 4 | rd), Address(address)));
                                }
                            },
                            _ => {}
                        }
                    },
                    0b0000100 => {
                        match subop {
                            8 => {
                                inst_vec.push(Instruction::Trap(N(n)));
                            },
                            9 => {
                                inst_vec.push(Instruction::Call(Rs(rd)));
                            },
                            10 => {
                                inst_vec.push(Instruction::Reti);
                            },
                            11 => {
                                inst_vec.push(Instruction::Rets(N(n)));
                            },
                            12 => {
                                todo!("MMTM - looks confusing")
                            },
                            13 => {
                                todo!("MMFM - looks confusing")
                            },
                            14 => {
                                let iw = word_iter.next().unwrap().load::<u16>();
                                inst_vec.push(Instruction::Moviw(IW(iw),Rd((rf as u8) << 4 | rd)));
                            },
                            15 => {
                                let mut il: u32 = 0;
                                let lsb = word_iter.next().unwrap().load::<u16>();
                                let msb = word_iter.next().unwrap().load::<u16>();
                                il.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                                il.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                                inst_vec.push(Instruction::Movil(IL(il),Rd((rf as u8) << 4 | rd)));
                            },
                            _ => {}
                        }
                    },
                    0b0000101 => {
                        match subop {
                            8 => {
                                let iw = word_iter.next().unwrap().load::<u16>();
                                inst_vec.push(Instruction::Addiw(IW(iw), Rd((rf as u8) << 4 | rd)));
                            },
                            9 => {
                                let mut il: u32 = 0;
                                let lsb = word_iter.next().unwrap().load::<u16>();
                                let msb = word_iter.next().unwrap().load::<u16>();
                                il.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                                il.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                                inst_vec.push(Instruction::Addil(IL(il), Rd((rf as u8) << 4 | rd)));
                            },
                            10 => {
                                let iw = word_iter.next().unwrap().load::<u16>();
                                inst_vec.push(Instruction::Cmpiw(IW(iw), Rd((rf as u8) << 4 | rd)));
                            },
                            11 => {
                                let mut il: u32 = 0;
                                let lsb = word_iter.next().unwrap().load::<u16>();
                                let msb = word_iter.next().unwrap().load::<u16>();
                                il.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                                il.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                                inst_vec.push(Instruction::Cmpil(IL(il), Rd((rf as u8) << 4 | rd)));
                            },
                            12 => {
                                let mut il: u32 = 0;
                                let lsb = word_iter.next().unwrap().load::<u16>();
                                let msb = word_iter.next().unwrap().load::<u16>();
                                il.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                                il.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                                inst_vec.push(Instruction::Andi(IL(il), Rd((rf as u8) << 4 | rd)));
                            }
                            13 => {
                                let mut il: u32 = 0;
                                let lsb = word_iter.next().unwrap().load::<u16>();
                                let msb = word_iter.next().unwrap().load::<u16>();
                                il.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                                il.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                                inst_vec.push(Instruction::Ori(IL(il), Rd((rf as u8) << 4 | rd)));
                            },
                            14 => {
                                let mut il: u32 = 0;
                                let lsb = word_iter.next().unwrap().load::<u16>();
                                let msb = word_iter.next().unwrap().load::<u16>();
                                il.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                                il.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                                inst_vec.push(Instruction::Xori(IL(il), Rd((rf as u8) << 4 | rd)));
                            },
                            15 => {
                                let iw = word_iter.next().unwrap().load::<u16>();
                                inst_vec.push(Instruction::Subiw(IW(iw), Rd((rf as u8) << 4 | rd)));
                            },
                            _ => {}
                        }
                    },
                    0b0000110 => {
                        match subop {
                            8 => {
                                let mut il: u32 = 0;
                                let lsb = word_iter.next().unwrap().load::<u16>();
                                let msb = word_iter.next().unwrap().load::<u16>();
                                il.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                                il.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                                inst_vec.push(Instruction::Subil(IL(il), Rd((rf as u8) << 4 | rd)));
                            },
                            9 => {
                                let offset = word_iter.next().unwrap().load::<u16>();
                                inst_vec.push(Instruction::Callr(Offset(offset)));
                            },
                            10 => {
                                let mut address: u32 = 0;
                                let lsb = word_iter.next().unwrap().load::<u16>();
                                let msb = word_iter.next().unwrap().load::<u16>();
                                address.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                                address.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                                inst_vec.push(Instruction::Calla(Address(address)));
                            },
                            11 => {
                                inst_vec.push(Instruction::Eint);
                            },
                            12 => {
                                let offset = word_iter.next().unwrap().load::<u16>();
                                inst_vec.push(Instruction::Dsj(Rd((rf as u8) << 4 | rd), Offset(offset)));
                            },
                            13 => {
                                let offset = word_iter.next().unwrap().load::<u16>();
                                inst_vec.push(Instruction::Dsjeq(Rd((rf as u8) << 4 | rd), Offset(offset)));
                            },
                            14 => {
                                let offset = word_iter.next().unwrap().load::<u16>();
                                inst_vec.push(Instruction::Dsjne(Rd((rf as u8) << 4 | rd), Offset(offset)));
                            },
                            15 => {
                                inst_vec.push(Instruction::Setc);
                            },
                            _ => {}
                        }
                    },
                    0b0000111 => {
                        match subop {
                            8 => {
                                inst_vec.push(Instruction::Pixbltll);
                            },
                            9 => {
                                inst_vec.push(Instruction::Pixbltlxy);
                            },
                            10 => {
                                inst_vec.push(Instruction::Pixbltxyl);
                            },
                            11 => {
                                inst_vec.push(Instruction::Pixbltxyxy);
                            },
                            12 => {
                                inst_vec.push(Instruction::Pixbltbl);
                            },
                            13 => {
                                inst_vec.push(Instruction::Pixbltbxy);
                            },
                            14 => {
                                inst_vec.push(Instruction::Filll);
                            },
                            15 => {
                                inst_vec.push(Instruction::Fillxy);
                            },
                            _ => {},
                        }
                    },
                    0b0001000 | 0b0001001 => {
                        if k == 1 {
                            inst_vec.push(Instruction::Inc(Rd((rf as u8) << 4 | rd)));
                        }
                        else {
                            inst_vec.push(Instruction::Addk(K(k), Rd((rf as u8) << 4 | rd)));
                        }
                    },
                    0b0001010 | 0b0001011 => {
                        if k == 1 {
                            inst_vec.push(Instruction::Dec(Rd((rf as u8) << 4 | rd)));
                        }
                        else {
                            inst_vec.push(Instruction::Subk(K(k), Rd((rf as u8) << 4 | rd)));
                        }
                    },
                    0b0001100 | 0b0001101 => {
                        inst_vec.push(Instruction::Movk(K(k), Rd((rf as u8) << 4 | rd))); 
                    },
                    0b0001110 | 0b0001111 => {
                        todo!("BTSTK - don't feel like dealing with 1's complement right now");       
                    },
                    0b0010000 | 0b0010001 => {
                        inst_vec.push(Instruction::Slak(K(k), Rd((rf as u8) << 4 | rd)));
                    },
                    0b0010010 | 0b0010011 => {
                        inst_vec.push(Instruction::Sllk(K(k), Rd((rf as u8) << 4 | rd)));
                    },
                    0b0010100 | 0b0010101 => {
                        todo!("SRAK - don't feel like dealing with 2's complement right now");
                    },
                    0b0010110 | 0b0010111 => {
                        todo!("SRLK - don't feel like dealing with 2's complement right now");
                    },
                    0b0011000 | 0b0011001 => {
                        inst_vec.push(Instruction::Rlk(K(k), Rd((rf as u8) << 4 | rd)));
                    },
                    0b0011100..=0b0011111 => {
                        inst_vec.push(Instruction::Dsjs(D(d), Rd((rf as u8) << 4 | rd), K(k))); 
                    },
                    0b0100000 => {
                        inst_vec.push(Instruction::Add(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b0100001 => {
                        inst_vec.push(Instruction::Addc(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b0100010 => {
                        inst_vec.push(Instruction::Sub(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b0100011 => {
                        inst_vec.push(Instruction::Subb(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b0100100 => {
                        inst_vec.push(Instruction::Cmp(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b0100101 => {
                        inst_vec.push(Instruction::Btst(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b0100110 | 0b0100111 => {
                        inst_vec.push(Instruction::MoveReg(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd), M(f)));
                    },
                    0b0101000 => {
                        inst_vec.push(Instruction::And(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b0101001 => {
                        inst_vec.push(Instruction::Andn(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b0101010 => {
                        inst_vec.push(Instruction::Or(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b0101011 => {
                        if rs == rd {
                            inst_vec.push(Instruction::Clr(Rd((rf as u8) << 4 | rd)));
                        }
                        else {
                            inst_vec.push(Instruction::Xor(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                        }
                    },
                    0b0101100 => {
                        inst_vec.push(Instruction::Divs(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b0101101 => {
                        inst_vec.push(Instruction::Divu(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b0101110 => {
                        inst_vec.push(Instruction::Mpys(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b0101111 => {
                        inst_vec.push(Instruction::Mpyu(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b0110001 => {
                        inst_vec.push(Instruction::Sll(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b0110010 => {
                        inst_vec.push(Instruction::Sra(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b0110011 => {
                        inst_vec.push(Instruction::Srl(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b0110100 => {
                        inst_vec.push(Instruction::Rl(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b0110101 => {
                        inst_vec.push(Instruction::Lmo(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b0110110 => {
                        inst_vec.push(Instruction::Mods(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b0110111 => {
                        inst_vec.push(Instruction::Modu(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b1000000 | 0b1000001 => {
                        inst_vec.push(Instruction::MoveFieldRegToIndirect(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd), Some(F(f))));
                    },
                    0b1000010 | 0b1000011 => {
                        inst_vec.push(Instruction::MoveFieldIndirectToReg(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd), Some(F(f))));
                    },
                    0b1000100 | 0b1000101 => {
                        inst_vec.push(Instruction::MoveFieldIndirectToIndirect(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd), Some(F(f))));
                    },
                    0b1000110 => {
                        inst_vec.push(Instruction::MovbRegToIndirect(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b1000111 => {
                        inst_vec.push(Instruction::MovbIndirectToReg(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b1001000 | 0b1001001 => {
                        inst_vec.push(Instruction::MoveFieldRegToIndirectPostinc(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd), Some(F(f))));
                    },
                    0b1001010 | 0b1001011 => {
                        inst_vec.push(Instruction::MoveFieldIndirectPostincToReg(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd), Some(F(f))));
                    },
                    0b1001100 | 0b1001101 => {
                        inst_vec.push(Instruction::MoveFieldIndirectToIndirectPostinc(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd), Some(F(f))));
                    },
                    0b1001110 => {
                        inst_vec.push(Instruction::MovbIndirectToIndirect(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b1010000 | 0b1010001 => {
                        inst_vec.push(Instruction::MoveFieldRegToIndirectPredec(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd), Some(F(f))));
                    },
                    0b1010010 | 0b1010011 => {
                        inst_vec.push(Instruction::MoveFieldIndirectPredecToReg(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd), Some(F(f))));
                    },
                    0b1010110 => {
                        let offset = word_iter.next().unwrap().load::<u16>();
                        inst_vec.push(Instruction::MovbRegToIndirectOffset(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd), Offset(offset)));
                    },
                    0b1010111 => {
                        let offset = word_iter.next().unwrap().load::<u16>();
                        inst_vec.push(Instruction::MovbIndirectOffsetToReg(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd), Offset(offset)));
                    },
                    0b1011000 | 0b1011001 => {
                        let offset = word_iter.next().unwrap().load::<u16>();
                        inst_vec.push(Instruction::MoveFieldRegToIndirectOffset(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd), Some(F(f)), Offset(offset)));
                    },
                    0b1011010 | 0b1011011 => {
                        let offset = word_iter.next().unwrap().load::<u16>();
                        inst_vec.push(Instruction::MoveFieldIndirectOffsetToReg(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd), Some(F(f)), Offset(offset)));
                    },
                    0b1011100 | 0b1011101 => {
                        let src_offset = word_iter.next().unwrap().load::<u16>();
                        let dst_offset = word_iter.next().unwrap().load::<u16>();
                        inst_vec.push(Instruction::MoveFieldIndirectOffsetToIndirectOffset(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd), Some(F(f)), Offset(src_offset), Offset(dst_offset)));
                    },
                    0b1011110 => {
                        let src_offset = word_iter.next().unwrap().load::<u16>();
                        let dst_offset = word_iter.next().unwrap().load::<u16>();
                        inst_vec.push(Instruction::MovbIndirectOffsetToIndirectOffset(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd), Offset(src_offset), Offset(dst_offset)));
                    },
                    0b1100000..=0b1100111 => {
                        todo!("JA or JR")
                    },
                    0b1101000 | 0b1101001 => {
                        let offset = word_iter.next().unwrap().load::<u16>();
                        inst_vec.push(Instruction::MoveFieldIndirectOffsetToIndirectPostinc(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd), Some(F(f)), Offset(offset)));
                    },
                    0b1101010 | 0b1101011 => {
                        match subop {
                            0 => {
                                let mut address: u32 = 0;
                                let lsb = word_iter.next().unwrap().load::<u16>();
                                let msb = word_iter.next().unwrap().load::<u16>();
                                address.view_bits_mut::<Lsb0>()[0..=15].store::<u16>(lsb);
                                address.view_bits_mut::<Lsb0>()[16..=31].store::<u16>(msb);
                                inst_vec.push(Instruction::MoveFieldAbsoluteToIndirectPostinc(Address(address), Rd((rf as u8) << 4 | rd), Some(F(f))));
                            },
                            8 => {
                                inst_vec.push(Instruction::Exgf(Rd((rf as u8) << 4 | rd), Some(F(f))));
                            },
                            _ => {}
                        }
                    },
                    0b1101111 => {
                        inst_vec.push(Instruction::Line(Z(z)));
                    },
                    0b1110000 => {
                        inst_vec.push(Instruction::Addxy(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b1110001 => {
                        inst_vec.push(Instruction::Subxy(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b1110010 => {
                        inst_vec.push(Instruction::Cmpxy(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b1110011 => {
                        inst_vec.push(Instruction::Cpw(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b1110100 => {
                        inst_vec.push(Instruction::Cvxyl(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b1110110 => {
                        inst_vec.push(Instruction::Movx(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b1110111 => {
                        inst_vec.push(Instruction::Movy(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b1111000 => {
                        inst_vec.push(Instruction::PixtRegToIndirectxy(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b1111001 => {
                        inst_vec.push(Instruction::PixtIndirectxyToReg(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b1111010 => {
                        inst_vec.push(Instruction::PixtIndirectxytoIndirectxy(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b1111011 => {
                        inst_vec.push(Instruction::Drav(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b1111100 => {
                        inst_vec.push(Instruction::PixtRegToIndirect(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b1111101 => {
                        inst_vec.push(Instruction::PixtIndirectToReg(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
                    },
                    0b1111110 => {
                        inst_vec.push(Instruction::PixtIndirectToIndirect(Rs((rf as u8) << 4 | rs), Rd((rf as u8) << 4 | rd)));
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
