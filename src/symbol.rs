#[derive(Debug, Clone, Copy)]
pub struct Rs(pub u8);
#[derive(Debug, Clone, Copy)]
pub struct Rd(pub u8);
#[derive(Debug, Clone, Copy)]
pub struct IW(pub u16);
#[derive(Debug, Clone, Copy)]
pub struct IL(pub u32);
#[derive(Debug, Clone, Copy)]
pub struct K(pub u8);
#[derive(Debug, Clone, Copy)]
pub struct F(pub bool);
#[derive(Debug, Clone, Copy)]
pub struct Address(pub u32); // 32-bit specified in assembly code, turned into 8-bit offset from current PC
#[derive(Debug, Clone, Copy)]
pub struct FS(pub u8);
#[derive(Debug, Clone, Copy)]
pub struct FE(pub bool);
#[derive(Debug, Clone, Copy)]
pub struct N(pub u8);
#[derive(Debug, Clone, Copy)]
pub struct Offset8(pub u8);
#[derive(Debug, Clone, Copy)]
pub struct Offset(pub u16);
#[derive(Debug, Clone, Copy)]
pub struct Z(pub bool);
#[derive(Debug, Clone, Copy)]
pub struct Condition(pub u8);
#[derive(Debug, Clone, Copy)]
pub struct D(pub bool);
#[derive(Debug, Clone, Copy)]
pub struct M(pub bool);
