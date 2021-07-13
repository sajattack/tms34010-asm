use crate::symbol::{Rs, Rd, IW, IL, K, F};

enum Instruction {
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
    Andni(IL, Rd),
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
    // Graphics
    // Control
    // Jump
    // Shift
}
