use crate::types::Token;

use crate::types::SlidingWindow;
use crate::types::VecWindow;

pub fn tokenize_basic<'a>(input: String) -> Vec<Token>
{
    let mut output_vec: Vec<Token> = Vec::new();
    let input_chars: Vec<char> = input.chars().collect();

    let mut window = VecWindow::new(&input_chars, 0);

    while window.remaining_length() > 0
    {
        let value_tuple = (window.get_value(0), window.get_value(1), window.get_value(2));
        let mut extended_token = false;

        println!("Matching slice: {:?}", (window.get_value(0), window.get_value(1), window.get_value(2)));

        let token = match value_tuple
        {
            (Some('/'), Some('/'), _) => { extended_token = true; extend_basic_comment(&mut window) },

            (Some('a'..='z'), _, _) |
            (Some('A'..='Z'), _, _) => { extended_token = true; extend_basic_alphanum(&mut window) },

            (Some('0'..='9'), _, _) => {
                extended_token = true;
                // If the last token was a colon, this is a data field access and we can tokenize it as an alphanum
                if let Some(Token::Colon) = output_vec.last()
                { extend_basic_alphanum(&mut window) }
                else
                { extend_basic_num(&mut window) }
            },

            (Some('"'), _, _)  => Some(Token::Quote),
            (Some('='), _, _)  => Some(Token::Equal),
            (Some('+'), _, _)  => Some(Token::Plus),
            (Some('-'), _, _)  => Some(Token::Minus),
            (Some('*'), _, _)  => Some(Token::Star),
            (Some('('), _, _)  => Some(Token::LParen),
            (Some(')'), _, _)  => Some(Token::RParen),
            (Some('.'), _, _)  => Some(Token::Period),
            (Some('<'), _, _)  => Some(Token::LAngleBrak),
            (Some('>'), _, _)  => Some(Token::RAngleBrak),
            (Some('!'), _, _)  => Some(Token::Exclam),
            (Some('%'), _, _)  => Some(Token::Percent),
            (Some(':'), _, _)  => Some(Token::Colon),
            (Some('\n'), _, _) => Some(Token::Newline),

            _                  => None
        };

        if let Some(tok) = token
        {
            output_vec.push(tok);
        }

        // If we didn't extend for this token, advance the view by one
        if extended_token == false
        {
            window.move_view(1);
        }
    }

    output_vec
}

fn extend_basic_comment(window: &mut VecWindow<char>) -> Option<Token>
{
    let mut char_vec: Vec<char> = Vec::new();

    while window.remaining_length() > 0
    {
        match window.get_value(0)
        {
            Some('\n') => break,
            Some(&c) => char_vec.push(c),
            _ => break
        };

        window.move_view(1);
    }

    let output: String = char_vec.into_iter().collect();
    Some(Token::Comment(output))
}

fn extend_basic_alphanum(window: &mut VecWindow<char>) -> Option<Token>
{
    let mut char_vec: Vec<char> = Vec::new();

    while window.remaining_length() > 0
    {
        match window.get_value(0)
        {
            Some(&c @ 'a'..='z') |
            Some(&c @ 'A'..='Z') |
            Some(&c @ '0'..='9') => char_vec.push(c),

            _ => break
        };

        window.move_view(1);
    }

    let output: String = char_vec.into_iter().collect();
    Some(Token::AlphaNumToken(output))
}

fn extend_basic_num(window: &mut VecWindow<char>) -> Option<Token>
{
    let mut digits: Vec<char> = Vec::new();

    while window.remaining_length() > 0
    {
        match window.get_value(0)
        {
            Some(&num @ '0'..='9') => digits.push(num),
            _ => break
        };

        window.move_view(1);
    }

    let number = digits.into_iter().collect();
    Some(Token::NumToken(number))
}

