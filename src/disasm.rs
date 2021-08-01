use bitvec::prelude::*;

use crate::instruction::Instruction;

#[allow(unused)]
use crate::symbol::{Rs, Rd, IW, IL, K, F, D, Address, FS, FE, N, Offset8, Offset16, Z, Condition};

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
                                inst_vec.push(Instruction::Setf(FS(fs), FE(fe), F(f)));
                            }
                            _ => { /*todo!("bunch of move instructions I don't want to deal with right now")*/}
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
                            _ => { /*todo!("bunch of move instructions I don't want to deal with right now")*/}
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
                                inst_vec.push(Instruction::Callr(Offset16(offset)));
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
                                inst_vec.push(Instruction::Dsj(Rd((rf as u8) << 4 | rd), Offset16(offset)));
                            },
                            13 => {
                                let offset = word_iter.next().unwrap().load::<u16>();
                                inst_vec.push(Instruction::Dsjeq(Rd((rf as u8) << 4 | rd), Offset16(offset)));
                            },
                            14 => {
                                let offset = word_iter.next().unwrap().load::<u16>();
                                inst_vec.push(Instruction::Dsjne(Rd((rf as u8) << 4 | rd), Offset16(offset)));
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
                        //todo!("BTSTK - don't feel like dealing with 1's complement right now");       
                    },
                    0b0010000 | 0b0010001 => {
                        inst_vec.push(Instruction::Slak(K(k), Rd((rf as u8) << 4 | rd)));
                    },
                    0b0010010 | 0b0010011 => {
                        inst_vec.push(Instruction::Sllk(K(k), Rd((rf as u8) << 4 | rd)));
                    },
                    0b0010100 | 0b0010101 => {
                        //todo!(SRAK - don't feel like dealing with 2's complement right now");
                    },
                    0b0010110 | 0b0010111 => {
                        //todo!(SRLK - don't feel like dealing with 2's complement right now");
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
                         //todo!("move instruction I don't want to deal with right now")
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
