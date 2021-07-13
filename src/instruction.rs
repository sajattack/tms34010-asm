enum Instruction {
    // Arithmetic/Logical/Comparison
    Abs(u8),            // Rd
    Add(u8, u8),        // Rs, Rd
    Addc(u8, u8),       // Rs, Rd
    Addiw(u16, u8),     // IW, Rd
    Addil(u32, u8),     // IL, Rd
    Addk(u8, u8),       // K, Rd
    Addxy(u8, u8),      // Rs, Rd
    And(u8, u8),        // Rs, Rd
    Andi(u32, u8),      // IL, Rd
    Andn(u8, u8),       // Rs, Rd
    Andni(u32, u8),     // IL, Rd
    Btstk(u8, u8),      // K, Rd
    Btst(u8, u8),       // Rs, Rd
    Clr(u8),            // Rd
    Clrc,
    Cmp(u8, u8),        // Rs, Rd
    Cmpiw(u16, u8),     // IW, Rd 
    Cmpil(u32, u8),     // IL, Rd
    Cmpxy(u8, u8),      // Rs, Rd
    Dec(u8),            // Rd
    Divs(u8, u8),       // Rs, Rd
    Divu(u8, u8),       // Rs, Rd
    Lmo(u8, u8),        // Rs, Rd
    Mods(u8, u8),       // Rs, Rd
    Modu(u8, u8),       // Rs, Rd
    Mpys(u8, u8),       // Rs, Rd
    Mpyu(u8, u8),       // Rs, Rd
    Neg(u8),            // Rd
    Negb(u8),           // Rd
    Not(u8),            // Rd
    Or(u8, u8),         // Rs, Rd
    Ori(u32, u8),       // IL, Rd
    Setc,
    Sext(u8, u8),       // Rd, F
    Sub(u8, u8),        // Rs, Rd
    Subb(u8, u8),       // Rs, Rd
    Subiw(u16, u8),     // IW, Rd
    Subil(u32, u8),     // IL, Rd
    Subk(u8, u8),       // K, Rd
    Subxy(u8, u8),      // Rs, Rd
    Xor(u8, u8),        // Rs, Rd
    Xori(u32, u8),      // IL, Rd
    Zext(u8, u8),       // Rd, F
    // Move
    // Graphics
    // Control
    // Jump
    // Shift
}
