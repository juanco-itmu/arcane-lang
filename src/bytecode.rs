use crate::value::Value;

#[derive(Debug, Clone)]
pub enum OpCode {
    Constant(usize),    // Push constant from pool
    Pop,                // Pop top of stack

    GetVar(String),     // Load variable onto stack
    SetVar(String),     // Store top of stack in variable

    Add,                // +
    Subtract,           // -
    Multiply,           // *
    Divide,             // /
    Negate,             // unary -

    Equal,              // ==
    NotEqual,           // !=
    Less,               // <
    LessEqual,          // <=
    Greater,            // >
    GreaterEqual,       // >=

    Not,                // !
    And,                // && (short-circuit)
    Or,                 // || (short-circuit)

    Print,              // Print top of stack
    Jump(usize),        // Unconditional jump
    JumpIfFalse(usize), // Jump if top of stack is falsy
    Return,             // End program
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn write(&mut self, op: OpCode) -> usize {
        self.code.push(op);
        self.code.len() - 1
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn patch_jump(&mut self, offset: usize, target: usize) {
        match &mut self.code[offset] {
            OpCode::Jump(ref mut addr) => *addr = target,
            OpCode::JumpIfFalse(ref mut addr) => *addr = target,
            _ => panic!("Tried to patch non-jump instruction"),
        }
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}
