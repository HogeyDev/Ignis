use std::collections::{HashMap, HashSet};

use crate::{analyzer::Analyzer, parser::{Declaration, DeclarationId, RootAst}};

pub type BlockId = usize;
pub struct BasicBlock {
    label: String,
    preds: Vec<BlockId>,
    succs: Vec<BlockId>,

    insts: Vec<Instruction>,

    defs: HashMap<String, ValueId>,
    inc_phis: HashMap<String, ValueId>,
}
impl BasicBlock {
    pub fn new(label: String) -> Self {
        Self {
            label,
            preds: Vec::new(),
            succs: Vec::new(),

            insts: Vec::new(),

            defs: HashMap::new(),
            inc_phis: HashMap::new(),
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

pub type ValueId = usize;
pub enum Value {
    Undef,
    PhiNode {
        block_id: BlockId,
        operands: Vec<ValueId>,
    },
}

pub enum Instruction {
    BinExp {
        dest: String,
        op: BinOp,
    },
}

pub struct IrBuilder<'a> {
    analyzer: &'a Analyzer<'a>,
    pub block_arena: Vec<BasicBlock>,

    label_counter: usize,
    sealed_blocks: HashSet<BlockId>,
    value_arena: Vec<Value>,
}

impl<'a> IrBuilder<'a> {
    pub fn new(analyzer: &'a Analyzer) -> Self {
        Self {
            analyzer,
            block_arena: Vec::new(),

            label_counter: 0,
            sealed_blocks: HashSet::new(),
            value_arena: Vec::new(),
        }
    }

    fn numeric_label(&mut self) -> String {
        self.label_counter += 1;
        format!("label_{}", self.label_counter-1)
    }
    fn new_block(&mut self, label: String) -> BlockId {
        self.block_arena.push(BasicBlock::new(label));
        self.block_arena.len() - 1
    }

    pub fn run(&mut self, ast: &RootAst) {
        self.new_block(String::from("entrypoint"));

        for decl_id in ast {
            self.declaration(*decl_id);
        }
    }

    fn write_variable(&mut self, var_name: String, block_id: BlockId, value_id: ValueId) {
        self.block_arena[block_id].defs.insert(var_name, value_id);
    }
    fn read_variable(&mut self, var_name: String, block_id: BlockId) -> ValueId {
        match self.block_arena[block_id].defs.get(&var_name) {
            Some(&value_id) => value_id,
            None => self.read_variable_recursive(var_name, block_id),
        }
    }
    fn read_variable_recursive(&mut self, var_name: String, block_id: BlockId) -> ValueId {
        let value_id = if !self.sealed_blocks.contains(&block_id) {
            let phi_id = self.value_arena.len();
            self.value_arena.push(Value::PhiNode { block_id, operands: Vec::new() });
            self.block_arena[block_id].inc_phis.insert(var_name.clone(), phi_id);
            phi_id
        } else if self.block_arena[block_id].preds.len() == 1 {
            self.read_variable(var_name.clone(), self.block_arena[block_id].preds[0])
        } else {
            let phi_id = self.value_arena.len();
            self.value_arena.push(Value::PhiNode { block_id, operands: Vec::new() });
            self.write_variable(var_name.clone(), block_id, phi_id);
            self.add_phi_operands(var_name.clone(), phi_id)
        };
        self.write_variable(var_name, block_id, value_id);
        value_id
    }
    fn add_phi_operands(&mut self, var_name: String, phi_id: ValueId) -> ValueId {
        // TODO:
        // for pred in phi.block.preds:
        //     phi.appendOperand(readVariable(variable, pred))
        // return tryRemoveTrivialPhi(phi)

        let Value::PhiNode { block_id, .. } = self.value_arena[phi_id] else { unreachable!() };
        let preds = self.block_arena[block_id].preds.clone();

        let new_operands: Vec<_> = preds.iter().map(|pred| self.read_variable(var_name.clone(), *pred)).collect();
        
        let Value::PhiNode { operands, .. } = &mut self.value_arena[phi_id] else { unreachable!() };
        operands.extend(new_operands);

        self.try_remove_trivial_phi(phi_id)
    }
    fn try_remove_trivial_phi(&mut self, phi_id: ValueId) -> ValueId {
        let Value::PhiNode { ref operands, .. } = self.value_arena[phi_id] else { unreachable!() };

        let mut same: Option<ValueId> = None;
        for op in operands {
            if Some(*op) == same || *op == phi_id { continue; }
            if same.is_some() { return phi_id; }

            same = Some(*op);
        }

        let same_id = match same {
            Some(id) => id,
            None => {
                self.value_arena.push(Value::Undef);
                self.value_arena.len()-1
            }
        };
        let mut users = self.get_users_of(phi_id);
        users.retain(|&user_id| user_id != phi_id);

        for &user_id in &users {
            self.replace_operand_in_value(user_id, phi_id, same_id);
        }

        for user_id in users {
            let user = &self.value_arena[user_id];
            if matches!(user, Value::PhiNode { .. }) {
                self.try_remove_trivial_phi(user_id);
            }
        }

        same_id
    }
    fn get_users_of(&mut self, target_id: ValueId) -> Vec<ValueId> {
        let mut users = Vec::new();
        for (id, value) in self.value_arena.iter().enumerate() {
            if let Value::PhiNode { operands, .. } = value {
                if operands.contains(&target_id) {
                    users.push(id);
                }
            }
        }
        users
    }
    fn replace_operand_in_value(&mut self, target_id: ValueId, old_id: ValueId, new_id: ValueId) {
        match &mut self.value_arena[target_id] {
            Value::PhiNode { operands, .. } => {
                for op in operands {
                    if *op == old_id {
                        *op = new_id;
                    }
                }
            }
            _ => {}
        }
    }
    fn seal_block(&mut self, block_id: BlockId) {
        let inc_phis = std::mem::take(&mut self.block_arena[block_id].inc_phis);
        for (var_name, &id) in &inc_phis {
            self.add_phi_operands(var_name.clone(), id);
        }
        self.block_arena[block_id].inc_phis = inc_phis;
        self.sealed_blocks.insert(block_id);
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
            // Declaration::Statement(stmt_id) => eprintln!("{:#?}", self.analyzer.parser.statement_arena[*stmt_id]),
            x => todo!("{x:#?}"),
        }
    }
}
