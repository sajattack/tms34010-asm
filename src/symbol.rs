use core::fmt::{self,Formatter};

#[derive(Debug, Clone, Copy)]
pub struct Rs(pub u8);
impl fmt::Display for Rs {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let rfchar = if self.0 <= 14 {'A'} else if self.0 >= 16 && self.0 < 30 {'B'} else {'\0'};
        if self.0 == 15 || self.0 == 31 {
            write!(fmt, "SP")
        } else {
            write!(fmt, "{}{}", rfchar, self.0)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rd(pub u8);
impl fmt::Display for Rd {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let rfchar = if self.0 <= 14 {'A'} else if self.0 >= 16 && self.0 < 30 {'B'} else {'\0'};
        if self.0 == 15 || self.0 == 31 {
            write!(fmt, "SP")
        } else {
            write!(fmt, "{}{}", rfchar, self.0)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IW(pub u16);
#[derive(Debug, Clone, Copy)]
pub struct IL(pub u32);
#[derive(Debug, Clone, Copy)]

pub struct K(pub u8);
impl fmt::Display for K {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        if self.0 == 0
        {
            write!(fmt, "32")
        } else
        {
            write!(fmt, "{}", self.0)
        }
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
        write!(fmt, "{:x}h", self.0)
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
#[derive(Debug, Clone, Copy)]
pub struct Offset8(pub u8);
#[derive(Debug, Clone, Copy)]
pub struct Offset(pub u16);
#[derive(Debug, Clone, Copy)]
pub struct Address(pub u32);
#[derive(Debug, Clone, Copy)]
pub struct Z(pub bool);
#[derive(Debug, Clone, Copy)]
pub struct Condition(pub u8);
#[derive(Debug, Clone, Copy)]
pub struct D(pub bool);
#[derive(Debug, Clone, Copy)]
pub struct M(pub bool);
#[derive(Debug, Clone, Copy)]
pub struct RegList(pub u16);
