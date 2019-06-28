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

pub fn parse(input: Vec<Token>) -> Result<Vec<Stat>, StatError>
{
    let mut output_vec: Vec<Stat> = Vec::new();
    let mut window = VecWindow::new(&input, 0);

    while window.remaining_length() > 0
    {
        let value_tuple = (window.get_value(0), window.get_value(1), window.get_value(2));
        println!("[Parser] Matching slice: {:?}", value_tuple);

        let (parsed, advance) = match value_tuple
        {
            (Some(Token::Newline), _, _) => (None, 1),
            // _ => (Some(parse_statement(&mut window)?.as_ref().clone()), 0),
            _ => {
                let statement = match parse_statement(&mut window)
                {
                    Ok(stat) => stat.as_ref().clone(),
                    Err(error) => {
                        println!("Erroring out. Current collected outputs: {:?}", output_vec);
                        Err(error)?
                    },
                };
                (Some(statement), 0)
            }
        };

        if let Some(stat) = parsed
        {
            println!("Parsed statement:\n{:?}", stat);
            output_vec.push(stat);
        }

        window.move_view(advance);
    }

    Ok(output_vec)
}

fn parse_statement(window: &mut VecWindow<Token>) -> Result<Box<Stat>, StatError>
{
    let value_tuple = (window.get_value(0), window.get_value(1), window.get_value(2));
    println!("[Parse Stat] Matching slice: {:?}", value_tuple);
    let statement = match value_tuple
    {
        (Some(Token::Identifier(tok)), _, _) if tok.to_ascii_lowercase() == "goto" => {
            window.move_view(3);
            Stat::Goto(parse_expression(window)?)
        },

        (Some(Token::Identifier(tok)), _, _) if tok.to_ascii_lowercase() == "if" => {
            window.move_view(1);
            extend_if(window)?.as_ref().clone()
        },

        (Some(ident @ Token::Identifier(_)), Some(Token::Plus), Some(Token::Equal)) |
        (Some(ident @ Token::DataField(_)), Some(Token::Plus), Some(Token::Equal)) => {
            let cloned_ident = ident.clone();
            window.move_view(3);
            Stat::Assignment(cloned_ident, Op::AddAssign, parse_expression(window)?)
        },

        (Some(ident @ Token::Identifier(_)), Some(Token::Minus), Some(Token::Equal)) |
        (Some(ident @ Token::DataField(_)), Some(Token::Minus), Some(Token::Equal)) => {
            let cloned_ident = ident.clone();
            window.move_view(3);
            Stat::Assignment(cloned_ident, Op::SubAssign, parse_expression(window)?)
        },

        (Some(ident @ Token::Identifier(_)), Some(Token::Star), Some(Token::Equal)) |
        (Some(ident @ Token::DataField(_)), Some(Token::Star), Some(Token::Equal)) => {
            let cloned_ident = ident.clone();
            window.move_view(3);
            Stat::Assignment(cloned_ident, Op::MulAssign, parse_expression(window)?)
        },

        (Some(ident @ Token::Identifier(_)), Some(Token::Slash), Some(Token::Equal)) |
        (Some(ident @ Token::DataField(_)), Some(Token::Slash), Some(Token::Equal)) => {
            let cloned_ident = ident.clone();
            window.move_view(3);
            Stat::Assignment(cloned_ident, Op::DivAssign, parse_expression(window)?)
        },

        (Some(ident @ Token::Identifier(_)), Some(Token::Percent), Some(Token::Equal)) |
        (Some(ident @ Token::DataField(_)), Some(Token::Percent), Some(Token::Equal)) => {
            let cloned_ident = ident.clone();
            window.move_view(3);
            Stat::Assignment(cloned_ident, Op::ModAssign, parse_expression(window)?)
        },

        (Some(ident @ Token::Identifier(_)), Some(Token::Caret), Some(Token::Equal)) |
        (Some(ident @ Token::DataField(_)), Some(Token::Caret), Some(Token::Equal)) => {
            let cloned_ident = ident.clone();
            window.move_view(3);
            Stat::Assignment(cloned_ident, Op::PowAssign, parse_expression(window)?)
        },

        (Some(ident @ Token::Identifier(_)), Some(Token::Equal), Some(tok)) |
        (Some(ident @ Token::DataField(_)), Some(Token::Equal), Some(tok)) if *tok != Token::Equal => {
            let cloned_ident = ident.clone();
            window.move_view(2);
            Stat::Assignment(cloned_ident, Op::Assign, parse_expression(window)?)
        },

        _ => Stat::Expression(parse_expression(window)?)
    };

    // Ok(Box::new(Stat::Assignment(Token::Caret, Op::Abs, Box::new(Expr::Value(Token::Caret)))))
    Ok(Box::new(statement))
}

fn extend_if(window: &mut VecWindow<Token>) -> Result<Box<Stat>, StatError>
{
    println!("Parsing if condition...");
    let condition = parse_expression(window)?;
    println!("Finished if condition");

    println!("Vec head: {:?}", window.get_value(0));
    match window.get_value(0)
    {
        Some(Token::Identifier(tok)) if tok.to_ascii_lowercase() == "then" => {
            window.move_view(1);
        },
        
        tok => return Err(StatError::new(None,
                        ParseErrorKind::NoExtensionAvailable,
                        format!("Can't find 'then' to extend if. Found: {:?}", tok).as_ref()))
    }

    let mut body: Vec<Stat> = Vec::new();
    let mut else_body: Vec<Stat> = Vec::new();
    let mut parsing_else = false;

    while window.remaining_length() > 0
    {
        let value_tuple = (window.get_value(0), window.get_value(1));
        let statement = match value_tuple
        {
            (Some(Token::Identifier(tok)), _) if tok.to_ascii_lowercase() == "else" => {
                if parsing_else
                {
                    let error_stat = Stat::If(condition, body, Some(else_body));
                    return Err(StatError::new(Some(Box::new(error_stat)),
                                ParseErrorKind::RepeatedElseTokens,
                                "Found an else token after already finding one for this if!"))
                }
                window.move_view(1);
                parsing_else = true;
                continue
            },
            (Some(Token::Identifier(tok)), _) if tok.to_ascii_lowercase() == "end" => {
                window.move_view(1);
                break
            },

            _ => parse_statement(window)?.as_ref().clone()
        };

        if parsing_else
        {
            else_body.push(statement)
        }
        else
        {
            body.push(statement)
        }
    }

    let final_else = if else_body.len() > 0
    {
        Some(else_body)
    }
    else
    {
        None
    };

    Ok(Box::new(Stat::If(condition, body, final_else)))
}

fn parse_expression(window: &mut VecWindow<Token>) -> Result<Box<Expr>, ExprError>
{
    let value_tuple = (window.get_value(0), window.get_value(1), window.get_value(2));
    println!("[Parse Expr] Matching tuple: {:?}", value_tuple);
    let expression = match value_tuple
    {
        (Some(Token::LParen), _, _) => {
            window.move_view(1);
            println!("Saw LParen, recursing for group parsing...");
            let output = parse_expression(window)?;

            // println!("After group inner expression parse attempt. Vec head: {:?}", window.get_value(0));

            // match output
            // {
            //     Ok(expr) => {},
            //     Err(ExprError { input_expr: expr, ParseErrorKind::NoParseRuleMatch, ... }) => {},
            // }

            match window.get_value(0)
            {
                Some(Token::RParen) => {
                    window.move_view(1);
                    Expr::Grouping(output)
                },
                _ => return Err(ExprError::new(Some(output), ParseErrorKind::UnbalancedParenthesis, "Saw LParen, parsed expr, found no RParen!"))
            }
        },

        (Some(Token::Minus), Some(Token::Minus), _) => {
            window.move_view(2);
            Expr::UnaryOp(Op::PreDec, parse_expression(window)?)
        },
        (Some(Token::Plus), Some(Token::Plus), _) =>  {
            window.move_view(2);
            Expr::UnaryOp(Op::PreInc, parse_expression(window)?)
        },
        (Some(Token::Minus), _, _) =>  {
            window.move_view(1);
            Expr::UnaryOp(Op::Negate, parse_expression(window)?)
        },

        (Some(Token::Identifier(tok)), _, _) if tok.to_ascii_lowercase() == "abs" => {
            window.move_view(1);
            Expr::UnaryOp(Op::Abs, parse_expression(window)?)
        },
        (Some(Token::Identifier(tok)), _, _) if tok.to_ascii_lowercase() == "sqrt" => {
            window.move_view(1);
            Expr::UnaryOp(Op::Sqrt, parse_expression(window)?)
        },
        (Some(Token::Identifier(tok)), _, _) if tok.to_ascii_lowercase() == "sin" => {
            window.move_view(1);
            Expr::UnaryOp(Op::Sin, parse_expression(window)?)
        },
        (Some(Token::Identifier(tok)), _, _) if tok.to_ascii_lowercase() == "cos" => {
            window.move_view(1);
            Expr::UnaryOp(Op::Cos, parse_expression(window)?)
        },
        (Some(Token::Identifier(tok)), _, _) if tok.to_ascii_lowercase() == "tan" => {
            window.move_view(1);
            Expr::UnaryOp(Op::Tan, parse_expression(window)?)
        },
        (Some(Token::Identifier(tok)), _, _) if tok.to_ascii_lowercase() == "arcsin" => {
            window.move_view(1);
            Expr::UnaryOp(Op::Arcsin, parse_expression(window)?)
        },
        (Some(Token::Identifier(tok)), _, _) if tok.to_ascii_lowercase() == "arccos" => {
            window.move_view(1);
            Expr::UnaryOp(Op::Arccos, parse_expression(window)?)
        },
        (Some(Token::Identifier(tok)), _, _) if tok.to_ascii_lowercase() == "arctan" => {
            window.move_view(1);
            Expr::UnaryOp(Op::Arctan, parse_expression(window)?)
        },
        (Some(Token::Identifier(tok)), _, _) if tok.to_ascii_lowercase() == "not" => {
            window.move_view(1);
            Expr::UnaryOp(Op::Not, parse_expression(window)?)
        },

        (Some(tok @ Token::StringToken(_)), _, _) |
        (Some(tok @ Token::YololNum(_)), _, _) |
        (Some(tok @ Token::Identifier(_)), _, _) |
        (Some(tok @ Token::DataField(_)), _, _) => {
            let new_tok = tok.clone();
            window.move_view(1);
            Expr::Value(new_tok)
        },

        tok => { println!("[Parse Expr] No match, returning..."); return Err(ExprError::new(None, ParseErrorKind::NoParseRuleMatch, format!("Parsing {:?} and didn't match", tok).as_str())) }
    };

    let expression = match extend_expression(Box::new(expression), window)
    {
        Ok(expr) |
        Err(ExprError { input_expr: Some(expr), kind: ParseErrorKind::NoExtensionAvailable, .. }) => expr,
        _ => panic!("Unknown extend expression error!")
    };

    println!("[Parse Expr] Returning parsed expr {:?}", expression.as_ref());
    Ok(expression)
}

// Best way to do this is to have an "expr error" that returns an error _and_ moves ownership of the input expr
// back out to the function that receives the error
fn extend_expression(expr: Box<Expr>, window: &mut VecWindow<Token>) -> Result<Box<Expr>, ExprError>
{
    let value_tuple = (window.get_value(0), window.get_value(1), window.get_value(2));
    println!("[Extend Expr] Matching tuple: {:?}", value_tuple);
    let expression = match value_tuple
    {
        // Post increment and post decrement
        (Some(Token::Plus), Some(Token::Plus), _) => {
            window.move_view(2);
            Expr::UnaryOp(Op::PostInc, expr)
        },
        (Some(Token::Minus), Some(Token::Minus), _) => {
            window.move_view(2);
            Expr::UnaryOp(Op::PostDec, expr)
        },

        // Less than, greater than, and derivatives
        (Some(Token::LAngleBrak), Some(Token::Equal), _) => {
            window.move_view(2);
            Expr::BinaryOp(Op::LesserEq, expr, parse_expression(window)?)
        },
        (Some(Token::RAngleBrak), Some(Token::Equal), _) => {
            window.move_view(2);
            Expr::BinaryOp(Op::GreaterEq, expr, parse_expression(window)?)
        },
        (Some(Token::LAngleBrak), _, _) => {
            window.move_view(1);
            Expr::BinaryOp(Op::Lesser, expr, parse_expression(window)?)
        },
        (Some(Token::RAngleBrak), _, _) => {
            window.move_view(1);
            Expr::BinaryOp(Op::Greater, expr, parse_expression(window)?)
        },

        // Equality and inverse equality matching
        (Some(Token::Equal), Some(Token::Equal), _) => {
            window.move_view(2);
            Expr::BinaryOp(Op::Equal, expr, parse_expression(window)?)
        },
        (Some(Token::Exclam), Some(Token::Equal), _) => {
            window.move_view(2);
            Expr::BinaryOp(Op::NotEqual, expr, parse_expression(window)?)
        },

        // Factorial matching
        (Some(Token::Exclam), _, _) => {
            window.move_view(1);
            Expr::UnaryOp(Op::Fact, expr)
        },

        // Logical and/or matching
        (Some(Token::Identifier(tok)), _, _) if tok.to_ascii_lowercase() == "and" => {
            window.move_view(1);
            Expr::BinaryOp(Op::And, expr, parse_expression(window)?)
        },
        (Some(Token::Identifier(tok)), _, _) if tok.to_ascii_lowercase() == "or" => {
            window.move_view(1);
            Expr::BinaryOp(Op::Or, expr, parse_expression(window)?)
        },

        // Infix operator matching
        (Some(Token::Plus), _, _) => {
            window.move_view(1);
            Expr::BinaryOp(Op::Add, expr, parse_expression(window)?)
        },
        (Some(Token::Minus), _, _) => {
            window.move_view(1);
            Expr::BinaryOp(Op::Sub, expr, parse_expression(window)?)
        },
        (Some(Token::Star), _, _) => {
            window.move_view(1);
            Expr::BinaryOp(Op::Mul, expr, parse_expression(window)?)
        },
        (Some(Token::Slash), _, _) => {
            window.move_view(1);
            Expr::BinaryOp(Op::Div, expr, parse_expression(window)?)
        },
        (Some(Token::Percent), _, _) => {
            window.move_view(1);
            Expr::BinaryOp(Op::Mod, expr, parse_expression(window)?)
        },
        (Some(Token::Caret), _, _) => {
            window.move_view(1);
            Expr::BinaryOp(Op::Pow, expr, parse_expression(window)?)
        },

        tok => { println!("[Extend Expr] No match, returning..."); return Err(ExprError::new(Some(expr), ParseErrorKind::NoExtensionAvailable, format!("Extending and didn't match on {:?}", tok).as_str())) }
    };

    let expression = match extend_expression(Box::new(expression), window)
    {
        Ok(expr) => expr,
        Err(ExprError { input_expr: Some(expr), kind: ParseErrorKind::NoExtensionAvailable, error_text: error }) => expr,
        _ => panic!("Unknown extend expression error!")
    };

    println!("[Extend Expr] Returning: {:?}", expression.as_ref());

    Ok(expression)
}


