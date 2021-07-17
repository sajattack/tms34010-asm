#[derive(Debug, Clone, Copy)]
pub struct Rs(u8);
#[derive(Debug, Clone, Copy)]
pub struct Rd(u8);
#[derive(Debug, Clone, Copy)]
pub struct IW(u16);
#[derive(Debug, Clone, Copy)]
pub struct IL(u32);
#[derive(Debug, Clone, Copy)]
pub struct K(u8);
#[derive(Debug, Clone, Copy)]
pub struct F(u8);
#[derive(Debug, Clone, Copy)]
pub struct Address(u32); // 32-bit specified in assembly code, turned into 8-bit offset from current PC
#[derive(Debug, Clone, Copy)]
pub struct FS(u8);
#[derive(Debug, Clone, Copy)]
pub struct FE(u8);
#[derive(Debug, Clone, Copy)]
pub struct N(u8);
#[derive(Debug, Clone, Copy)]
pub struct Offset(u8);
#[derive(Debug, Clone, Copy)]
pub struct Z(u8);
#[derive(Debug, Clone, Copy)]
pub struct Condition(u8);

