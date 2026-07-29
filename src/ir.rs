use crate::{analyzer::Analyzer, parser::{Declaration::{self, ParseError}, DeclarationId, RootAst}};

pub type BasicBlockId = usize;
pub struct BasicBlock {
    label: String,
    insts: Vec<Instruction>,

    preds: Vec<BasicBlockId>,
    succs: Vec<BasicBlockId>,
}
impl BasicBlock {
    pub fn new(label: String) -> Self {
        Self {
            label,
            insts: Vec::new(),

            preds: Vec::new(),
            succs: Vec::new(),
        }
    }
}

pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Xor,
    LShift,
    RShift,
    LNot,
    GT,
    GTE,
    LT,
    LTE,
    Eq,
    NotEq,
}
pub enum UnOp {}

pub enum Instruction {
    BinExp {
        dest: String,
        op: BinOp,
    }
}

pub struct IrBuilder<'a> {
    analyzer: &'a Analyzer<'a>,
    pub block_arena: Vec<BasicBlock>,

    label_counter: usize,
}

impl<'a> IrBuilder<'a> {
    pub fn new(analyzer: &'a Analyzer) -> Self {
        Self {
            analyzer,
            block_arena: Vec::new(),

            label_counter: 0,
        }
    }

    fn numeric_label(&mut self) -> String {
        self.label_counter += 1;
        format!("label_{}", self.label_counter-1)
    }
    fn new_block(&mut self, label: String) -> BasicBlockId {
        self.block_arena.push(BasicBlock::new(label));
        self.block_arena.len() - 1
    }

    pub fn run(&mut self, ast: &RootAst) {
        self.new_block(String::from("entrypoint"));

        for decl_id in ast {
            self.declaration(*decl_id);
        }
    }

    fn declaration(&mut self, decl_id: DeclarationId) {
        let decl = &self.analyzer.parser.declaration_arena[decl_id];
        match decl {
            Declaration::ParseError => unreachable!(),
            Declaration::Function { name, ret, params, body } => {
                let block_id = self.new_block(name.clone());
                if name == "main" {
                    self.block_arena[0].succs.push(block_id);
                }

                for param in params {
                    // self.block_arena[block_id].insts.push(Instruction::De);
                }
            }
        }
    }
}
