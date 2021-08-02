use crate::symbol::{
    Address, Condition, /*M,*/ Offset, Offset8, Rd, RegList, Rs, D, F, FE, FS, IL, IW, K, N,
    PC, Z,
};

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    // Arithmetic/Logical/Comparison
    Abs(Rd),
    Add(Rs, Rd),
    Addc(Rs, Rd),
    Addiw(IW, Rd),
    Addil(IL, Rd),
    Addk(K, Rd),
    Addxy(Rs, Rd),
    And(Rs, Rd),
    Andi(IL, Rd),
    Andn(Rs, Rd),
    //Andni(IL, Rd),  somehow this is an alias of andi?
    Btstk(K, Rd),
    Btst(Rs, Rd),
    Clr(Rd),
    Clrc,
    Cmp(Rs, Rd),
    Cmpiw(IW, Rd),
    Cmpil(IL, Rd),
    Cmpxy(Rs, Rd),
    Dec(Rd),
    Divs(Rs, Rd),
    Divu(Rs, Rd),
    Inc(Rd),
    Lmo(Rs, Rd),
    Mods(Rs, Rd),
    Modu(Rs, Rd),
    Mpys(Rs, Rd),
    Mpyu(Rs, Rd),
    Neg(Rd),
    Negb(Rd),
    Not(Rd),
    Or(Rs, Rd),
    Ori(IL, Rd),
    Setc,
    Sext(Rd, F),
    Sub(Rs, Rd),
    Subb(Rs, Rd),
    Subiw(IW, Rd),
    Subil(IL, Rd),
    Subk(K, Rd),
    Subxy(Rs, Rd),
    Xor(Rs, Rd),
    Xori(IL, Rd),
    Zext(Rd, F),
    // Move
    MovbRegToIndirect(Rs, Rd),
    MovbIndirectToReg(Rs, Rd),
    MovbIndirectToIndirect(Rs, Rd),
    MovbRegToIndirectOffset(Rs, Rd, Offset),
    MovbIndirectOffsetToReg(Rs, Rd, Offset),
    MovbIndirectOffsetToIndirectOffset(Rs, Rd, Offset, Offset),
    MovbRegToAbsolute(Rs, Address),
    MovbAbsoluteToReg(Address, Rd),
    MovbAbsoluteToAbsolute(Address, Address),
    MoveReg(Rs, Rd /*M*/), // might need M for asm later but we definitely don't need it for disasm
    MoveFieldRegToIndirect(Rs, Rd, Option<F>),
    MoveFieldRegToIndirectPredec(Rs, Rd, Option<F>),
    MoveFieldRegToIndirectPostinc(Rs, Rd, Option<F>),
    MoveFieldIndirectToReg(Rs, Rd, Option<F>),
    MoveFieldIndirectPredecToReg(Rs, Rd, Option<F>),
    MoveFieldIndirectPostincToReg(Rs, Rd, Option<F>),
    MoveFieldIndirectToIndirect(Rs, Rd, Option<F>),
    MoveFieldIndirectToIndirectPredec(Rs, Rd, Option<F>),
    MoveFieldIndirectToIndirectPostinc(Rs, Rd, Option<F>),
    MoveFieldRegToIndirectOffset(Rs, Rd, Option<F>, Offset),
    MoveFieldIndirectOffsetToReg(Rs, Rd, Option<F>, Offset),
    MoveFieldIndirectOffsetToIndirectPostinc(Rs, Rd, Option<F>, Offset),
    MoveFieldIndirectOffsetToIndirectOffset(Rs, Rd, Option<F>, Offset, Offset),
    MoveFieldRegToAbsolute(Rs, Address, Option<F>),
    MoveFieldAbsoluteToReg(Address, Rd, Option<F>),
    MoveFieldAbsoluteToIndirectPostinc(Address, Rd, Option<F>),
    MoveFieldAbsoluteToAbsolute(Address, Address, Option<F>),
    Moviw(IW, Rd),
    Movil(IL, Rd),
    Movk(K, Rd),
    Movx(Rs, Rd),
    Movy(Rs, Rd),
    Mmtm(Rd, RegList),
    Mmfm(Rs, RegList),
    // Graphics
    Cpw(Rs, Rd),
    Cvxyl(Rs, Rd),
    Drav(Rs, Rd),
    Filll,
    Fillxy,
    Line(Z),
    Pixbltbl,
    Pixbltbxy,
    Pixbltll,
    Pixbltlxy,
    Pixbltxyl,
    Pixbltxyxy,
    PixtRegToIndirect(Rs, Rd),
    PixtRegToIndirectxy(Rs, Rd),
    PixtIndirectToReg(Rs, Rd),
    PixtIndirectToIndirect(Rs, Rd),
    PixtIndirectxyToReg(Rs, Rd),
    PixtIndirectxytoIndirectxy(Rs, Rd),
    // Control
    Call(Rs),
    Calla(Address),
    Callr(Offset, PC),
    Dint,
    Eint,
    Emu,
    Exgf(Rd, Option<F>),
    Exgpc(Rd),
    Getpc(Rd),
    Getst(Rd),
    Nop,
    Popst,
    Pushst,
    Putst(Rs),
    Reti,
    Rets(N),
    Rev(Rd),
    Setf(FS, FE, Option<F>),
    Trap(N),
    // Jump
    Dsj(Rd, Offset),
    Dsjeq(Rd, Offset),
    Dsjne(Rd, Offset),
    Dsjs(D, Rd, K, PC), // the manual calls this offset rather than K but it's in the position of K and 5 bits long
    // it's also in with other K instructions
    Ja(Condition, Address),
    Jr(Condition, Offset8, PC),
    Jrs(Condition, Offset),
    Jump(Rs),
    // Shift
    Rlk(K, Rd),
    Rl(Rs, Rd),
    Slak(K, Rd),
    Sla(Rs, Rd),
    Sllk(K, Rd),
    Sll(Rs, Rd),
    Srak(K, Rd),
    Sra(Rs, Rd),
    Srlk(K, Rd),
    Srl(Rs, Rd),
}
