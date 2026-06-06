pub mod disasm;
pub mod instruction;
pub mod symbol;

#[cfg(test)]
mod tests {
    use super::*;
    use disasm::Dis;
    use instruction::Instruction;

    #[test]
    fn disasm() {
        struct TestCase {
            input: Vec<u8>,
            want: Vec<Dis>,
        }
        let table: Vec<TestCase> = vec![TestCase {
            input: vec![0b0010_0000, 0b0000_0011],
            want: vec![Dis(0x00, Instruction::Clrc, vec![0x0320])],
        }];
        for tc in table {
            let got = disasm::disassemble_stage1(&tc.input, 0x00);
            assert_eq!(got, tc.want);
        }
    }
}
