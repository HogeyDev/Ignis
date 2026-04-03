class Instr:
    def __init__(self, template, *args):
        self.template = template
        self.args = args
        
    def __repr__(self):
        return self.template.format(*self.args)


class Node: pass

class Num(Node):
    def __init__(self, val): self.val = val

class Var(Node):
    def __init__(self, name): self.name = name

class BinOp(Node):
    def __init__(self, left, op, right):
        self.left = left
        self.op = op
        self.right = right

class Assign(Node):
    def __init__(self, name, expr):
        self.name = name
        self.expr = expr

class While(Node):
    def __init__(self, cond, body):
        self.cond = cond
        self.body = body


class Block:
    def __init__(self, name):
        self.name = name
        self.preds = []
        self.defs = {}
        self.sealed = False
        self.incomplete_phis = {}
        self.instructions = []

    def add_pred(self, block):
        self.preds.append(block)

    def emit(self, instr):
        self.instructions.append(instr)

    def __repr__(self): return self.name

class Phi:
    def __init__(self, block):
        self.block = block
        self.operands = set()
        
    def __repr__(self):
        if not self.operands: return "phi(incomplete)"
        ops = sorted([str(o) for o in self.operands])
        return f"phi({', '.join(ops)})"


def write_variable(var_name, block, value):
    block.defs[var_name] = value

def read_variable(var_name, block):
    if var_name in block.defs: return block.defs[var_name]
    return read_variable_recursive(var_name, block)

def read_variable_recursive(var_name, block):
    val = None
    if not block.sealed:
        val = Phi(block)
        block.incomplete_phis[var_name] = val
    elif len(block.preds) == 1:
        val = read_variable(var_name, block.preds[0])
    else:
        val = Phi(block)
        write_variable(var_name, block, val)
        val = add_phi_operands(var_name, val)

    write_variable(var_name, block, val)
    return val

def add_phi_operands(var_name, phi):
    for pred in phi.block.preds:
        phi.operands.add(read_variable(var_name, pred))
    return try_remove_trivial_phi(phi)

def try_remove_trivial_phi(phi):
    unique = {op for op in phi.operands if op is not phi}
    if len(unique) == 1:
        same = unique.pop()
        for k, v in list(phi.block.defs.items()):
            if v is phi: phi.block.defs[k] = same
        return same
    return phi

def seal_block(block):
    for var_name, phi in list(block.incomplete_phis.items()):
        patched_val = add_phi_operands(var_name, phi)
        if patched_val is not phi:
            block.defs[var_name] = patched_val
    block.sealed = True


class SSACompiler:
    def __init__(self):
        self.blocks = []
        self.current_block = None
        self.var_counter = 0

    def new_block(self, name):
        b = Block(name)
        self.blocks.append(b)
        return b

    def new_tmp(self):
        self.var_counter += 1
        return f"t_{self.var_counter}"

    def compile(self, ast_nodes):
        self.current_block = self.new_block("Entry")
        seal_block(self.current_block)

        for node in ast_nodes:
            self.visit(node)
            
        return self.blocks

    def visit(self, node):
        if isinstance(node, Num):
            return str(node.val)
            
        elif isinstance(node, Var):
            return read_variable(node.name, self.current_block)
            
        elif isinstance(node, BinOp):
            left_ssa = self.visit(node.left)
            right_ssa = self.visit(node.right)
            tmp = self.new_tmp()
            self.current_block.emit(Instr("{} = {} {} {}", tmp, left_ssa, node.op, right_ssa))
            return tmp
            
        elif isinstance(node, Assign):
            rhs_ssa = self.visit(node.expr)
            write_variable(node.name, self.current_block, rhs_ssa)
            self.current_block.emit(Instr("// {} is now {}", node.name, rhs_ssa))
            
        elif isinstance(node, While):
            header_block = self.new_block("LoopHeader")
            body_block = self.new_block("LoopBody")
            exit_block = self.new_block("LoopExit")

            header_block.add_pred(self.current_block)
            self.current_block.emit(Instr("jmp {}", header_block.name))
            
            self.current_block = header_block
            cond_ssa = self.visit(node.cond)
            self.current_block.emit(Instr("br {}, {}, {}", cond_ssa, body_block.name, exit_block.name))

            body_block.add_pred(header_block)
            seal_block(body_block)
            self.current_block = body_block
            
            for stmt in node.body:
                self.visit(stmt)
                
            header_block.add_pred(self.current_block)
            self.current_block.emit(Instr("jmp {}", header_block.name))

            seal_block(header_block)

            exit_block.add_pred(header_block)
            seal_block(exit_block)
            self.current_block = exit_block


if __name__ == "__main__":
    # x = 0;
    # while (x < 10) {
    #     x = x + 1;
    # }
    ast = [
        Assign("x", Num(0)),
        While(
            cond=BinOp(Var("x"), "<", Num(10)),
            body=[
                Assign("x", BinOp(Var("x"), "+", Num(1)))
            ]
        )
    ]

    compiler = SSACompiler()
    generated_blocks = compiler.compile(ast)

    print("=== FINAL GENERATED SSA ===")
    for b in generated_blocks:
        print(f"--- Block: {b.name} ---")
        
        for var, val in b.defs.items():
            if isinstance(val, Phi):
                print(f"  {var}_phi = {val}")
                
        for instr in b.instructions:
            print(f"  {instr}")
        print()
