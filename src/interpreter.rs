use crate::types::Value;
use crate::types::Statement as Stat;
use crate::types::Expression as Expr;

use crate::types::Line;

use crate::types::Operator as Op;

use crate::types::EvaluationError;
use crate::types::EvaluationErrorKind;

use crate::types::LiteralValue;

use crate::environment::Environment as Env;
use crate::environment::ContextMap;

pub fn evaluate_line(env: &mut Env, input: &Line) -> Result<(), EvaluationError>
{
    for statement in &input.0
    {
        evaluate_statement(env, statement.clone())?;
    }

    Ok(())
}

pub fn evaluate_statement(env: &mut Env, input: Stat) -> Result<(), EvaluationError>
{
    match input
    {
        Stat::Comment(_) => {},
        Stat::If(cond, body, else_body) => evaluate_if(env, cond, body, else_body)?,
        Stat::Goto(target) => evaluate_goto(env, target)?,
        Stat::Assignment(ident, op, expr) => evaluate_assignment(env, ident, op, expr)?,
        Stat::Expression(expr) => { evaluate_expression(env, expr)?; },
    };

    Ok(())
}

fn evaluate_if(env: &mut Env, cond: Box<Expr>, body: Vec<Stat>, else_body: Option<Vec<Stat>>) -> Result<(), EvaluationError>
{
    let cond_result = evaluate_expression(env, cond)?;

    if cond_result == LiteralValue::get_false()
    {
        if let Some(else_body_vec) = else_body
        {
            for statement in else_body_vec
            {
                evaluate_statement(env, statement)?;
            }
        }
    }
    else
    {
        for statement in body
        {
            evaluate_statement(env, statement)?;
        }
    }

    Ok(())
}

fn evaluate_goto(env: &mut Env, target: Box<Expr>) -> Result<(), EvaluationError>
{
    let value = evaluate_expression(env, target)?;

    match value
    {
        LiteralValue::NumberVal(num) => {
            let num = num.floor();
            let num = num.clamp(1, 20);
            env.set_next_line(num);
        },
        LiteralValue::StringVal(_) => {
            return Err(EvaluationError {
                kind: EvaluationErrorKind::Misc,
                error_text: String::from("Attempted to goto with a string value!")
            });
        }
    }

    Ok(())
}

fn evaluate_assignment(env: &mut Env, ident: Value, op: Op, expr: Box<Expr>) -> Result<(), EvaluationError>
{
    let ident_string = match ident
    {
        Value::LocalVar(string) |
        Value::DataField(string) => string,

        _ => return Err(EvaluationError {
            kind: EvaluationErrorKind::OperatorError,
            error_text: String::from("Attempting to use assignment with a non-identifier on the left side!")
        })
    };

    let current_value = env.get_val(&ident_string);

    let new_value = if let Op::Assign = op
    {
        evaluate_expression(env, expr)?
    }
    else
    {
        let val = match op
        {
            Op::AddAssign => current_value + evaluate_expression(env, expr)?,
            Op::SubAssign => current_value - evaluate_expression(env, expr)?,
            Op::MulAssign => current_value * evaluate_expression(env, expr)?,
            Op::DivAssign => current_value / evaluate_expression(env, expr)?,
            Op::ModAssign => current_value % evaluate_expression(env, expr)?,

            _ => {
                return Err(EvaluationError {
                    kind: EvaluationErrorKind::OperatorError,
                    error_text: String::from("Attempting to evaluate an assignment without an assignment op!")
                })
            }
        };

        val?
    };

    env.set_val(ident_string, new_value);
    Ok(())
}

fn evaluate_expression(env: &mut Env, input: Box<Expr>) -> Result<LiteralValue, EvaluationError>
{
    match *input
    {
        Expr::BinaryOp(op, left, right) => evaluate_binary_op(env, op, left, right),
        Expr::UnaryOp(op, target) => evaluate_unary_op(env, op, target),
        Expr::Value(value) => evaluate_value(env, value),
    }
}

fn evaluate_binary_op(env: &mut Env, op: Op, left: Box<Expr>, right: Box<Expr>) -> Result<LiteralValue, EvaluationError>
{
    let left_value = evaluate_expression(env, left)?;
    let right_value = evaluate_expression(env, right)?;

    if let Op::Lesser | Op::Greater | Op::LesserEq | Op::GreaterEq |
                                Op::Equal | Op::NotEqual | Op::And | Op::Or = op
    {
        let bool_result = match op
        {
            Op::Lesser => left_value < right_value,
            Op::Greater => left_value > right_value,
            Op::LesserEq => left_value <= right_value,
            Op::GreaterEq => left_value >= right_value,
            Op::Equal => left_value == right_value,
            Op::NotEqual => left_value != right_value,
            Op::And => (left_value != LiteralValue::get_false()) && (right_value != LiteralValue::get_false()),
            Op::Or => (left_value != LiteralValue::get_false()) || (right_value != LiteralValue::get_false()),

            _ => return Err(EvaluationError {
                kind: EvaluationErrorKind::NonExhaustivePattern,
                error_text: String::from("Didn't find behaviour to match in boolean section of binary ops")
            })
        };

        Ok(LiteralValue::from(bool_result))
    }
    else
    {
        let result = match op
        {
            Op::Add => left_value + right_value,
            Op::Sub => left_value - right_value,
            Op::Mul => left_value * right_value,
            Op::Div => left_value / right_value,
            Op::Mod => left_value % right_value,
            Op::Pow => left_value.pow(right_value),

            _ => return Err(EvaluationError {
                kind: EvaluationErrorKind::NonExhaustivePattern,
                error_text: String::from("Didn't find behaviour to match in numerical section of binary ops")
            })
        };

        Ok(result?)
    }
}

fn evaluate_unary_op(env: &mut Env, op: Op, target: Box<Expr>) -> Result<LiteralValue, EvaluationError>
{
    if let Op::PreInc | Op::PostInc | Op::PreDec | Op::PostDec = op
    {
        let ident = match *target
        {
            Expr::Value(Value::LocalVar(ident)) |
            Expr::Value(Value::DataField(ident)) => ident,
            _ => {
                return Err(EvaluationError {
                    kind: EvaluationErrorKind::OperatorError,
                    error_text: String::from("Tried to use pre/post - inc/dec on a non-identifier!")
                });
            }
        };

        match op
        {
            Op::PreInc => {
                let new_value = (env.get_val(&ident) + LiteralValue::from(1))?;
                env.set_val(ident, new_value.clone());
                Ok(new_value)
            },
            Op::PostInc => {
                let original_value = env.get_val(&ident);
                let new_value = original_value.clone() + LiteralValue::from(1);
                env.set_val(ident, new_value?);
                Ok(original_value)
            },
            Op::PreDec => {
                let new_value = (env.get_val(&ident) - LiteralValue::from(1))?;
                env.set_val(ident, new_value.clone());
                Ok(new_value)
            },
            Op::PostDec => {
                let original_value = env.get_val(&ident);
                let new_value = original_value.clone() - LiteralValue::from(1);
                env.set_val(ident, new_value?);
                Ok(original_value)
            },

            _ => Err(EvaluationError {
                kind: EvaluationErrorKind::NonExhaustivePattern,
                error_text: String::from("Didn't find behaviour to match in pre/post-inc/dec section of unary ops")
            })
        }
    }
    else
    {
        let value = evaluate_expression(env, target)?;

        if let value @ LiteralValue::NumberVal(_) = value
        {
            let result = match op
            {
                Op::Negate => -value,
                Op::Fact => value.factorial(),
                
                Op::Abs => value.abs(),
                Op::Sqrt => value.sqrt(),

                Op::Sin => value.sin(),
                Op::Cos => value.cos(),
                Op::Tan => value.tan(),

                Op::Arcsin => value.arcsin(),
                Op::Arccos => value.arccos(),
                Op::Arctan => value.arctan(),

                Op::Not => !value,

                _ => return Err(EvaluationError {
                    kind: EvaluationErrorKind::NonExhaustivePattern,
                    error_text: String::from("Didn't find behaviour to match in numerical section of unary ops")
                })
            };

            Ok(result?)
        }
        else
        {
            match op
            {
                Op::Not => Ok((!value)?),

                _ => Err(EvaluationError {
                    kind: EvaluationErrorKind::NonExhaustivePattern,
                    error_text: String::from("Didn't find behaviour to match in final section of binary ops")
                })
            }
        }
    }
}

fn evaluate_value(env: &mut Env, input: Value) -> Result<LiteralValue, EvaluationError>
{
    let output = match input
    {
        Value::Group(expr) => evaluate_expression(env, expr)?,
        Value::LocalVar(ident) => env.get_val(&ident),
        Value::DataField(ident) => env.get_val(&ident),
        Value::NumberVal(number) => LiteralValue::NumberVal(number),
        Value::StringVal(string) => LiteralValue::StringVal(string),
    };

    Ok(output)
}