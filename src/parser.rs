use crate::types::Token;
use crate::types::Statement as Stat;
use crate::types::Expression as Expr;
use crate::types::Operator as Op;

use crate::types::Value;

use crate::types::ParseErrorKind;
use crate::types::ExprError;
use crate::types::StatError;

use crate::types::SlidingWindow;
use crate::types::VecWindow;

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
            // This eventually needs to have meaning in ending the current line
            (Some(Token::Newline), _, _) => (None, 1),

            _ => {
                let statement = match parse_statement(&mut window)
                {
                    Ok(stat) => stat,
                    Err(error) => {
                        println!("Erroring out. Current collected outputs: {:?}", output_vec);
                        return Err(error);
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

fn parse_statement(window: &mut VecWindow<Token>) -> Result<Stat, StatError>
{
    let value_tuple = (window.get_value(0), window.get_value(1), window.get_value(2));
    println!("[Parse Stat] Matching slice: {:?}", value_tuple);
    let statement = match value_tuple
    {
        (Some(Token::Comment(comment)), _, _) => {
            let comment_string = comment.clone();
            window.move_view(1);
            Stat::Comment(comment_string)
        } 

        (Some(Token::Goto), _, _) => {
            window.move_view(1);
            Stat::Goto(parse_expression(window)?)
        },

        (Some(Token::If), _, _) => {
            window.move_view(1);
            extend_if(window)?
        },

        (Some(ident @ Token::Identifier(_)), Some(Token::Plus), Some(Token::Equal)) => {
            let value = Value::from(ident.clone());
            window.move_view(3);
            Stat::Assignment(value, Op::AddAssign, parse_expression(window)?)
        },

        (Some(ident @ Token::Identifier(_)), Some(Token::Minus), Some(Token::Equal)) => {
            let value = Value::from(ident.clone());
            window.move_view(3);
            Stat::Assignment(value, Op::SubAssign, parse_expression(window)?)
        },

        (Some(ident @ Token::Identifier(_)), Some(Token::Star), Some(Token::Equal)) => {
            let value = Value::from(ident.clone());
            window.move_view(3);
            Stat::Assignment(value, Op::MulAssign, parse_expression(window)?)
        },

        (Some(ident @ Token::Identifier(_)), Some(Token::Slash), Some(Token::Equal)) => {
            let value = Value::from(ident.clone());
            window.move_view(3);
            Stat::Assignment(value, Op::DivAssign, parse_expression(window)?)
        },

        (Some(ident @ Token::Identifier(_)), Some(Token::Percent), Some(Token::Equal)) => {
            let value = Value::from(ident.clone());
            window.move_view(3);
            Stat::Assignment(value, Op::ModAssign, parse_expression(window)?)
        },

        (Some(ident @ Token::Identifier(_)), Some(Token::Equal), Some(tok)) if *tok != Token::Equal => {
            let value = Value::from(ident.clone());
            window.move_view(2);
            Stat::Assignment(value, Op::Assign, parse_expression(window)?)
        },

        _ => Stat::Expression(parse_expression(window)?)
    };

    // Ok(Box::new(Stat::Assignment(Token::Caret, Op::Abs, Box::new(Expr::Value(Token::Caret)))))
    Ok(statement)
}

fn extend_if(window: &mut VecWindow<Token>) -> Result<Stat, StatError>
{
    println!("Parsing if condition...");
    let condition = parse_expression(window)?;
    println!("Finished if condition");

    println!("Vec head: {:?}", window.get_value(0));
    match window.get_value(0)
    {
        Some(Token::Then) => {
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
            (Some(Token::Else), _) => {
                if parsing_else
                {
                    let error_stat = Stat::If(condition, body, Some(else_body));
                    return Err(StatError::new(Some(error_stat),
                                ParseErrorKind::RepeatedElseTokens,
                                "Found an else token after already finding one for this if!"))
                }
                window.move_view(1);
                parsing_else = true;
                continue
            },
            (Some(Token::End), _) => {
                window.move_view(1);
                break
            },

            _ => parse_statement(window)?
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

    let final_else = if else_body.is_empty() == false
    {
        Some(else_body)
    }
    else
    {
        None
    };

    Ok(Stat::If(condition, body, final_else))
}

fn parse_expression(window: &mut VecWindow<Token>) -> Result<Box<Expr>, ExprError>
{
    Ok(Box::new(expr_and(window)?))
}

fn expr_and(window: &mut VecWindow<Token>) -> Result<Expr, ExprError>
{
    println!("[expr_and] Matching slice {:?}", (window.get_value(0), window.get_value(1), window.get_value(2)));
    match expr_or(window)
    {
        // The lower rule did match, so attempt to extend
        Ok(expr) => {
            match window.get_value(0)
            {
                Some(Token::And) => {
                    extend_and(expr, window)
                },

                _ => Ok(expr)
            }
        },
        // An error occurred in a lower rule, this is bad, throw back up the error
        error @ Err(_) => {
            error
        },
    }
}

fn extend_and(left: Expr, window: &mut VecWindow<Token>) -> Result<Expr, ExprError>
{
    let op = match window.get_value(0)
    {
        Some(Token::And) => {
            Op::And
        },

        _ => return Ok(left)
    };

    window.move_view(1);
    match expr_or(window)
    {
        Ok(right) => {
            // Found a right hand side for our rule, so construct the object
            let expr = Expr::BinaryOp(op, Box::new(left), Box::new(right));
            extend_and(expr, window)
        }

        _ => Err(ExprError::new(Some(left), ParseErrorKind::NoExtensionAvailable, "Syntax error in parsing an and!"))
    }
}

fn expr_or(window: &mut VecWindow<Token>) -> Result<Expr, ExprError>
{
    println!("[expr_or] Matching slice {:?}", (window.get_value(0), window.get_value(1), window.get_value(2)));
    match expr_equality(window)
    {
        // The lower rule did match, so attempt to extend
        Ok(expr) => {
            match window.get_value(0)
            {
                Some(Token::Or) => {
                    extend_or(expr, window)
                },

                _ => Ok(expr)
            }
        },
        // An error occurred in a lower rule, this is bad, throw back up the error
        error @ Err(_) => {
            error
        },
    }
}

fn extend_or(left: Expr, window: &mut VecWindow<Token>) -> Result<Expr, ExprError>
{
    let op = match window.get_value(0)
    {
        Some(Token::Or) => {
            Op::Or
        },

        _ => return Ok(left)
    };

    window.move_view(1);
    match expr_equality(window)
    {
        Ok(right) => {
            // Found a right hand side for our rule, so construct the object
            let expr = Expr::BinaryOp(op, Box::new(left), Box::new(right));
            extend_or(expr, window)
        }

        _ => Err(ExprError::new(Some(left), ParseErrorKind::NoExtensionAvailable, "Syntax error in parsing an or!"))
    }
}

fn expr_equality(window: &mut VecWindow<Token>) -> Result<Expr, ExprError>
{
    println!("[expr_equality] Matching slice {:?}", (window.get_value(0), window.get_value(1), window.get_value(2)));
    match expr_order(window)
    {
        // The lower rule did match, so attempt to extend
        Ok(expr) => {
            let value_tuple = (window.get_value(0), window.get_value(1));
            match value_tuple
            {
                (Some(Token::Equal), Some(Token::Equal)) |
                (Some(Token::Exclam), Some(Token::Equal)) => {
                    extend_equality(expr, window)
                },

                _ => Ok(expr)
            }
        },
        // An error occurred in a lower rule, this is bad, throw back up the error
        error @ Err(_) => {
            error
        },
    }
}

fn extend_equality(left: Expr, window: &mut VecWindow<Token>) -> Result<Expr, ExprError>
{
    let op = match (window.get_value(0), window.get_value(1))
    {
        (Some(Token::Equal), Some(Token::Equal)) => {
            Op::Equal
        },

        (Some(Token::Exclam), Some(Token::Equal)) => {
            Op::NotEqual
        },

        _ => return Ok(left)
    };

    window.move_view(2);
    match expr_order(window)
    {
        Ok(right) => {
            // Found a right hand side for our rule, so construct the object
            let expr = Expr::BinaryOp(op, Box::new(left), Box::new(right));
            extend_equality(expr, window)
        }

        _ => Err(ExprError::new(Some(left), ParseErrorKind::NoExtensionAvailable, "Syntax error in parsing an equality!"))
    }
}

fn expr_order(window: &mut VecWindow<Token>) -> Result<Expr, ExprError>
{
    println!("[expr_order] Matching slice {:?}", (window.get_value(0), window.get_value(1), window.get_value(2)));
    match expr_additive(window)
    {
        // The lower rule did match, so attempt to extend
        Ok(expr) => {
            let value_tuple = (window.get_value(0), window.get_value(1));
            match value_tuple
            {
                (Some(Token::LAngleBrak), Some(Token::Equal)) |
                (Some(Token::LAngleBrak), _) |
                (Some(Token::RAngleBrak), Some(Token::Equal)) |
                (Some(Token::RAngleBrak), _) => {
                    extend_order(expr, window)
                },

                _ => Ok(expr)
            }
        },
        // An error occurred in a lower rule, this is bad, throw back up the error
        error @ Err(_) => {
            error
        },
    }
}

fn extend_order(left: Expr, window: &mut VecWindow<Token>) -> Result<Expr, ExprError>
{
    let op = match (window.get_value(0), window.get_value(1))
    {
        (Some(Token::LAngleBrak), Some(Token::Equal)) => {
            window.move_view(2);
            Op::LesserEq
        },
        (Some(Token::LAngleBrak), _) => {
            window.move_view(1);
            Op::Lesser
        },

        (Some(Token::RAngleBrak), Some(Token::Equal)) => {
            window.move_view(2);
            Op::GreaterEq
        },
        (Some(Token::RAngleBrak), _) => {
            window.move_view(1);
            Op::Greater
        },

        _ => return Ok(left)
    };

    match expr_additive(window)
    {
        Ok(right) => {
            // Found a right hand side for our rule, so construct the object
            let expr = Expr::BinaryOp(op, Box::new(left), Box::new(right));
            extend_order(expr, window)
        }

        _ => Err(ExprError::new(Some(left), ParseErrorKind::NoExtensionAvailable, "Syntax error in parsing an order!"))
    }
}

fn expr_additive(window: &mut VecWindow<Token>) -> Result<Expr, ExprError>
{
    println!("[expr_additive] Matching slice {:?}", (window.get_value(0), window.get_value(1), window.get_value(2)));
    match expr_multiply(window)
    {
        // The lower rule did match, so attempt to extend
        Ok(expr) => {
            match window.get_value(0)
            {
                Some(Token::Plus) |
                Some(Token::Minus) => {
                    extend_additive(expr, window)
                }

                _ => Ok(expr)
            }
        },
        // An error occurred in a lower rule, this is bad, throw back up the error
        error @ Err(_) => {
            error
        },
    }
}

fn extend_additive(left: Expr, window: &mut VecWindow<Token>) -> Result<Expr, ExprError>
{
    let op = match window.get_value(0)
    {
        Some(Token::Plus) => {
            Op::Add
        },

        Some(Token::Minus) => {
            Op::Sub
        },

        _ => return Ok(left)
    };

    window.move_view(1);
    match expr_multiply(window)
    {
        Ok(right) => {
            // Found a right hand side for our rule, so construct the object
            let expr = Expr::BinaryOp(op, Box::new(left), Box::new(right));
            extend_additive(expr, window)
        }

        _ => Err(ExprError::new(Some(left), ParseErrorKind::NoExtensionAvailable, "Syntax error in parsing an additive!"))
    }
}

fn expr_multiply(window: &mut VecWindow<Token>) -> Result<Expr, ExprError>
{
    println!("[expr_multiply] Matching slice {:?}", (window.get_value(0), window.get_value(1), window.get_value(2)));
    match expr_exponent(window)
    {
        // The lower rule did match, so attempt to extend
        Ok(expr) => {
            match window.get_value(0)
            {
                Some(Token::Slash) |
                Some(Token::Star)  |
                Some(Token::Percent) => {
                    extend_multiply(expr, window)
                },

                _ => Ok(expr)
            }
        },
        // An error occurred in a lower rule, this is bad, throw back up the error
        error @ Err(_) => {
            error
        },
    }
}

fn extend_multiply(left: Expr, window: &mut VecWindow<Token>) -> Result<Expr, ExprError>
{
    let op = match window.get_value(0)
    {
        Some(Token::Slash) => {
            Op::Div
        },

        Some(Token::Star) => {
            Op::Mul
        },

        Some(Token::Percent) => {
            Op::Mod
        },

        _ => return Ok(left)
    };

    window.move_view(1);
    match expr_exponent(window)
    {
        Ok(right) => {
            // Found a right hand side for our rule, so construct the object
            let expr = Expr::BinaryOp(op, Box::new(left), Box::new(right));
            extend_multiply(expr, window)
        }

        _ => Err(ExprError::new(Some(left), ParseErrorKind::NoExtensionAvailable, "Syntax error in parsing a multiply!"))
    }
}

// Doesn't use extension idiom due to being right associative
fn expr_exponent(window: &mut VecWindow<Token>) -> Result<Expr, ExprError>
{
    println!("[expr_exponent] Matching slice {:?}", (window.get_value(0), window.get_value(1), window.get_value(2)));
    match expr_postfix(window)
    {
        // The lower rule did match, so attempt to extend
        Ok(expr) => {
            match window.get_value(0)
            {
                Some(Token::Caret) => {
                    window.move_view(1);
                    let left = Box::new(expr);
                    let right = Box::new(expr_exponent(window)?);

                    Ok(Expr::BinaryOp(Op::Pow, left, right))
                },
                _ => Ok(expr)
            }
        },
        // An error occurred in a lower rule, this is bad, throw back up the error
        error @ Err(_) => {
            error
        },
    }
}

fn expr_postfix(window: &mut VecWindow<Token>) -> Result<Expr, ExprError>
{
    println!("[expr_postfix] Matching slice {:?}", (window.get_value(0), window.get_value(1), window.get_value(2)));
    match expr_keyword(window)
    {
        // The lower rule did match, so attempt to extend to form this rule
        Ok(expr) => Ok(extend_postfix(expr, window)),

        // An error occurred in a lower rule, this is bad, throw back up the error
        error @ Err(_) => {
            error
        },
    }
}

fn extend_postfix(expr: Expr, window: &mut VecWindow<Token>) -> Expr
{
    match window.get_value(0)
    {
        Some(Token::Exclam) => {
            window.move_view(1);
            extend_postfix(Expr::UnaryOp(Op::Fact, Box::new(expr)), window)
        }
        _ => expr
    }
}

fn expr_keyword(window: &mut VecWindow<Token>) -> Result<Expr, ExprError>
{
    println!("[expr_keyword] Matching slice {:?}", (window.get_value(0), window.get_value(1), window.get_value(2)));
    match expr_neg(window)
    {
        // The rule below simply didn't match onto the window, so now it's our turn
        Err(ExprError { kind: ParseErrorKind::NoParseRuleMatch, .. }) => {
            match window.get_value(0)
            {
                Some(Token::Abs) => {
                    window.move_view(1);
                    let operand = Box::new(expr_keyword(window)?);

                    Ok(Expr::UnaryOp(Op::Abs, operand))
                },
                Some(Token::Sqrt) => {
                    window.move_view(1);
                    let operand = Box::new(expr_keyword(window)?);

                    Ok(Expr::UnaryOp(Op::Sqrt, operand))
                },
                Some(Token::Sin) => {
                    window.move_view(1);
                    let operand = Box::new(expr_keyword(window)?);

                    Ok(Expr::UnaryOp(Op::Sin, operand))
                },
                Some(Token::Cos) => {
                    window.move_view(1);
                    let operand = Box::new(expr_keyword(window)?);

                    Ok(Expr::UnaryOp(Op::Cos, operand))
                },
                Some(Token::Tan) => {
                    window.move_view(1);
                    let operand = Box::new(expr_keyword(window)?);

                    Ok(Expr::UnaryOp(Op::Tan, operand))
                },
                Some(Token::Arcsin) => {
                    window.move_view(1);
                    let operand = Box::new(expr_keyword(window)?);

                    Ok(Expr::UnaryOp(Op::Arcsin, operand))
                },
                Some(Token::Arccos) => {
                    window.move_view(1);
                    let operand = Box::new(expr_keyword(window)?);

                    Ok(Expr::UnaryOp(Op::Arccos, operand))
                },
                Some(Token::Arctan) => {
                    window.move_view(1);
                    let operand = Box::new(expr_keyword(window)?);

                    Ok(Expr::UnaryOp(Op::Arctan, operand))
                },
                Some(Token::Not) => {
                    window.move_view(1);
                    let operand = Box::new(expr_keyword(window)?);

                    Ok(Expr::UnaryOp(Op::Not, operand))
                },

                _ => Err(ExprError::new(None,
                        ParseErrorKind::NoParseRuleMatch,
                        "In expr_keyword, can't find keyword operator after lower rule failed to match!"))
            }
        },
        // The lower rule did match, so just pass back up the expression it created
        expr @ Ok(_) => {
            expr
        }
        // A different error occurred in a lower rule, this is bad, throw back up the error
        error @ Err(_) => {
            error
        },
    }
}

fn expr_neg(window: &mut VecWindow<Token>) -> Result<Expr, ExprError>
{
    println!("[expr_neg] Matching slice {:?}", (window.get_value(0), window.get_value(1), window.get_value(2)));
    match expr_ident(window)
    {
        // The rule below simply didn't match onto the window, so now it's our turn
        Err(ExprError { kind: ParseErrorKind::NoParseRuleMatch, .. }) => {
            match window.get_value(0)
            {
                Some(Token::Minus) => {
                    window.move_view(1);
                    let operand = Box::new(expr_neg(window)?);

                    Ok(Expr::UnaryOp(Op::Negate, operand))
                }

                _ => {
                    Err(ExprError::new(None,
                        ParseErrorKind::NoParseRuleMatch,
                        "In expr_neg, can't find minus after lower rule failed to match!"))
                }
            }
        },
        // The lower rule did match, so just pass back up the expression it created
        expr @ Ok(_) => {
            expr
        }
        // A different error occurred in a lower rule, this is bad, throw back up the error
        error @ Err(_) => {
            error
        },
    }
}

fn expr_ident(window: &mut VecWindow<Token>) -> Result<Expr, ExprError>
{
    let value_tuple = (window.get_value(0), window.get_value(1), window.get_value(2));
    println!("[expr_ident] Matching slice {:?}", value_tuple);   
    let expr = match value_tuple
    {
        // Postfix inc/dec operator parsing
        (Some(ident @ Token::Identifier(_)), Some(Token::Plus), Some(Token::Plus)) => {
            let value = Value::from(ident.clone());
            window.move_view(3);
            
            Expr::UnaryOp(Op::PostInc, Box::new(Expr::Value(value)))
        },
        (Some(ident @ Token::Identifier(_)), Some(Token::Minus), Some(Token::Minus)) => {
            let value = Value::from(ident.clone());
            window.move_view(3);

            Expr::UnaryOp(Op::PostDec, Box::new(Expr::Value(value)))
        },

        // Prefix inc/dec operator parsing
        (Some(Token::Plus), Some(Token::Plus), Some(ident @ Token::Identifier(_))) => {
            let value = Value::from(ident.clone());
            window.move_view(3);

            Expr::UnaryOp(Op::PreInc, Box::new(Expr::Value(value)))
        },
        (Some(Token::Minus), Some(Token::Minus), Some(ident @ Token::Identifier(_))) => {
            let value = Value::from(ident.clone());
            window.move_view(3);

            Expr::UnaryOp(Op::PreDec, Box::new(Expr::Value(value)))
        },

        // Parses into any value, which is then wrapped into an expression
        _ => Expr::Value(parse_value(window)?)
    };

    Ok(expr)
}


fn parse_value(window: &mut VecWindow<Token>) -> Result<Value, ExprError>
{
    match window.get_value(0)
    {
        Some(tok @ Token::StringToken(_)) |
        Some(tok @ Token::YololNum(_)) |
        Some(tok @ Token::Identifier(_)) => {
            let tok = tok.clone();
            println!("Parsing value {:?}", tok);
            window.move_view(1);
            println!("Window after value parse: {:?}", (window.get_value(0), window.get_value(1), window.get_value(2)));

            Ok(Value::from(tok))
        },

        Some(Token::LParen) => {
            window.move_view(1);
            println!("Saw LParen, starting group parsing...");
            let output = parse_expression(window)?;

            match window.get_value(0)
            {
                Some(Token::RParen) => {
                    window.move_view(1);
                    Ok(Value::Group(output))
                },

                _ => Err(ExprError::new(Some(*output), ParseErrorKind::UnbalancedParenthesis, "Saw LParen, parsed expr, found no RParen!"))
            }
        },

        _ => Err(ExprError::new(None, ParseErrorKind::NoParseRuleMatch, "No match while parsing value!"))
    }
}


