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
    MoveFieldRegToIndirect(Rs, Rd, F),
    MoveFieldRegToIndirectPredec(Rs, Rd, F),
    MoveFieldRegToIndirectPostinc(Rs, Rd, F),
    MoveFieldIndirectToReg(Rs, Rd, F),
    MoveFieldIndirectPredecToReg(Rs, Rd, F),
    MoveFieldIndirectPostincToReg(Rs, Rd, F),
    MoveFieldIndirectToIndirect(Rs, Rd, F),
    MoveFieldIndirectToIndirectPredec(Rs, Rd, F),
    MoveFieldIndirectToIndirectPostinc(Rs, Rd, F),
    MoveFieldRegToIndirectOffset(Rs, Rd, F, Offset),
    MoveFieldIndirectOffsetToReg(Rs, Rd, F, Offset),
    MoveFieldIndirectOffsetToIndirectPostinc(Rs, Rd, F, Offset),
    MoveFieldIndirectOffsetToIndirectOffset(Rs, Rd, F, Offset, Offset),
    MoveFieldRegToAbsolute(Rs, Address, F),
    MoveFieldAbsoluteToReg(Address, Rd, F),
    MoveFieldAbsoluteToIndirectPostinc(Address, Rd, F),
    MoveFieldAbsoluteToAbsolute(Address, Address, F),
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
    PixtIndirectxyToIndirectxy(Rs, Rd),
    // Control
    Call(Rs),
    Calla(Address),
    Callr(Offset, PC),
    Dint,
    Eint,
    Emu,
    Exgf(Rd, F),
    Exgpc(Rd, F),
    Getpc(Rd),
    Getst(Rd),
    Nop,
    Popst,
    Pushst,
    Putst(Rs),
    Reti,
    Rets(N),
    Rev(Rd),
    Setf(FS, FE, F),
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
    // not actually an instruction, just a convenience for me
    // there's probably a smarter way to do this
    Dw(IW),
}

impl Instruction {
    pub fn get_mnemonic(&self) -> &'static str {
        match self {
            Self::Abs(_) => "ABS",
            Self::Add(_, _) => "ADD",
            Self::Addc(_, _) => "ADDC",
            Self::Addiw(_, _) | Self::Addil(_, _) => "ADDI",
            Self::Addk(_, _) => "ADDK",
            Self::Addxy(_, _) => "ADDXY",
            Self::And(_, _) => "AND",
            Self::Andi(_, _) => "ANDI",
            Self::Andn(_, _) => "ANDN",
            Self::Btstk(_, _) | Self::Btst(_, _) => "BTST",
            Self::Clr(_) => "CLR",
            Self::Clrc => "CLRC",
            Self::Cmp(_, _) => "CMP",
            Self::Cmpiw(_, _) | Self::Cmpil(_, _) => "CMPI",
            Self::Cmpxy(_, _) => "CMPXY",
            Self::Dec(_) => "DEC",
            Self::Divs(_, _) => "DIVS",
            Self::Divu(_, _) => "DIVU",
            Self::Inc(_) => "INC",
            Self::Lmo(_, _) => "LMO",
            Self::Mods(_, _) => "MODS",
            Self::Modu(_, _) => "MODU",
            Self::Mpys(_, _) => "MPYS",
            Self::Mpyu(_, _) => "MPYU",
            Self::Neg(_) => "NEG",
            Self::Negb(_) => "NEGB",
            Self::Not(_) => "NOT",
            Self::Or(_, _) => "OR",
            Self::Ori(_, _) => "ORI",
            Self::Setc => "SETC",
            Self::Sext(_, _) => "SEXT",
            Self::Sub(_, _) => "SUB",
            Self::Subb(_, _) => "SUBB",
            Self::Subk(_, _) => "SUBK",
            Self::Subxy(_, _) => "SUBXY",
            Self::Subil(_, _) | Self::Subiw(_, _) => "SUBI",
            Self::Xor(_, _) => "XOR",
            Self::Xori(_, _) => "XORI",
            Self::Zext(_, _) => "ZEXT",
            Self::MovbRegToIndirect(_, _)
            | Self::MovbIndirectToReg(_, _)
            | Self::MovbIndirectToIndirect(_, _)
            | Self::MovbRegToIndirectOffset(_, _, _)
            | Self::MovbIndirectOffsetToReg(_, _, _)
            | Self::MovbIndirectOffsetToIndirectOffset(_, _, _, _)
            | Self::MovbRegToAbsolute(_, _)
            | Self::MovbAbsoluteToReg(_, _)
            | Self::MovbAbsoluteToAbsolute(_, _) => "MOVB",
            Self::MoveReg(_, _)
            | Self::MoveFieldRegToIndirect(_, _, _)
            | Self::MoveFieldRegToIndirectPredec(_, _, _)
            | Self::MoveFieldRegToIndirectPostinc(_, _, _)
            | Self::MoveFieldIndirectToReg(_, _, _)
            | Self::MoveFieldIndirectPredecToReg(_, _, _)
            | Self::MoveFieldIndirectPostincToReg(_, _, _)
            | Self::MoveFieldIndirectToIndirect(_, _, _)
            | Self::MoveFieldIndirectToIndirectPredec(_, _, _)
            | Self::MoveFieldIndirectToIndirectPostinc(_, _, _)
            | Self::MoveFieldRegToIndirectOffset(_, _, _, _)
            | Self::MoveFieldIndirectOffsetToReg(_, _, _, _)
            | Self::MoveFieldIndirectOffsetToIndirectPostinc(_, _, _, _)
            | Self::MoveFieldIndirectOffsetToIndirectOffset(_, _, _, _, _)
            | Self::MoveFieldRegToAbsolute(_, _, _)
            | Self::MoveFieldAbsoluteToReg(_, _, _)
            | Self::MoveFieldAbsoluteToIndirectPostinc(_, _, _)
            | Self::MoveFieldAbsoluteToAbsolute(_, _, _) => "MOVE",
            Self::Movil(_, _) | Self::Moviw(_, _) => "MOVI",
            Self::Movk(_, _) => "MOVK",
            Self::Movx(_, _) => "MOVX",
            Self::Movy(_, _) => "MOVY",
            Self::Mmtm(_, _) => "MMTM",
            Self::Mmfm(_, _) => "MMFM",
            Self::Cpw(_, _) => "CPW",
            Self::Cvxyl(_, _) => "CVXYL",
            Self::Drav(_, _) => "DRAV",
            Self::Filll => "FILL L",
            Self::Fillxy => "FILL XY",
            Self::Line(_) => "LINE",
            Self::Pixbltbl => "PIXBLT B,L",
            Self::Pixbltbxy => "PIXBLT B,XY",
            Self::Pixbltlxy => "PIXBLT L,XY",
            Self::Pixbltll => "PIXBLT L,L",
            Self::Pixbltxyl => "PIXBLT XY,L",
            Self::Pixbltxyxy => "PIXBLT XY,XY",
            Self::PixtRegToIndirect(_, _) 
            | Self::PixtRegToIndirectxy(_, _)
            | Self::PixtIndirectToReg(_, _)
            | Self::PixtIndirectToIndirect(_, _)
            | Self::PixtIndirectxyToReg(_, _)
            | Self::PixtIndirectxyToIndirectxy(_, _) => "PIXT",
            Self::Call(_) => "CALL",
            Self::Calla(_) => "CALLA",
            Self::Callr(_, _) => "CALLR",
            Self::Dint => "DINT",
            Self::Eint => "EINT",
            Self::Emu => "EMU",
            Self::Exgf(_, _) => "EXGF",
            Self::Exgpc(_, _) => "EXGPC",
            Self::Getpc(_) => "GETPC",
            Self::Getst(_) => "GETST",
            Self::Nop => "NOP",
            Self::Popst => "POPST",
            Self::Pushst => "PUSHST",
            Self::Putst(_) => "PUTST",
            Self::Reti => "RETI",
            Self::Rets(_) => "RETS",
            Self::Rev(_) => "REV",
            Self::Setf(_, _, _) => "SETF",
            Self::Trap(_) => "TRAP",
            Self::Dsj(_, _) => "DSJ",
            Self::Dsjeq(_, _) => "DSJEQ",
            Self::Dsjne(_, _) => "DSJNE",
            Self::Dsjs(_, _, _, _) => "DSJS",
            Self::Ja(_, _) => "JA",
            Self::Jr(_, _, _) => "JR",
            Self::Jrs(_, _) => "JRS",
            Self::Jump(_) => "JUMP",
            Self::Rlk(_, _) => "RLK",
            Self::Rl(_, _) => "RL",
            Self::Slak(_, _) => "SLAK",
            Self::Sla(_, _) => "SLA",
            Self::Sllk(_, _) => "SLLK",
            Self::Sll(_, _) => "SLL",
            Self::Srak(_, _) => "SRAK",
            Self::Sra(_, _) => "SRA",
            Self::Srlk(_, _) => "SRLK",
            Self::Srl(_, _) => "SRL",
            Self::Dw(_) => "DW",
        }
    }
}
