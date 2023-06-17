use crate::ast::AsAny;
use crate::ast::BooleanLiteral;
use crate::ast::Expression;
use crate::ast::ExpressionStatement;
use crate::ast::IntegerLiteral;
use crate::ast::LetStatement;
use crate::ast::Node;
use crate::ast::PrefixExpression;
use crate::ast::Program;
use crate::ast::ReturnStatement;
use crate::ast::Statement;
use crate::object::Boolean;
use crate::object::Integer;
use crate::object::Null;
use crate::object::Object;
use crate::object::ObjectTypes;

const TRUE: Boolean = Boolean { value: true };
const FALSE: Boolean = Boolean { value: false };
const NULL: Null = Null{};

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

fn eval_statements(statements: &Vec<Box<dyn Statement>>) -> Box<dyn Object> {
    let mut result: Option<Box<dyn Object>> = None;
    for stmt in statements {
        if eval_helper_statement::<LetStatement>(stmt) {
            let value = safely_downcast_statement::<LetStatement>(stmt);
            result = Some(eval(value));
        } else if eval_helper_statement::<ReturnStatement>(stmt) {
            let value = safely_downcast_statement::<ReturnStatement>(stmt);
            result = Some(eval(value));
        } else if eval_helper_statement::<ExpressionStatement>(stmt) {
            let value = safely_downcast_statement::<ExpressionStatement>(stmt);
            result = Some(eval(value));
        } else {
            unreachable!()
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
    } else if eval_helper_expression::<PrefixExpression>(node){
        let value = safely_downcast_expression::<PrefixExpression>(node);
        let right = eval_expression(value.expression_right());
        return eval_prefix_expression(value.operator(), &right);
    }else {
        return Box::new(NULL);
    }
}

fn eval_bang_operator_expression(exp: &Box<dyn Object>) -> Box<dyn Object> {
    match exp.obj_type() {
        ObjectTypes::BOOLEAN=> {
            if exp.inspect() == "true" {
                Box::new(FALSE)
            } else {
                Box::new(TRUE)
            }
        },
        ObjectTypes::NULL=> Box::new(TRUE),
        _ => Box::new(FALSE)
    }
}

fn eval_minus_operator_expression(exp: &Box<dyn Object>) -> Box<dyn Object> {
    match exp.obj_type() {
        ObjectTypes::INTEGER=> {
            let v: isize = exp.inspect().parse().expect("Value was not an isize"); 
            Box::new(Integer::new(-v))
        },
        _ => Box::new(NULL)
    }
}

fn eval_prefix_expression(operator: &str, right: &Box<dyn Object>) -> Box<dyn Object>{
    match operator {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_operator_expression(right),
        _ => Box::new(NULL)
    }
}

pub fn eval(node: &(impl Node + AsAny)) -> Box<dyn Object> {
    // if eval_helper::<IntegerLiteral>(node) {
    //     let value = safely_downcast::<IntegerLiteral>(node);
    //     return Box::new(Integer::new(*value.value()));
    // } else if eval_helper::<BooleanLiteral>(node) {
    //     let value = safely_downcast::<BooleanLiteral>(node);
    //     return Box::new(Boolean::new(*value.value()));
    // } else if eval_helper::<ExpressionStatement>(node) {
    if eval_helper::<ExpressionStatement>(node) {
        let value = safely_downcast::<ExpressionStatement>(node);
        return eval_expression(value.expression());
    } else if eval_helper::<Program>(node) {
        let value = safely_downcast::<Program>(node);
        return eval_statements(&value.statements);
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{lexer::Lexer, object::Integer, parser::Parser};

    fn test_eval(input: &str) -> Box<dyn Object> {
        let l = Lexer::new(input.into());
        let mut p = Parser::new(l);
        let program = p.parse_program().expect("Program did not parse properly");
        eval(&program)
    }

    fn test_int(obj: Box<dyn Object>, exp: &isize) {
        let obj = obj
            .as_any()
            .downcast_ref::<Integer>()
            .expect("Object was not an Integer");
        assert_eq!(exp, obj.value());
    }

    fn test_bool(obj: Box<dyn Object>, exp: &bool) {
        let obj = obj
            .as_any()
            .downcast_ref::<Boolean>()
            .expect("Object was not an Boolean");
        assert_eq!(exp, obj.value());
    }

    #[test]
    fn eval_works() {
        let inputs = [("5", 5), ("10", 10), ("-5", -5), ("-10", -10)];

        for (s, i) in inputs {
            let evaluated = test_eval(s);
            test_int(evaluated, &i)
        }
    }

    #[test]
    fn eval_bool_exprsesion() {
        let inputs = [("true", true), ("false", false)];

        for (s, i) in inputs {
            let evaluated = test_eval(s);
            test_bool(evaluated, &i);
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
            println!("{:?}", evaluated);
            test_bool(evaluated, &i);
        }
    }
}
