
use crate::expression::Expression;

// Define a struct for TypeChecker
pub struct TypeChecker;

// Define types
#[derive(Debug, PartialEq)]
pub enum Type {
    Num,
    Bool,
}

// Define error
#[derive(Debug, PartialEq)]
pub struct TypeError {
    expected: Type,
    found: Type,
}

impl TypeChecker {
    pub fn visit(&self, expr: &Expression) -> Result<Type, TypeError> {
        // Placeholder for visit method logic
        unimplemented!()
    }
}