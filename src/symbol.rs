//! TMD34010 assembler symbols.
//!
//! This module uses the notation described in the User's Guide whenever possible.

use core::fmt::{self, Formatter};

/// Rs is the source register for an instruction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rs(pub u8);
impl fmt::Display for Rs {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        if self.0 == 15 || self.0 == 31 {
            write!(fmt, "SP")
        } else if self.0 <= 14 {
            write!(fmt, "A{}", self.0)
        } else {
            write!(fmt, "B{}", self.0 - 16)
        }
    }
}

/// Rd is the destination register for an instruction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rd(pub u8);
impl fmt::Display for Rd {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        if self.0 == 15 || self.0 == 31 {
            write!(fmt, "SP")
        } else if self.0 <= 14 {
            write!(fmt, "A{}", self.0)
        } else {
            write!(fmt, "B{}", self.0 - 16)
        }
    }
}

/// IW is a 16-bit immediate value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IW(pub u16);
impl fmt::Display for IW {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{:X}h", self.0)
    }
}

/// IL is a 32-bit immediate value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IL(pub u32);
impl fmt::Display for IL {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{:X}h", self.0)
    }
}

/// K is a 5-bit constant.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct K(pub u8);
impl fmt::Display for K {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.0)
    }
}

/// F is the field select parameter for `MOVE` instructions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct F(pub bool);
impl fmt::Display for F {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.0 as u8)
    }
}

/// FS indicates the field size.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FS(pub u8);
impl fmt::Display for FS {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        if self.0 == 0 {
            write!(fmt, "32")
        } else {
            write!(fmt, "{}", self.0)
        }
    }
}

/// FE indicates the field extension.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FE(pub bool);
impl fmt::Display for FE {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.0 as u8)
    }
}

/// N is a general unit count.
///
/// It's used in the `RETS` and `TRAP` instructions, where it means "additional words to add to the
/// stack pointer" and "trap number", respectively.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct N(pub u8);
impl fmt::Display for N {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Offset8(pub u8);
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Offset(pub u16);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Address(pub u32);
impl fmt::Display for Address {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{:X}h", self.0)
    }
}

/// Z is the LINE algorithm selection bit.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Z(pub bool);
impl fmt::Display for Z {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.0 as u8)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Condition(pub u8);
impl fmt::Display for Condition {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let cond_str = match self.0 {
            // there's some dupes in here I'm not sure how to deal with
            // need a better understanding of when it's one or the other
            0b0000 => "UC",
            0b0001 => "LO", // also P
            0b0010 => "LS",
            0b0011 => "HI",
            0b0100 => "LT",
            0b0101 => "GE", // also Z
            0b0110 => "LE",
            0b0111 => "GT",
            0b1000 => "B",
            0b1001 => "HS", // also NB/NC
            0b1010 => "EQ",
            0b1011 => "NE",
            0b1100 => "V",
            0b1101 => "NV",
            0b1110 => "N",
            0b1111 => "NN",
            _ => "",
        };
        write!(fmt, "{}", cond_str)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct D(pub bool);
impl fmt::Display for D {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.0 as u8)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct M(pub bool);

// Thought about writing a Display trait for this but unfortunately it relies
// on outside information, the rf of the rd register in the instruction this is
// used in determines which  register letter should be shown
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RegList(pub u16);

// this is a bit of a hack to support instructions with offsets relative to PC
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PC(pub u32);
