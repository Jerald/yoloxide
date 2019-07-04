use crate::types::Token;

use crate::types::SlidingWindow;
use crate::types::VecWindow;
use crate::types::YololNumber;

use std::convert::TryInto;

pub fn tokenize_basic(input: String) -> Result<Vec<Token>, String>
{
    let mut output_vec: Vec<Token> = Vec::new();
    let input_chars: Vec<char> = input.chars().collect();

    let mut window = VecWindow::new(&input_chars, 0);

    while window.remaining_length() > 0
    {
        let value_tuple = (window.get_value(0), window.get_value(1), window.get_value(2));

        println!("Matching slice: {:?}", (window.get_value(0), window.get_value(1), window.get_value(2)));

        let (token, advance) = match value_tuple
        {
            // Comment. Everything from '//' to the end of the line
            (Some('/'), Some('/'), _)           => (extend_comment(&mut window), 0),

            // Identifier. Starts with an alpha, then can be alphanum
            (Some('a'..='z'), _, _) |
            (Some('A'..='Z'), _, _)             => (extend_alphanum(&mut window), 0),

            // DataField. Starts with a colon then is all alphanums
            (Some(':'), Some('0'..='9'), _) |
            (Some(':'), Some('a'..='z'), _) |
            (Some(':'), Some('A'..='Z'), _)     => (extend_alphanum(&mut window), 0),

            // String. Starts with a quote then extends all normal ascii chars until another quote
            (Some('"'), Some(' '..='~'), _)     => (extend_string(&mut window), 0),

            // YololNumber. Starts with a number extends through all other numbers
            // Will match on periods so it can represent the YololNumber decimals
            (Some('0'..='9'), _, _)             => (extend_yololnum(&mut window), 0),
            
            // Newline. Matches on CRLF or LF
            (Some('\r'), Some('\n'), _ ) => (Some(Token::Newline), 2),
            (Some('\n'), _, _) => (Some(Token::Newline), 1),

            // Special chars. Matches on each relevant special char
            (Some('='), _, _)  => (Some(Token::Equal), 1),
            (Some('+'), _, _)  => (Some(Token::Plus), 1),
            (Some('-'), _, _)  => (Some(Token::Minus), 1),
            (Some('*'), _, _)  => (Some(Token::Star), 1),
            (Some('/'), _, _)  => (Some(Token::Slash), 1),
            (Some('('), _, _)  => (Some(Token::LParen), 1),
            (Some(')'), _, _)  => (Some(Token::RParen), 1),
            (Some('<'), _, _)  => (Some(Token::LAngleBrak), 1),
            (Some('>'), _, _)  => (Some(Token::RAngleBrak), 1),
            (Some('!'), _, _)  => (Some(Token::Exclam), 1),
            (Some('^'), _, _)  => (Some(Token::Caret), 1),
            (Some('%'), _, _)  => (Some(Token::Percent), 1),

            (Some(' '), _, _) => (None, 1),

            // Matches on anything else. Returns an error and prints the window that failed matching
            c => { println!("Error on: {:?}", c); return Err(String::from("Bad things happening!")); }
        };

        if let Some(tok) = token
        {
            output_vec.push(tok);
        }

        window.move_view(advance);
    }

    Ok(output_vec)
}

fn extend_comment(window: &mut VecWindow<char>) -> Option<Token>
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

fn extend_alphanum(window: &mut VecWindow<char>) -> Option<Token>
{
    let mut char_vec: Vec<char> = Vec::new();

    let extending_data_field = if let Some(':') = window.get_value(0)
    {
        window.move_view(1);
        char_vec.push(':');
        true
    }
    else
    {
        false
    };

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

    if extending_data_field
    {
        Some(Token::DataField(output))
    }
    else
    {
        Some(Token::Identifier(output))
    }
}

fn extend_string(window: &mut VecWindow<char>) -> Option<Token>
{
    let mut char_vec: Vec<char> = Vec::new();

    if let Some('"') = window.get_value(0)
    {
        window.move_view(1);
    }

    while window.remaining_length() > 0
    {
        match window.get_value(0)
        {
            Some('"') => {
                window.move_view(1);
                break;
            }

            Some(&c @ ' '..='~') => char_vec.push(c),

            _ => break
        };

        window.move_view(1);
    }

    let output: String = char_vec.into_iter().collect();
    Some(Token::StringToken(output))
}

fn extend_yololnum(window: &mut VecWindow<char>) -> Option<Token>
{
    let mut left_digits: Vec<char> = Vec::new();
    let mut right_digits: Vec<char> = Vec::new();

    let mut decimal_hit = false;

    while window.remaining_length() > 0
    {
        match window.get_value(0)
        {
            Some('.') => decimal_hit = true,

            Some(&num @ '0'..='9') if decimal_hit => right_digits.push(num),
            Some(&num @ '0'..='9') => left_digits.push(num),

            _ => break
        };

        window.move_view(1);
    }

    let left_string: String = left_digits.into_iter().collect();
    let right_string: String = right_digits.into_iter().collect();

    let left_num: i64 = left_string.parse::<i64>().unwrap();

    println!("right string len: {}", right_string.len());

    let right_num: i64 = if right_string.len() == 0
    {
        0
    }
    else if right_string.len() >= 4
    {
        right_string[0..4].parse::<i64>().unwrap()
    }
    else
    {
        let right_string_len: u32 = right_string.len().try_into().unwrap();
        let shift: i64 = (10i64).pow(4 - right_string_len);
        right_string[0..right_string.len()].parse::<i64>().unwrap() * shift
    };

    println!("Left num: {}, right num: {}, left string: {}, right string: {}", left_num, right_num, left_string, right_string);

    let yolol_num = YololNumber::from_split(left_num, right_num);
    Some(Token::YololNum(yolol_num))
}



