use core::fmt::{self, Formatter};

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub struct IW(pub u16);
impl fmt::Display for IW {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{:X}h", self.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IL(pub u32);
impl fmt::Display for IL {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{:X}h", self.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct K(pub u8);
impl fmt::Display for K {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.0)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct F(pub bool);
impl fmt::Display for F {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.0 as u8)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FS(pub u8);
impl fmt::Display for FS {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{:X}h", self.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FE(pub bool);
impl fmt::Display for FE {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.0 as u8)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct N(pub u8);
impl fmt::Display for N {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Offset8(pub u8);
#[derive(Debug, Clone, Copy)]
pub struct Offset(pub u16);

#[derive(Debug, Clone, Copy)]
pub struct Address(pub u32);
impl fmt::Display for Address {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{:X}h", self.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Z(pub bool);

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub struct D(pub bool);
impl fmt::Display for D {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.0 as u8)
    }
}


#[derive(Debug, Clone, Copy)]
pub struct M(pub bool);
#[derive(Debug, Clone, Copy)]
pub struct RegList(pub u16);

// this is a bit of a hack to support instructions with offsets relative to PC
#[derive(Debug, Clone, Copy)]
pub struct PC(pub u32);
