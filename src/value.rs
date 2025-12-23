use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

/// Represents a compiled function
#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub arity: usize,
    pub chunk_index: usize,       // Index into the VM's functions vector
    pub upvalue_count: usize,     // Number of upvalues this function captures
}

/// Describes how to capture an upvalue
#[derive(Debug, Clone)]
pub struct UpvalueDescriptor {
    pub index: usize,     // Local slot or upvalue index
    pub is_local: bool,   // True if capturing from immediate enclosing scope
}

/// Runtime upvalue - can be open (on stack) or closed (heap)
#[derive(Debug, Clone)]
pub struct Upvalue {
    pub location: UpvalueLocation,
}

#[derive(Debug, Clone)]
pub enum UpvalueLocation {
    Open(usize),         // Stack index
    Closed(Value),       // Closed-over value
}

/// A closure wraps a function with its captured upvalues
#[derive(Debug, Clone)]
pub struct Closure {
    pub function: Rc<Function>,
    pub upvalues: Vec<Rc<RefCell<Upvalue>>>,
}

/// Represents a native (built-in) function
#[derive(Clone)]
pub struct NativeFunction {
    pub name: String,
    pub arity: usize,
    pub func: fn(&[Value]) -> Result<Value, String>,
}

impl std::fmt::Debug for NativeFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NativeFunction({})", self.name)
    }
}

/// A constructor for an algebraic data type
#[derive(Debug, Clone)]
pub struct TypeConstructorDef {
    pub type_name: String,          // Name of the type (e.g., "Opsie")
    pub constructor_name: String,   // Name of the constructor (e.g., "Sommige")
    pub arity: usize,               // Number of fields
}

/// An instance of an algebraic data type
#[derive(Debug, Clone)]
pub struct AdtInstance {
    pub type_name: String,          // Name of the type
    pub constructor_name: String,   // Name of the constructor used
    pub fields: Vec<Value>,         // Field values
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    String(Rc<String>),
    Nil,
    List(Rc<Vec<Value>>),
    Function(Rc<Function>),
    Closure(Rc<Closure>),
    NativeFunction(Rc<NativeFunction>),
    // ADT constructor (callable to create instances)
    TypeConstructor(Rc<TypeConstructorDef>),
    // ADT instance (result of calling constructor)
    Adt(Rc<AdtInstance>),
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Boolean(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::List(l) => !l.is_empty(),
            Value::Function(_) => true,
            Value::Closure(_) => true,
            Value::NativeFunction(_) => true,
            Value::TypeConstructor(_) => true,
            Value::Adt(_) => true,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            Value::Boolean(b) => {
                if *b {
                    write!(f, "waar")
                } else {
                    write!(f, "vals")
                }
            }
            Value::String(s) => write!(f, "{}", s),
            Value::Nil => write!(f, "nil"),
            Value::List(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            Value::Function(func) => write!(f, "<funksie {}>", func.name),
            Value::Closure(closure) => write!(f, "<funksie {}>", closure.function.name),
            Value::NativeFunction(nf) => write!(f, "<ingeboude {}>", nf.name),
            Value::TypeConstructor(tc) => write!(f, "<konstruktor {}.{}>", tc.type_name, tc.constructor_name),
            Value::Adt(adt) => {
                write!(f, "{}", adt.constructor_name)?;
                if !adt.fields.is_empty() {
                    write!(f, "(")?;
                    for (i, field) in adt.fields.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", field)?;
                    }
                    write!(f, ")")?;
                }
                Ok(())
            }
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            (Value::List(a), Value::List(b)) => a == b,
            (Value::Function(a), Value::Function(b)) => Rc::ptr_eq(a, b),
            (Value::Closure(a), Value::Closure(b)) => Rc::ptr_eq(a, b),
            (Value::NativeFunction(a), Value::NativeFunction(b)) => Rc::ptr_eq(a, b),
            (Value::TypeConstructor(a), Value::TypeConstructor(b)) => Rc::ptr_eq(a, b),
            (Value::Adt(a), Value::Adt(b)) => {
                a.type_name == b.type_name
                    && a.constructor_name == b.constructor_name
                    && a.fields == b.fields
            }
            _ => false,
        }
    }
}
