use crate::types::Token;
use crate::types::Token as Val;
use crate::types::Statement as Stat;
use crate::types::Expression as Expr;
use crate::types::Operator as Op;

use crate::types::ParseErrorKind;
use crate::types::ExprError;
use crate::types::StatError;

use crate::types::SlidingWindow;
use crate::types::VecWindow;

use crate::types::YololNumber;

use crate::environment::Environment as Env;
use crate::environment::ContextMap;

fn evaluate_statement(env: &mut Env, input: Stat)
{
    match input
    {
        Stat::If(cond, body, else_body) => evaluate_if(env, cond, body, else_body),
        Stat::Goto(target) => evaluate_goto(env, target),
        Stat::Assignment(ident, op, expr) => evaluate_assignment(env, ident, op, expr),
        Stat::Expression(expr) => { evaluate_expression(env, expr); },
    }
}

fn evaluate_if(env: &mut Env, cond: Box<Expr>, body: Vec<Stat>, else_body: Option<Vec<Stat>>)
{
    let cond_result = evaluate_expression(env, cond);

    if let Token::YololNum(num) = cond_result
    {
        if num == YololNumber::from(0) { return }
    }

    for statement in body
    {
        evaluate_statement(env, statement);
    }

    if let Some(else_body_vec) = else_body
    {
        for statement in else_body_vec
        {
            evaluate_statement(env, statement);
        }
    }
}

fn evaluate_goto(env: &mut Env, target: Box<Expr>)
{
    println!("Goto does nothing right now. Just FYI");
}

fn evaluate_assignment(env: &mut Env, ident: Token, op: Op, expr: Box<Expr>)
{
    let new_value = match op
    {
        // Op::Assign => evaluate_expression(env, expr),
        // Op::AddAssign => env.context.get_val(&ident.to_string()) + evaluate_expression(env, expr),
        Op::SubAssign => {},
        Op::MulAssign => {},
        Op::DivAssign => {},
        Op::ModAssign => {},
        Op::PowAssign => {},

        _ => panic!("Fix this")
    };

    // env.context.insert(ident.to_string(), new_value);
}

fn evaluate_expression(env: &mut Env, input: Box<Expr>) -> Token
{
    match *input
    {
        Expr::Grouping(expr) => evaluate_expression(env, expr),
        Expr::BinaryOp(op, left, right) => evaluate_binary_op(env, op, left, right),
        Expr::UnaryOp(op, target) => evaluate_unary_op(env, op, target),
        Expr::Value(_) => Token::Caret,
    }
}

fn evaluate_binary_op(env: &mut Env, op: Op, left: Box<Expr>, right: Box<Expr>) -> Token
{

    Token::Caret
}

fn evaluate_unary_op(env: &mut Env, op: Op, target: Box<Expr>) -> Token
{

    Token::Caret
}