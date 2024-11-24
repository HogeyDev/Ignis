use std::collections::HashMap;



#[derive(Debug)]
pub enum AsmInstruction {
    // Singles are like: "inc rax"
    // Doubles are like: "mov rax, 10"
    BlankSingle(&'static str, String),
    BlankDouble(&'static str, String, String),
    Add(AsmLocation, AsmLocation),
    Call(String),
    Compare(AsmLocation, AsmLocation),
    Divide(AsmLocation, AsmLocation),
    Global(AsmLocation),
    IntegerMultiply(AsmLocation, AsmLocation),
    Increment(AsmLocation),
    JumpEqual(AsmLocation),
    Jump(AsmLocation),
    JumpNotEqual(AsmLocation),
    LoadEffectiveAddress(AsmLocation, AsmLocation),
    Move(AsmLocation, AsmLocation),
    MoveZeroExtend(AsmLocation, AsmLocation),
    Negate(AsmLocation),
    Pop(AsmLocation),
    Push(AsmLocation),
    Return,
    SetEqual(AsmLocation),
    SetGreaterThan(AsmLocation),
    SetLessThan(AsmLocation),
    SetLessEqual(AsmLocation),
    Subtract(AsmLocation, AsmLocation),
    SysCall(AsmLocation),
}

#[derive(Debug)]
pub enum AsmLocation {
    Blank(String),
    Value(String),
    Memory(String, String),
    Label(String),
    SubRoutine(String),
    Register(String),
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub enum AsmSection {
    None,
    Text,
    Data,
}

pub struct AsmBlock {
    pub sections: HashMap<AsmSection, Vec<AsmInstruction>>,
}

impl AsmBlock {
    pub fn new() -> Self {
        Self {
            sections: HashMap::new(),
        }
    }
    pub fn push(&mut self, section: AsmSection, inst: AsmInstruction) {
        if !self.sections.contains_key(&section) {
            self.sections.insert(section, Vec::new());
        }
        self.sections.get_mut(&section).unwrap().push(inst);
    }
    pub fn push_block(&mut self, block: AsmBlock) {
        for (section, instructions) in block.sections {
            for inst in instructions {
                self.push(section, inst);
            }
        }
    }
}
