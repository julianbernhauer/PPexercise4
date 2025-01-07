
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
    pub expected: Type,
    pub found: Type,
}

impl TypeChecker {
    pub fn visit(&self, expr: &Expression) -> Result<Type, TypeError> {
        // Placeholder for visit method logic
        //unimplemented!()

        match expr {
            Expression::True | Expression::False => Ok(Type::Bool),

            Expression::Num(_) => Ok(Type::Num),

            Expression::Neg(e) => {
                // Check the type of the inner expression
                match self.visit(e.as_ref())? {
                    Type::Num => Ok(Type::Num),
                    found => Err(TypeError {
                        expected: Type::Num,
                        found,
                    }),
                }
            }

            Expression::Add(e1, e2) => {
                // Check both operands
                let t1 = self.visit(e1.as_ref())?;
                let t2 = self.visit(e2.as_ref())?;
                if t1 == Type::Num && t2 == Type::Num {
                    Ok(Type::Num)
                } else {
                    Err(TypeError {
                        expected: Type::Num,
                        found: if t1 != Type::Num { t1 } else { t2 },
                    })
                }
            }

            Expression::Or(e1, e2) => {
                // Check both operands
                let t1 = self.visit(e1.as_ref())?;
                let t2 = self.visit(e2.as_ref())?;
                if t1 == Type::Bool && t2 == Type::Bool {
                    Ok(Type::Bool)
                } else {
                    Err(TypeError {
                        expected: Type::Bool,
                        found: if t1 != Type::Bool { t1 } else { t2 },
                    })
                }
            }

            Expression::Eq(e1, e2) => {
                // Check both operands
                let t1 = self.visit(e1.as_ref())?;
                let t2 = self.visit(e2.as_ref())?;
                if t1 == t2 {
                    Ok(Type::Bool)
                } else {
                    Err(TypeError {
                        expected: t1,
                        found: t2,
                    })
                }
            }

            Expression::If(e1, e2, e3) => {
                // Check condition
                let cond_type = self.visit(e1.as_ref())?;
                if cond_type != Type::Bool {
                    return Err(TypeError {
                        expected: Type::Bool,
                        found: cond_type,
                    });
                }

                // Check both branches
                let t2 = self.visit(e2.as_ref())?;
                let t3 = self.visit(e3.as_ref())?;
                if t2 == t3 {
                    Ok(t2)
                } else {
                    Err(TypeError {
                        expected: t2,
                        found: t3,
                    })
                }
            }
        }
    }
}