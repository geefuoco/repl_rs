
use crate::ast::BlockStatement;
use crate::ast::Expressions;
use crate::ast::Identifier;
use crate::ast::IfExpression;
use crate::ast::Program;
use crate::ast::Statements;
use crate::builtins::BuiltinFunctionNames;
use crate::builtins::BuiltinFunctions;
use crate::object::Boolean;
use crate::object::Environment;
use crate::object::ErrorObject;
use crate::object::Function;
use crate::object::Integer;
use crate::object::Null;
use crate::object::Object;
use crate::object::ObjectTypes;
use crate::object::Objects;
use crate::object::Return;
use crate::object::StringObject;

const TRUE: Boolean = Boolean { value: true };
const FALSE: Boolean = Boolean { value: false };
const NULL: Null = Null {};
const BUILTIN: BuiltinFunctions = BuiltinFunctions::new();

fn bool_helper(b: bool) -> Boolean {
    if b {
        return TRUE;
    }
    FALSE
}

fn eval_identifier(ident: &Identifier, env: &mut Environment) -> Objects {
    match ident.value() {
        "len" => {
            let wrapper = BUILTIN.get(BuiltinFunctionNames::Len);
            return Objects::Builtin(wrapper.clone());
        }
        "drop" => {
            let wrapper = BUILTIN.get(BuiltinFunctionNames::Drop);
            return Objects::Builtin(wrapper.clone());
        }
        _ => {}
    }
    if let Some(obj) = env.get(ident.value().into()) {
        return obj;
    } else {
        return Objects::Error(ErrorObject::new(format!(
            "identifier not found: {}",
            ident.value()
        )));
    }
}

fn eval_block_statement(block: &mut BlockStatement, env: &mut Environment) -> Option<Objects> {
    let mut final_result: Option<Objects> = None;
    for stmt in block.statements_mut().iter_mut() {
        let result = eval_statement(stmt, env)?;

        if result.obj_type() == ObjectTypes::Error {
            return Some(result);
        }

        if let Objects::Return(result) = result {
            return Some(Objects::Return(result));
        } else {
            final_result = Some(result);
        }
    }
    final_result
}

fn eval_statement(statement: &mut Statements, env: &mut Environment) -> Option<Objects> {
    let mut result: Option<Objects> = None;
    match statement {
        Statements::LetStatement(value) => {
            let let_value = eval_expression(&mut value.value_mut(), env);
            if let_value.is_err() {
                return Some(let_value);
            }
            env.set(value.name().value().into(), let_value);
        }
        Statements::ReturnStatement(value) => {
            let return_value = eval_expression(&mut value.return_value_mut(), env);
            if return_value.is_err() {
                return Some(return_value);
            }
            result = Some(Objects::Return(Return::new(return_value)));
        }
        Statements::ExpressionStatement(value) => {
            result = Some(eval_expression(&mut value.expression_mut(), env));
        }
        Statements::BlockStatement(value) => {
            let block_statement = eval_block_statement(value, env)?;
            result = Some(block_statement);
        }
        Statements::Empty => panic!("Reached an empty statement"),
    }
    result
}

fn eval_statements(statements: &mut [Statements], env: &mut Environment) -> Option<Objects> {
    let mut result: Option<Objects> = None;
    let mut is_err = false;
    for stmt in statements.iter_mut() {
        match stmt {
            Statements::LetStatement(value) => {
                let let_value = eval_expression(&mut value.value_mut(), env);
                if let_value.is_err() {
                    println!("Error in let statement");
                    return Some(let_value);
                }
                env.set(value.name().value().into(), let_value);
            }
            Statements::ReturnStatement(value) => {
                let return_value = eval_expression(&mut value.return_value_mut(), env);
                let ret = Objects::Return(Return::new(return_value));
                return Some(ret);
            }
            Statements::ExpressionStatement(value) => {
                result = Some(eval_expression(&mut value.expression_mut(), env));
            }
            Statements::BlockStatement(value) => {
                result = eval_block_statement(value, env);
            }
            Statements::Empty => panic!("Reached an empty statement"),
        }
        if let Some(ref result) = result {
            is_err = result.is_err();
        }
        if is_err {
            return result;
        }
    }
    result
}

fn eval_expression(node: &mut Expressions, env: &mut Environment) -> Objects {
    match node {
        Expressions::Identifier(value) => eval_identifier(value, env),
        Expressions::BooleanLiteral(value) => {
            if *value.value() {
                return Objects::Boolean(TRUE);
            } else {
                return Objects::Boolean(FALSE);
            }
        }
        Expressions::IntegerLiteral(value) => Objects::Integer(Integer::new(value.value())),
        Expressions::IfExpression(value) => {
            eval_if_expression(value, env).expect("could not evaluate if expression")
        }
        Expressions::InfixExpression(value) => {
            let left = eval_expression(&mut value.expression_left_mut(), env);
            if left.is_err() {
                return left;
            }
            let right = eval_expression(&mut value.expression_right_mut(), env);
            if right.is_err() {
                return right;
            }
            return eval_infix_expression(value.operator(), left, right);
        }
        Expressions::PrefixExpression(value) => {
            let right = eval_expression(&mut value.expression_right_mut(), env);
            if right.is_err() {
                return right;
            }
            return eval_prefix_expression(value.operator(), &right);
        }
        Expressions::FunctionLiteral(value) => {
            let params: Vec<Identifier> = value.parameters().to_vec();
            let body = std::mem::replace(value.body_mut(), BlockStatement::empty());
            return Objects::Function(Function::new(params, body, env.clone()));
        }
        Expressions::CallExpression(value) => {
            let mut func = eval_expression(&mut value.function_mut(), env);
            if func.is_err() {
                return func;
            }
            let mut arguments = eval_expressions(&mut value.arguments_mut(), env);
            if arguments.len() == 1 && arguments[0].is_err() {
                return arguments.remove(0);
            }
            return apply_function(&mut func, &mut arguments);
        }
        Expressions::StringLiteral(value) => {
            Objects::String(StringObject::new(value.value().into()))
        }
        _ => Objects::Null(NULL),
    }
}

fn apply_function(func: &mut Objects, arguments: &mut Vec<Objects>) -> Objects {
    match func {
        Objects::Builtin(b) => {
            let func = b.func();
            let evaluated = func(&arguments);
            return evaluated;
        }
        Objects::Function(func) => {
            let extended_env = extend_function_env(&func, arguments);
            if extended_env.is_none() {
                return Objects::Error(ErrorObject::new(
                    "Invalid number of arguments to function".into(),
                ));
            }
            let mut extended_env = extended_env.unwrap();
            let evaluated = eval_block_statement(&mut func.body_mut(), &mut extended_env);
            return evaluated.expect("Expected an Objects value but evaluated to None");
        }
        _ => {
            Objects::Error(ErrorObject::new(
                format!("not a function: {}", func.obj_type()).into(),
            ))
        }
    }
}

fn extend_function_env(func: &Function, args: &mut Vec<Objects>) -> Option<Environment> {
    let mut extended_env = Environment::new_enclosed_environment(func.environment().clone());
    for (_, p) in func.parameters().iter().enumerate() {
        let next = args.remove(0);
        extended_env.set(p.value().into(), next);
    }
    Some(extended_env)
}

fn eval_expressions(expressions: &mut [Expressions], env: &mut Environment) -> Vec<Objects> {
    let mut v = Vec::new();
    for ex in expressions.iter_mut() {
        let evaluated = eval_expression(ex, env);
        if evaluated.is_err() {
            return [evaluated].to_vec();
        }
        v.push(evaluated);
    }
    v
}

fn eval_bang_operator_expression(exp: &Objects) -> Objects {
    match exp.obj_type() {
        ObjectTypes::Boolean => {
            if exp.inspect() == "true" {
                Objects::Boolean(FALSE)
            } else {
                Objects::Boolean(TRUE)
            }
        }
        ObjectTypes::Null => Objects::Boolean(TRUE),
        _ => Objects::Boolean(FALSE),
    }
}

fn eval_minus_operator_expression(exp: &Objects) -> Objects {
    match exp.obj_type() {
        ObjectTypes::Integer => {
            let v: isize = exp.inspect().parse().expect("Value was not an isize");
            Objects::Integer(Integer::new(-v))
        }
        _ => Objects::Error(ErrorObject::new(format!("unknown operator: -{}", exp))),
    }
}

fn eval_prefix_expression(operator: &str, right: &Objects) -> Objects {
    match operator {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_operator_expression(right),
        _ => Objects::Error(ErrorObject::new(format!(
            "unknown operator: {}{}",
            operator, right
        ))),
    }
}

fn eval_infix_expression(operator: &str, left: Objects, right: Objects) -> Objects {
    if left.obj_type() == ObjectTypes::Integer && right.obj_type() == ObjectTypes::Integer {
        let left = left.as_integer().unwrap();
        let right = right.as_integer().unwrap();
        return eval_integer_infix_expression(operator, &left, &right);
    } else if left.obj_type() == ObjectTypes::String && right.obj_type() == ObjectTypes::String {
        let left = left.as_str().expect("Could not cast to String");
        let right = right.as_str().expect("Could not cast to String");
        return eval_string_infix_expression(operator, &left, &right);
    } else if operator == "==" {
        let left = left.as_boolean().expect("Could not cast to Boolean");
        let right = right.as_boolean().expect("Could not cast to Boolean");
        Objects::Boolean(bool_helper(left == right))
    } else if operator == "!=" {
        let left = left.as_boolean().expect("Could not cast to Boolean");
        let right = right.as_boolean().expect("Could not cast to Boolean");
        Objects::Boolean(bool_helper(left != right))
    } else if left.obj_type() != right.obj_type() {
        Objects::Error(ErrorObject::new(format!(
            "type mismatch: {} {} {}",
            left, operator, right
        )))
    } else {
        Objects::Error(ErrorObject::new(format!(
            "unknown operator: {} {} {}",
            left, operator, right
        )))
    }
}

fn eval_string_infix_expression(
    operator: &str,
    left: &StringObject,
    right: &StringObject,
) -> Objects {
    match operator {
        "+" => Objects::String(StringObject::new(format!(
            "{}{}",
            &left.value(),
            &right.value()
        ))),
        _ => Objects::Error(ErrorObject::new(format!(
            "unknown operator: {} {} {}",
            left, operator, right
        ))),
    }
}

fn eval_integer_infix_expression(operator: &str, left: &Integer, right: &Integer) -> Objects {
    match operator {
        "+" => Objects::Integer(Integer::new(left.value() + right.value())),
        "-" => Objects::Integer(Integer::new(left.value() - right.value())),
        "*" => Objects::Integer(Integer::new(left.value() * right.value())),
        "/" => Objects::Integer(Integer::new(left.value() / right.value())),
        "<" => Objects::Boolean(Boolean::new(left.value() < right.value())),
        ">" => Objects::Boolean(Boolean::new(left.value() > right.value())),
        "==" => Objects::Boolean(Boolean::new(left.value() == right.value())),
        "!=" => Objects::Boolean(Boolean::new(left.value() != right.value())),
        _ => Objects::Error(ErrorObject::new(format!(
            "unknown operator: {} {} {}",
            left, operator, right
        ))),
    }
}

fn eval_if_expression(exp: &mut IfExpression, env: &mut Environment) -> Option<Objects> {
    let condition = eval_expression(&mut exp.condition_mut(), env);
    if condition.is_err() {
        return Some(condition);
    }
    if is_truthy(&condition) {
        return eval_block_statement(&mut exp.consequence_mut(), env);
    } else if exp.alternative().is_some() {
        return eval_block_statement(&mut exp.alternative_mut().unwrap(), env);
    } else {
        return Some(Objects::Null(NULL));
    }
}

fn is_truthy(obj: &Objects) -> bool {
    match obj.obj_type() {
        ObjectTypes::Integer => true,
        ObjectTypes::Null => false,
        ObjectTypes::Boolean => obj
            .inspect()
            .parse::<bool>()
            .expect("object was not a Boolean"),
        ObjectTypes::Return => false,
        _ => false,
    }
}

pub fn eval_program(node: &mut Program, env: &mut Environment) -> Option<Objects> {
    eval_statements(&mut node.statements, env)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{lexer::Lexer, parser::Parser};

    enum Types {
        Integer(isize),
        String(String),
    }

    fn test_eval(input: &str) -> Option<Objects> {
        let l = Lexer::new(input.into());
        let mut p = Parser::new(l);
        let mut program = p.parse_program().expect("Program did not parse properly");
        let mut env = Environment::new();
        eval_program(&mut program, &mut env)
    }

    fn test_int(obj: &Objects, exp: &isize) {
        if obj.is_err() {
            println!("{}", obj.clone().as_err().unwrap().message());
            return;
        }
        let obj = obj
            .clone()
            .as_integer()
            .expect(format!("Object was not an Integer. It was a {}", obj).as_str());
        assert_eq!(exp, obj.value());
    }

    fn test_bool(obj: &Objects, exp: &bool) {
        let obj = obj
            .clone()
            .as_boolean()
            .expect(format!("Object was not an Boolean. It was a {}", obj).as_str());
        assert_eq!(exp, obj.value());
    }

    fn test_null(obj: &Objects) {
        assert!(obj.obj_type() == ObjectTypes::Null);
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
            match evaluated {
                Some(v) => test_int(&v, &i),
                None => {
                    println!("No output")
                }
            }
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
            match evaluated {
                Some(v) => test_bool(&v, &i),
                None => {
                    println!("No output")
                }
            }
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
            match evaluated {
                Some(v) => test_bool(&v, &i),
                None => {
                    println!("No output")
                }
            }
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
            match evaluated {
                Some(v) => match v {
                    Objects::Integer(_) => test_int(&v, &i.expect("No integer found")),
                    _ => assert!(false, "did not evaluate to an integer"),
                },
                None => {
                    println!("No output");
                }
            }
        }

        let inputs = [("if (false) { 10 }"), ("if (1 > 2) { 10 }")];

        for s in inputs {
            let evaluated = test_eval(s);
            match evaluated {
                Some(v) => match v {
                    Objects::Null(_) => test_null(&v),
                    _ => assert!(false, "did not evaluate to null"),
                },
                None => {
                    println!("No output");
                }
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
            match evaluated {
                Some(v) => match v {
                    Objects::Return(ret) => test_int(ret.value(), &i),
                    _ => {
                        let exp = v.obj_type();
                        let msg = format!("Expcted Return. Got {}", &exp);
                        eprintln!("{}", msg);
                        assert!(false);
                    }
                },
                None => println!("No output"),
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
            ("foobar", "identifier not found: foobar"),
            (r#""hello" - "world""#, "unknown operator: STRING - STRING"),
        ];

        for (s, exp) in inputs {
            println!("{}", exp);
            let evaluated = test_eval(s);
            match evaluated {
                Some(o) => match o {
                    Objects::Error(e) => assert_eq!(exp, e.message()),
                    _ => assert!(false, "No error message found"),
                },
                None => assert!(false, "Could not evaluate expressions"),
            }
        }
    }

    #[test]
    fn eval_let_statements() {
        let inputs = [
            ("let a = 5; a; a;", 5),
            ("let a = 5; a;", 5),
            ("let a = 5 * 5; a;", 25),
            ("let a = 5; let b = a; b;", 5),
            ("let a = 5; let b = a; let c = a + b + 5; c;", 15),
        ];

        for (s, exp) in inputs {
            let evaluated = test_eval(s);
            if let Some(evaluated) = evaluated {
                test_int(&evaluated, &exp);
            } else {
                assert!(false, "No output");
            }
        }
    }

    #[test]
    fn test_function_object() {
        let input = "fn(x) {x + 2};";
        let evaluated = test_eval(input);
        if let Some(evaluated) = evaluated {
            let fn_object = evaluated
                .clone()
                .as_fn()
                .expect(format!("Expected Function, received: {}", evaluated).as_str());
            assert_eq!(1, fn_object.parameters().len());
            assert_eq!(
                "x",
                fn_object
                    .parameters()
                    .get(0)
                    .expect("Expected a value in parameters")
                    .to_string()
            );
            let expected_body = "(x + 2)";
            assert_eq!(expected_body, fn_object.body().to_string());
        } else {
            assert!(false, "No output")
        }
    }

    #[test]
    fn test_function_application() {
        let inputs = [
            ("let identity = fn(x) {x;}; identity(5);", 5),
            ("let identity = fn(x) { return x; }; identity(5);", 5),
            ("let double = fn(x) { x * 2; }; double(5);", 10),
            ("let add = fn(x, y) { x + y; }; add(5, 5);", 10),
            ("let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));", 20),
            ("fn(x) { x; }(5)", 5),
        ];

        for (input, exp) in inputs {
            let evaluated = test_eval(input);
            if let Some(evaluated) = evaluated {
                match evaluated {
                    Objects::Return(x) => test_int(&x.value(), &exp),
                    Objects::Integer(_) => test_int(&evaluated, &exp),
                    _ => {
                        panic!("Expected integer. Received: {}", evaluated.obj_type())
                    }
                }
            } else {
                assert!(false, "No output");
            }
        }
    }

    #[test]
    fn test_closure() {
        let input = r#"
        let newAdder = fn(x) {
            fn(y) {x + y}
        }
        let addTwo = newAdder(2);
        addTwo(2);
        "#;
        let evaluated = test_eval(input);
        if let Some(evaluated) = evaluated {
            test_int(&evaluated, &4);
        } else {
            assert!(false, "No output");
        }
    }

    #[test]
    fn test_string_object() {
        let input = r#""Hello World""#;
        match test_eval(input) {
            Some(ev) => {
                let ev = ev
                    .clone()
                    .as_str()
                    .expect("Expected was not a StringObject");
                assert_eq!("Hello World", ev.value());
            }
            None => assert!(false, "No output"),
        }
    }

    #[test]
    fn test_string_concatenation() {
        let input = r#""hello" + " " + "world""#;
        match test_eval(input) {
            Some(ev) => {
                let ev = ev.as_str().expect("Object was not a string literal");
                assert_eq!("hello world", ev.value());
            }
            None => assert!(false, "No output"),
        }
    }

    #[test]
    fn test_builtin_functions() {
        let inputs = [
            (r#"len("")"#, Types::Integer(0)),
            (r#"len("hello")"#, Types::Integer(5)),
            (r#"len("    ")"#, Types::Integer(4)),
            (
                r#"len(1)"#,
                Types::String("argument to 'len' not supported, got INTEGER".into()),
            ),
            (
                r#"len("one", "two")"#,
                Types::String("expected 1 argument but received 2".into()),
            ),
        ];

        for (input, exp) in inputs {
            match test_eval(input) {
                Some(ev) => match exp {
                    Types::Integer(x) => {
                        test_int(&ev, &x)
                    }
                    Types::String(x) => {
                        if !ev.is_err() {
                            panic!("Expected error object but received: {:?}", ev);
                        }
                        let err = ev.as_err().unwrap();
                        assert_eq!(x, err.message());
                    }
                },
                None => assert!(false, "No output"),
            }
        }
    }
}
