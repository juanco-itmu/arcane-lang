use std::collections::HashMap;

use crate::bytecode::{Chunk, OpCode};
use crate::value::Value;

pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
    globals: HashMap<String, Value>,
}

impl VM {
    pub fn new(chunk: Chunk) -> Self {
        VM {
            chunk,
            ip: 0,
            stack: Vec::new(),
            globals: HashMap::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        loop {
            if self.ip >= self.chunk.code.len() {
                return Ok(());
            }

            let instruction = self.chunk.code[self.ip].clone();
            self.ip += 1;

            match instruction {
                OpCode::Constant(idx) => {
                    let value = self.chunk.constants[idx].clone();
                    self.push(value);
                }
                OpCode::Pop => {
                    self.pop()?;
                }
                OpCode::GetVar(name) => {
                    let value = self
                        .globals
                        .get(&name)
                        .cloned()
                        .ok_or_else(|| format!("Ongedefinieerde veranderlike: '{}'", name))?;
                    self.push(value);
                }
                OpCode::SetVar(name) => {
                    let value = self.peek()?.clone();
                    self.globals.insert(name, value);
                }
                OpCode::Add => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (a, b) {
                        (Value::Number(x), Value::Number(y)) => {
                            self.push(Value::Number(x + y));
                        }
                        _ => return Err("Operande moet nommers wees vir '+'.".to_string()),
                    }
                }
                OpCode::Subtract => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (a, b) {
                        (Value::Number(x), Value::Number(y)) => {
                            self.push(Value::Number(x - y));
                        }
                        _ => return Err("Operande moet nommers wees vir '-'.".to_string()),
                    }
                }
                OpCode::Multiply => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (a, b) {
                        (Value::Number(x), Value::Number(y)) => {
                            self.push(Value::Number(x * y));
                        }
                        _ => return Err("Operande moet nommers wees vir '*'.".to_string()),
                    }
                }
                OpCode::Divide => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (a, b) {
                        (Value::Number(x), Value::Number(y)) => {
                            if y == 0.0 {
                                return Err("Deling deur nul.".to_string());
                            }
                            self.push(Value::Number(x / y));
                        }
                        _ => return Err("Operande moet nommers wees vir '/'.".to_string()),
                    }
                }
                OpCode::Negate => {
                    let value = self.pop()?;
                    match value {
                        Value::Number(n) => self.push(Value::Number(-n)),
                        _ => return Err("Operand moet 'n nommer wees vir negasie.".to_string()),
                    }
                }
                OpCode::Equal => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(Value::Boolean(self.values_equal(&a, &b)));
                }
                OpCode::NotEqual => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(Value::Boolean(!self.values_equal(&a, &b)));
                }
                OpCode::Less => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (a, b) {
                        (Value::Number(x), Value::Number(y)) => {
                            self.push(Value::Boolean(x < y));
                        }
                        _ => return Err("Operande moet nommers wees vir '<'.".to_string()),
                    }
                }
                OpCode::LessEqual => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (a, b) {
                        (Value::Number(x), Value::Number(y)) => {
                            self.push(Value::Boolean(x <= y));
                        }
                        _ => return Err("Operande moet nommers wees vir '<='.".to_string()),
                    }
                }
                OpCode::Greater => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (a, b) {
                        (Value::Number(x), Value::Number(y)) => {
                            self.push(Value::Boolean(x > y));
                        }
                        _ => return Err("Operande moet nommers wees vir '>'.".to_string()),
                    }
                }
                OpCode::GreaterEqual => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (a, b) {
                        (Value::Number(x), Value::Number(y)) => {
                            self.push(Value::Boolean(x >= y));
                        }
                        _ => return Err("Operande moet nommers wees vir '>='.".to_string()),
                    }
                }
                OpCode::Not => {
                    let value = self.pop()?;
                    self.push(Value::Boolean(!value.is_truthy()));
                }
                OpCode::And | OpCode::Or => {
                    // These are handled by jump instructions in the compiler
                    // but keeping them here for completeness
                }
                OpCode::Print => {
                    let value = self.pop()?;
                    println!("{}", value);
                }
                OpCode::Jump(target) => {
                    self.ip = target;
                }
                OpCode::JumpIfFalse(target) => {
                    let condition = self.peek()?;
                    if !condition.is_truthy() {
                        self.ip = target;
                    }
                }
                OpCode::Return => {
                    return Ok(());
                }
            }
        }
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Result<Value, String> {
        self.stack.pop().ok_or_else(|| "Stapel onderloop.".to_string())
    }

    fn peek(&self) -> Result<&Value, String> {
        self.stack.last().ok_or_else(|| "Stapel is leeg.".to_string())
    }

    fn values_equal(&self, a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => x == y,
            (Value::Boolean(x), Value::Boolean(y)) => x == y,
            (Value::Nil, Value::Nil) => true,
            _ => false,
        }
    }
}
