use crate::value::{UpvalueDescriptor, Value};

#[derive(Debug, Clone)]
pub enum OpCode {
    Constant(usize),    // Push constant from pool
    Pop,                // Pop top of stack

    // Global variables
    GetGlobal(String),  // Load global variable onto stack
    SetGlobal(String),  // Store top of stack in global variable
    DefineGlobal(String), // Define a new global variable

    // Local variables (stack-based)
    GetLocal(usize),    // Load local at stack offset onto stack
    SetLocal(usize),    // Store top of stack at stack offset

    // Upvalues (closure captures)
    GetUpvalue(usize),  // Load upvalue onto stack
    SetUpvalue(usize),  // Store top of stack in upvalue
    Closure(usize, Vec<UpvalueDescriptor>), // Create closure from function constant
    CloseUpvalue,       // Close the topmost local variable

    // Legacy aliases (will be removed)
    GetVar(String),     // Load variable onto stack (deprecated)
    SetVar(String),     // Store top of stack in variable (deprecated)

    // Arithmetic
    Add,                // + (also string concatenation)
    Subtract,           // -
    Multiply,           // *
    Divide,             // /
    Modulo,             // %
    Negate,             // unary -

    // Comparison
    Equal,              // ==
    NotEqual,           // !=
    Less,               // <
    LessEqual,          // <=
    Greater,            // >
    GreaterEqual,       // >=

    // Logical
    Not,                // !
    And,                // && (short-circuit)
    Or,                 // || (short-circuit)

    // Control flow
    Print,              // Print top of stack
    Jump(usize),        // Unconditional jump
    JumpIfFalse(usize), // Jump if top of stack is falsy

    // Functions
    Call(usize),        // Call function with N arguments
    TailCall(usize),    // Tail call - reuse current frame
    Return,             // Return from function (with value on stack)

    // Lists
    MakeList(usize),    // Create list from N values on stack
    GetIndex,           // Get element at index: stack[list, index] -> value

    // Pattern matching
    CheckConstructor(String, usize),  // Check if TOS is constructor with name and arity
    GetField(usize),                   // Get field at index from ADT at TOS (doesn't pop)
    GetFieldPop(usize),                // Get field at index from ADT and pop the ADT
    Dup,                               // Duplicate top of stack

    // Modules
    LoadModule(String, String),        // Load module: (path, alias) -> pushes Module value
    GetMember(String),                 // Get member from module on stack
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
