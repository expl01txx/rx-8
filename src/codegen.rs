use crate::parser::Instructions;

pub struct VMInstruction {
    opcode: u8,
    r0: u8,
    r1: u8,
}

impl VMInstruction {
    pub fn new(opcode: u8, r0: u8, r1: u8) -> VMInstruction {
        return VMInstruction {
            opcode,
            r0,
            r1,
        };
    }

    pub fn from(inst: u8) -> VMInstruction {
        VMInstruction {
            opcode: (inst >> 4) & 0b1111,
            r0: (inst >> 2) & 0b11,
            r1: inst & 0b11,
        }
    }

    pub fn encode(self) -> u8 {
        (self.opcode << 4) | (self.r0 << 2) | self.r1
    }
}

pub struct Codegen {
    inst: Vec<Instructions>,
}

impl Codegen {
    pub fn new(inst: Vec<Instructions>) -> Codegen {
        Codegen {
            inst,
        }
    }

    pub fn generate(&mut self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        for instruction in &self.inst {
            match instruction {
                Instructions::Mov { dest, source } => {
                    let vm_inst = VMInstruction::new(0b001, *dest as u8, 0).encode();
                    bytes.push(vm_inst);
                    bytes.push((*source as u8).to_le());
                }
                Instructions::Add { dest, source } => {
                    let vm_inst = VMInstruction::new(0b0010, *dest as u8, *source as u8).encode();
                    bytes.push(vm_inst);
                }
                Instructions::Sub { dest, source } => {
                    let vm_inst = VMInstruction::new(0b0011, *dest as u8, *source as u8).encode();
                    bytes.push(vm_inst);
                }
                Instructions::Mult { dest, source } => {
                    let vm_inst = VMInstruction::new(0b0100, *dest as u8, *source as u8).encode();
                    bytes.push(vm_inst);
                }
                Instructions::Div { dest, source } => {
                    let vm_inst = VMInstruction::new(0b0101, *dest as u8, *source as u8).encode();
                    bytes.push(vm_inst);
                }
                Instructions::Xor { dest, source } => {
                    let vm_inst = VMInstruction::new(0b0111, *dest as u8, *source as u8).encode();
                    bytes.push(vm_inst);
                }
                Instructions::Cmp { dest, source } => {
                    let vm_inst = VMInstruction::new(0b1000, *dest as u8, *source as u8).encode();
                    bytes.push(vm_inst);
                }
                Instructions::Out { source } => {
                    let vm_inst = VMInstruction::new(0b1101, *source as u8, 0).encode();
                    bytes.push(vm_inst);
                }
                Instructions::In { dest } => {
                    let vm_inst = VMInstruction::new(0b1110, *dest as u8, 0).encode();
                    bytes.push(vm_inst);
                }
                Instructions::Hlt => {
                    bytes.push(0xff);
                }
            }
        }

        bytes
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn codegen_test1(){
        let insts = vec![
            Instructions::Mov { dest: 1, source: 1 },
            Instructions::Add { dest: 1, source: 1 },
            Instructions::Sub { dest: 1, source: 1 },
            Instructions::Mult { dest: 1, source: 1 },
            Instructions::Div { dest: 1, source: 1 },
            Instructions::Xor { dest: 1, source: 1 },
            Instructions::Cmp { dest: 1, source: 1 },
            Instructions::Out { source: 1 },
            Instructions::In { dest: 1 },
            Instructions::Hlt
        ];
        let mut codegen = Codegen::new(insts);
        codegen.generate();
    }

    #[test]
    fn codegen_test2(){
        let insts = vec![];
        let mut codegen = Codegen::new(insts);
        codegen.generate();
    }

    #[test]
    fn codegen_test3(){
        let insts = vec![
            Instructions::Mov { dest: 1024, source: 768 },
            Instructions::Add { dest: 512, source: 1024 },
            Instructions::Sub { dest: 768, source: 512 },
            Instructions::Mult { dest: 1024, source: 768 },
            Instructions::Div { dest: 512, source: 1024 },
            Instructions::Xor { dest: 768, source: 512 },
            Instructions::Cmp { dest: 1024, source: 768 },
            Instructions::Out { source: 512 },
            Instructions::In { dest: 1024 },
            Instructions::Hlt
        ];
        let mut codegen = Codegen::new(insts);
        codegen.generate();
    }
    
    #[test]
    fn vminst_test1(){
        let inst = VMInstruction::new(0, 0, 0).encode();
        assert_eq!(inst, 0);
    }

    #[test]
    fn vminst_test2(){
        let inst = VMInstruction::new(128, 128, 128).encode();
        assert_eq!(inst, 0x80);
    }

    #[test]
    fn vminst_test3(){
        let inst = VMInstruction::new(255, 255, 255).encode();
        assert_eq!(inst, 0xFF)
    }

    #[test]
    fn vminst_test4(){
        let inst = VMInstruction::new(16, 8, 32).encode();
        assert_eq!(inst, 0x20)
    }
}