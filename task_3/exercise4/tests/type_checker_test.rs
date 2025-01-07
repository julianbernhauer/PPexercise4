

use exercise4::type_checker::{TypeChecker, Type};
use exercise4::expression::Expression;

#[cfg(test)]
mod tests {
    use exercise4::type_checker::TypeError;
    use super::*;

    #[test]
    fn test_true() {
        let checker = TypeChecker;
        let expr = Expression::True;
        assert_eq!(checker.visit(&expr), Ok(Type::Bool));
    }

    #[test]
    fn test_negation_of_number() {
        let checker = TypeChecker;
        let expr = Expression::Neg(Box::new(Expression::Num(5)));
        assert_eq!(checker.visit(&expr), Ok(Type::Num));
    }

    #[test]
    fn test_negation_of_boolean() {
        let checker = TypeChecker;
        let expr = Expression::Neg(Box::new(Expression::True));
        assert_eq!(
            checker.visit(&expr),
            Err(TypeError {
                expected: Type::Num,
                found: Type::Bool
            })
        );
    }

    #[test]
    fn test_addition_of_numbers() {
        let checker = TypeChecker;
        let expr = Expression::Add(
            Box::new(Expression::Num(3)),
            Box::new(Expression::Num(7)),
        );
        assert_eq!(checker.visit(&expr), Ok(Type::Num));
    }

    #[test]
    fn test_addition_of_boolean_and_number() {
        let checker = TypeChecker;
        let expr = Expression::Add(
            Box::new(Expression::True),
            Box::new(Expression::Num(7)),
        );
        assert_eq!(
            checker.visit(&expr),
            Err(TypeError {
                expected: Type::Num,
                found: Type::Bool
            })
        );
    }

    #[test]
    fn test_disjunction_of_booleans() {
        let checker = TypeChecker;
        let expr = Expression::Or(
            Box::new(Expression::True),
            Box::new(Expression::False),
        );
        assert_eq!(checker.visit(&expr), Ok(Type::Bool));
    }

    #[test]
    fn test_disjunction_of_boolean_and_number() {
        let checker = TypeChecker;
        let expr = Expression::Or(
            Box::new(Expression::True),
            Box::new(Expression::Num(5)),
        );
        assert_eq!(
            checker.visit(&expr),
            Err(TypeError {
                expected: Type::Bool,
                found: Type::Num
            })
        );
    }

    #[test]
    fn test_equality_of_same_types() {
        let checker = TypeChecker;
        let expr = Expression::Eq(
            Box::new(Expression::Num(10)),
            Box::new(Expression::Num(20)),
        );
        assert_eq!(checker.visit(&expr), Ok(Type::Bool));
    }

    #[test]
    fn test_equality_of_different_types() {
        let checker = TypeChecker;
        let expr = Expression::Eq(
            Box::new(Expression::Num(10)),
            Box::new(Expression::True),
        );
        assert_eq!(
            checker.visit(&expr),
            Err(TypeError {
                expected: Type::Num,
                found: Type::Bool
            })
        );
    }

    #[test]
    fn test_if_with_valid_condition_and_branches() {
        let checker = TypeChecker;
        let expr = Expression::If(
            Box::new(Expression::True),
            Box::new(Expression::Num(1)),
            Box::new(Expression::Num(2)),
        );
        assert_eq!(checker.visit(&expr), Ok(Type::Num));
    }

    #[test]
    fn test_if_with_invalid_condition() {
        let checker = TypeChecker;
        let expr = Expression::If(
            Box::new(Expression::Num(1)), // Invalid condition
            Box::new(Expression::Num(2)),
            Box::new(Expression::Num(3)),
        );
        assert_eq!(
            checker.visit(&expr),
            Err(TypeError {
                expected: Type::Bool,
                found: Type::Num
            })
        );
    }

    #[test]
    fn test_if_with_mismatched_branch_types() {
        let checker = TypeChecker;
        let expr = Expression::If(
            Box::new(Expression::True),
            Box::new(Expression::Num(1)),
            Box::new(Expression::True), // Different branch type
        );
        assert_eq!(
            checker.visit(&expr),
            Err(TypeError {
                expected: Type::Num,
                found: Type::Bool
            })
        );
    }

    #[test]
    fn test_nested_expressions() {
        let checker = TypeChecker;
        let expr = Expression::Add(
            Box::new(Expression::Neg(Box::new(Expression::Num(10)))),
            Box::new(Expression::Num(20)),
        );
        assert_eq!(checker.visit(&expr), Ok(Type::Num));
    }

    #[test]
    fn test_complex_expression() {
        let checker = TypeChecker;
        let expr = Expression::If(
            Box::new(Expression::Eq(
                Box::new(Expression::Add(
                    Box::new(Expression::Num(3)),
                    Box::new(Expression::Num(4)),
                )),
                Box::new(Expression::Num(7)),
            )),
            Box::new(Expression::True),
            Box::new(Expression::False),
        );
        assert_eq!(checker.visit(&expr), Ok(Type::Bool));
    }

}