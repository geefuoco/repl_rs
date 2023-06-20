use crate::ast::AsAny;
use crate::ast::BlockStatement;
use crate::ast::BooleanLiteral;
use crate::ast::Expression;
use crate::ast::ExpressionStatement;
use crate::ast::IfExpression;
use crate::ast::InfixExpression;
use crate::ast::IntegerLiteral;
use crate::ast::LetStatement;
use crate::ast::Node;
use crate::ast::PrefixExpression;
use crate::ast::Program;
use crate::ast::ReturnStatement;
use crate::ast::Statement;
use crate::object::Boolean;
use crate::object::ErrorObject;
use crate::object::Integer;
use crate::object::Null;
use crate::object::Object;
use crate::object::ObjectTypes;
use crate::object::Return;

const TRUE: Boolean = Boolean { value: true };
const FALSE: Boolean = Boolean { value: false };
const NULL: Null = Null {};

fn bool_helper(b: bool) -> Boolean {
    if b {
        return TRUE;
    }
    FALSE
}

fn eval_helper<T: 'static>(node: &(impl Node + AsAny)) -> bool {
    node.as_any().downcast_ref::<T>().is_some()
}

fn eval_helper_statement<T: 'static>(node: &Box<dyn Statement>) -> bool {
    node.as_any().downcast_ref::<T>().is_some()
}

fn eval_helper_expression<T: 'static>(node: &Box<dyn Expression>) -> bool {
    node.as_any().downcast_ref::<T>().is_some()
}

fn safely_downcast<T: 'static>(node: &(impl Node + AsAny)) -> &T {
    node.as_any()
        .downcast_ref::<T>()
        .expect("Could not safely downcast")
}

fn safely_downcast_statement<T: 'static>(node: &Box<dyn Statement>) -> &T {
    node.as_any()
        .downcast_ref::<T>()
        .expect("Could not safely downcast")
}

fn safely_downcast_expression<T: 'static>(node: &Box<dyn Expression>) -> &T {
    node.as_any()
        .downcast_ref::<T>()
        .expect("Could not safely downcast")
}

fn eval_block_statement(block: &BlockStatement) -> Option<Box<dyn Object>> {
    let mut final_result: Option<Box<dyn Object>> = None;
    for stmt in block.statements() {
        let result = eval_statement(stmt)?;

        if result.obj_type() == ObjectTypes::ERROR {
            return Some(result);
        }

        match result.as_any().downcast_ref::<Return>() {
            Some(_) => {
                return Some(result);
            }
            None => final_result = Some(result),
        }
    }
    final_result
}

fn eval_statement(statement: &Box<dyn Statement>) -> Option<Box<dyn Object>> {
    let mut result: Option<Box<dyn Object>> = None;
    if eval_helper_statement::<LetStatement>(statement) {
        todo!()
    } else if eval_helper_statement::<ReturnStatement>(statement) {
        let value = safely_downcast_statement::<ReturnStatement>(statement);
        let return_value = eval_expression(value.return_value());
        if return_value.is_err() {
            return Some(return_value);
        }
        return Some(Box::new(Return::new(return_value)));
    } else if eval_helper_statement::<ExpressionStatement>(statement) {
        let value = safely_downcast_statement::<ExpressionStatement>(statement);
        result = Some(eval_expression(value.expression()));
    } else if eval_helper_statement::<BlockStatement>(statement) {
        let value = safely_downcast_statement::<BlockStatement>(statement);
        let block_statement = eval_block_statement(value)?;
        result = Some(block_statement);
    } else {
        unreachable!()
    }
    result
}

fn eval_statements(statements: &Vec<Box<dyn Statement>>) -> Box<dyn Object> {
    let mut result: Option<Box<dyn Object>> = None;
    let mut is_err = false;
    for stmt in statements {
        if eval_helper_statement::<LetStatement>(stmt) {
            todo!()
        } else if eval_helper_statement::<ExpressionStatement>(stmt) {
            let value = safely_downcast_statement::<ExpressionStatement>(stmt);
            result = Some(eval_expression(value.expression()));
        } else if eval_helper_statement::<BlockStatement>(stmt) {
            let value = safely_downcast_statement::<BlockStatement>(stmt);
            result = eval_block_statement(value);
        } else if eval_helper_statement::<ReturnStatement>(stmt) {
            let value = safely_downcast_statement::<ReturnStatement>(stmt);
            let return_value = eval_expression(value.return_value());
            return return_value;
        } else {
            unreachable!()
        }
        if let Some(ref result) = result {
            is_err = result.is_err();
        }
        if is_err {
            return result.expect("Error should be returned here");
        }
    }
    result.expect("Could not evaluate any statements")
}

fn eval_expression(node: &Box<dyn Expression>) -> Box<dyn Object> {
    if eval_helper_expression::<IntegerLiteral>(node) {
        let value = safely_downcast_expression::<IntegerLiteral>(node);
        return Box::new(Integer::new(*value.value()));
    } else if eval_helper_expression::<BooleanLiteral>(node) {
        let value = safely_downcast_expression::<BooleanLiteral>(node);
        if *value.value() {
            return Box::new(TRUE);
        } else {
            return Box::new(FALSE);
        }
    } else if eval_helper_expression::<PrefixExpression>(node) {
        let value = safely_downcast_expression::<PrefixExpression>(node);
        let right = eval_expression(value.expression_right());
        if right.is_err() {
            return right;
        }
        return eval_prefix_expression(value.operator(), &right);
    } else if eval_helper_expression::<InfixExpression>(node) {
        let value = safely_downcast_expression::<InfixExpression>(node);
        let left = eval_expression(value.expression_left());
        if left.is_err() {
            return left;
        }
        let right = eval_expression(value.expression_right());
        if right.is_err() {
            return right;
        }
        return eval_infix_expression(value.operator(), &left, &right);
    } else if eval_helper_expression::<IfExpression>(node) {
        let value = safely_downcast_expression::<IfExpression>(node);
        return eval_if_expression(value).expect("could not evaluate if expression");
    } else {
        return Box::new(NULL);
    }
}

fn eval_bang_operator_expression(exp: &Box<dyn Object>) -> Box<dyn Object> {
    match exp.obj_type() {
        ObjectTypes::BOOLEAN => {
            if exp.inspect() == "true" {
                Box::new(FALSE)
            } else {
                Box::new(TRUE)
            }
        }
        ObjectTypes::NULL => Box::new(TRUE),
        _ => Box::new(FALSE),
    }
}

fn eval_minus_operator_expression(exp: &Box<dyn Object>) -> Box<dyn Object> {
    match exp.obj_type() {
        ObjectTypes::INTEGER => {
            let v: isize = exp.inspect().parse().expect("Value was not an isize");
            Box::new(Integer::new(-v))
        }
        _ => Box::new(ErrorObject::new(format!(
            "unknown operator: -{}",
            exp.obj_type()
        ))),
    }
}

fn eval_prefix_expression(operator: &str, right: &Box<dyn Object>) -> Box<dyn Object> {
    match operator {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_operator_expression(right),
        _ => Box::new(ErrorObject::new(format!(
            "unknown operator: {}{}",
            operator,
            right.obj_type()
        ))),
    }
}

fn eval_infix_expression(
    operator: &str,
    left: &Box<dyn Object>,
    right: &Box<dyn Object>,
) -> Box<dyn Object> {
    if left.obj_type() == ObjectTypes::INTEGER && right.obj_type() == ObjectTypes::INTEGER {
        let left = left.as_integer().expect("Could not cast to Integer");
        let right = right.as_integer().expect("Could not cast to Integer");
        return eval_integer_infix_expression(operator, left, right);
    } else if operator == "==" {
        let left = left.as_boolean().expect("Could not cast to Boolean");
        let right = right.as_boolean().expect("Could not cast to Boolean");
        Box::new(bool_helper(left == right))
    } else if operator == "!=" {
        let left = left.as_boolean().expect("Could not cast to Boolean");
        let right = right.as_boolean().expect("Could not cast to Boolean");
        Box::new(bool_helper(left != right))
    } else if left.obj_type() != right.obj_type() {
        Box::new(ErrorObject::new(format!(
            "type mismatch: {} {} {}",
            left.obj_type(),
            operator,
            right.obj_type()
        )))
    } else {
        Box::new(ErrorObject::new(format!(
            "unknown operator: {} {} {}",
            left.obj_type(),
            operator,
            right.obj_type()
        )))
    }
}

fn eval_integer_infix_expression(operator: &str, left: Integer, right: Integer) -> Box<dyn Object> {
    match operator {
        "+" => Box::new(Integer::new(left.value() + right.value())),
        "-" => Box::new(Integer::new(left.value() - right.value())),
        "*" => Box::new(Integer::new(left.value() * right.value())),
        "/" => Box::new(Integer::new(left.value() / right.value())),
        "<" => Box::new(Boolean::new(left.value() < right.value())),
        ">" => Box::new(Boolean::new(left.value() > right.value())),
        "==" => Box::new(Boolean::new(left.value() == right.value())),
        "!=" => Box::new(Boolean::new(left.value() != right.value())),
        _ => Box::new(ErrorObject::new(format!(
            "unknown operator: {} {} {}",
            left.obj_type(),
            operator,
            right.obj_type()
        ))),
    }
}

fn eval_if_expression(exp: &IfExpression) -> Option<Box<dyn Object>> {
    let condition = eval_expression(exp.condition());
    if condition.is_err() {
        return Some(condition);
    }
    if is_truthy(&condition) {
        return eval_block_statement(exp.consequence());
    } else if exp.alternative().is_some() {
        return eval_block_statement(exp.alternative().as_ref().unwrap());
    } else {
        return Some(Box::new(NULL));
    }
}

fn is_truthy(obj: &Box<dyn Object>) -> bool {
    match obj.obj_type() {
        ObjectTypes::INTEGER => true,
        ObjectTypes::NULL => false,
        ObjectTypes::BOOLEAN => obj
            .inspect()
            .parse::<bool>()
            .expect("object was not a Boolean"),
        ObjectTypes::RETURN => false,
        _ => false,
    }
}

pub fn eval_program(node: &(impl Node + AsAny)) -> Box<dyn Object> {
    if eval_helper::<Program>(node) {
        let value = safely_downcast::<Program>(node);
        return eval_statements(&value.statements);
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        lexer::Lexer,
        object::{ErrorObject, Integer},
        parser::Parser,
    };

    fn test_eval(input: &str) -> Box<dyn Object> {
        let l = Lexer::new(input.into());
        let mut p = Parser::new(l);
        let program = p.parse_program().expect("Program did not parse properly");
        eval_program(&program)
    }

    fn test_int(obj: &Box<dyn Object>, exp: &isize) {
        let obj = obj
            .as_any()
            .downcast_ref::<Integer>()
            .expect("Object was not an Integer");
        assert_eq!(exp, obj.value());
    }

    fn test_bool(obj: &Box<dyn Object>, exp: &bool) {
        let obj = obj
            .as_any()
            .downcast_ref::<Boolean>()
            .expect("Object was not an Boolean");
        assert_eq!(exp, obj.value());
    }

    fn test_null(obj: &Box<dyn Object>) {
        assert!(obj.obj_type() == ObjectTypes::NULL);
    }

    #[test]
    fn eval_works() {
        let inputs = [
            ("5", 5),
            ("10", 10),
            ("-5", -5),
            ("-10", -10),
            ("5 + 5 + 5", 15),
            ("2 * 2 * 2 * 2 * 2", 32),
            ("-50 + 100 + -50", 0),
            ("5 * 2 + 10", 20),
            ("5 + 5 + 5 + 5 - 10", 10),
            ("2 * 2 * 2 * 2 * 2", 32),
            ("-50 + 100 + -50", 0),
            ("5 * 2 + 10", 20),
            ("5 + 2 * 10", 25),
            ("20 + 2 * -10", 0),
            ("50 / 2 * 2 + 10", 60),
            ("2 * (5 + 10)", 30),
            ("3 * 3 * 3 + 10", 37),
            ("3 * (3 * 3) + 10", 37),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
        ];

        for (s, i) in inputs {
            let evaluated = test_eval(s);
            test_int(&evaluated, &i)
        }
    }

    #[test]
    fn eval_bool_expression() {
        let inputs = [
            ("true", true),
            ("false", false),
            ("true == true", true),
            ("true == false", false),
            ("true != true", false),
            ("true != false", true),
            ("false == false", true),
            ("false != false", false),
            ("(1 < 2) == true", true),
            ("(1 > 2) == true", false),
            ("(1 < 2) == false", false),
            ("(1 > 2) == false", true),
            ("1 < 2", true),
            ("1 == 1", true),
            ("1 != 1", false),
            ("1 > 2", false),
            ("1 < 1", false),
            ("1 != 2", true),
        ];

        for (s, i) in inputs {
            let evaluated = test_eval(s);
            test_bool(&evaluated, &i);
        }
    }

    #[test]
    fn test_bang_operator() {
        let inputs = [
            ("!true", false),
            ("!false", true),
            ("!5", false),
            ("!!true", true),
            ("!!false", false),
            ("!!5", true),
        ];

        for (s, i) in inputs {
            let evaluated = test_eval(s);
            test_bool(&evaluated, &i);
        }
    }

    #[test]
    fn test_if_else_expressions() {
        let inputs = [
            ("if (true) { 10 }", Some(10)),
            ("if (1) { 10 }", Some(10)),
            ("if (1 < 2) { 10 }", Some(10)),
            ("if (1 > 2) { 10 } else { 20 }", Some(20)),
            ("if (1 < 2) { 10 } else { 20 }", Some(10)),
        ];

        for (s, i) in inputs {
            let evaluated = test_eval(s);
            match evaluated.as_any().downcast_ref::<Integer>() {
                Some(_) => test_int(&evaluated, &i.expect("No integer found")),
                None => assert!(false, "did not evaluate to an integer"),
            }
        }

        let inputs = [("if (false) { 10 }"), ("if (1 > 2) { 10 }")];

        for s in inputs {
            let evaluated = test_eval(s);
            match evaluated.as_any().downcast_ref::<Null>() {
                Some(_) => test_null(&evaluated),
                None => assert!(false, "did not evaluate to NULL"),
            }
        }
    }

    #[test]
    fn test_return_statements() {
        let inputs = [
            ("return 10;", 10),
            ("return 10; 9;", 10),
            ("return 2 * 5; 9;", 10),
            ("9; return 2 * 5; 9;", 10),
            (
                r#"
             if (10 > 1) {
                if (10 > 1) {
                    return 10;
                }   
                return 1;
             }"#,
                10,
            ),
        ];

        for (s, i) in inputs {
            let evaluated = test_eval(s);
            match evaluated.as_any().downcast_ref::<Return>() {
                Some(ret) => test_int(ret.value(), &i),
                None => {}
            }
        }
    }

    #[test]
    fn test_error_handling() {
        let inputs = [
            ("5 + true;", "type mismatch: INTEGER + BOOLEAN"),
            ("5 + true; 5", "type mismatch: INTEGER + BOOLEAN"),
            ("-true;", "unknown operator: -BOOLEAN"),
            ("false + true;", "unknown operator: BOOLEAN + BOOLEAN"),
            (
                "if (10 > 1) {false + true;}",
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
            (
                "if (10 > 1) {if (10 > 1) {false + true;} return 1;}",
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
        ];

        for (s, exp) in inputs {
            let evaluated = test_eval(s);
            let error_obj = evaluated
                .as_any()
                .downcast_ref::<ErrorObject>()
                .expect(format!("Could not cast {:?} to error object", &evaluated).as_str());
            assert_eq!(exp, error_obj.message());
        }
    }
}
