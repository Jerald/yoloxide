use crate::types::Token;

use crate::types::SlidingWindow;
use crate::types::VecWindow;

use crate::types::YololNumber;

use std::convert::TryInto;

pub fn tokenize_extended(input: Vec<Token>) -> Vec<Token>
{
    let mut output_vec: Vec<Token> = Vec::new();
    let mut window = VecWindow::new(&input, 0);

    while window.remaining_length() > 0
    {
        let value_tuple = (window.get_value(0), window.get_value(1), window.get_value(2));

        println!("Matching slice: {:?}", (window.get_value(0), window.get_value(1), window.get_value(2)));

        let (token, advance) = match value_tuple
        {
            (Some(Token::NumToken(_)), _, _)                    => (tokenize_yololnum(&mut window), 0), 

            (Some(Token::Plus), Some(Token::Plus), _)           => (Some(Token::PlusPlus), 2),
            (Some(Token::Minus), Some(Token::Minus), _)         => (Some(Token::MinusMinus), 2),

            (Some(Token::Plus), Some(Token::Equal), _)          => (Some(Token::PlusEqual), 2),
            (Some(Token::Minus), Some(Token::Equal), _)         => (Some(Token::MinusEqual), 2),

            (Some(Token::Star), Some(Token::Equal), _)          => (Some(Token::StarEqual), 2),
            (Some(Token::Slash), Some(Token::Equal), _)         => (Some(Token::SlashEqual), 2),
            (Some(Token::Percent), Some(Token::Equal), _)       => (Some(Token::PercentEqual), 2),

            (Some(Token::LAngleBrak), Some(Token::Equal), _)    => (Some(Token::LAngleBrakEqual), 2),
            (Some(Token::RAngleBrak), Some(Token::Equal), _)    => (Some(Token::RAngleBrakEqual), 2),

            (Some(Token::Exclam), Some(Token::Equal), _)        => (Some(Token::ExclamEqual), 2),
            (Some(Token::Equal), Some(Token::Equal), _)         => (Some(Token::EqualEqual), 2),

            (Some(Token::AlphaNumToken(tok)), _, _) if tok.to_ascii_lowercase() == "abs" => (Some(Token::Abs), 1),
            (Some(Token::AlphaNumToken(tok)), _, _) if tok.to_ascii_lowercase() == "sqrt" => (Some(Token::Sqrt), 1),

            (Some(Token::AlphaNumToken(tok)), _, _) if tok.to_ascii_lowercase() == "sin" => (Some(Token::Sin), 1),
            (Some(Token::AlphaNumToken(tok)), _, _) if tok.to_ascii_lowercase() == "cos" => (Some(Token::Cos), 1),
            (Some(Token::AlphaNumToken(tok)), _, _) if tok.to_ascii_lowercase() == "tan" => (Some(Token::Tan), 1),

            (Some(Token::AlphaNumToken(tok)), _, _) if tok.to_ascii_lowercase() == "arcsin" => (Some(Token::Arcsin), 1),
            (Some(Token::AlphaNumToken(tok)), _, _) if tok.to_ascii_lowercase() == "arccos" => (Some(Token::Arccos), 1),
            (Some(Token::AlphaNumToken(tok)), _, _) if tok.to_ascii_lowercase() == "arctan" => (Some(Token::Arctan), 1),

            (Some(Token::AlphaNumToken(tok)), _, _) if tok.to_ascii_lowercase() == "goto" => (Some(Token::Goto), 1),
            
            (Some(Token::AlphaNumToken(tok)), _, _) if tok.to_ascii_lowercase() == "if" => (Some(Token::If), 1),
            (Some(Token::AlphaNumToken(tok)), _, _) if tok.to_ascii_lowercase() == "then" => (Some(Token::Then), 1),
            (Some(Token::AlphaNumToken(tok)), _, _) if tok.to_ascii_lowercase() == "end" => (Some(Token::End), 1),

            (Some(token), _, _)                                 => (Some(token.clone()), 1),

            _                                                   => panic!("Bad tokenizing state! Everything _should_ be tokenizable!")
        };

        if let Some(tok) = token
        {
            output_vec.push(tok);
        }

        window.move_view(advance);
    }

    output_vec
}

fn tokenize_yololnum(window: &mut VecWindow<Token>) -> Option<Token>
{
    let mut left_of_decimal: u64;
    let mut right_of_decimal: u64 = 0;

    let value_tuple = (window.get_value(0), window.get_value(1), window.get_value(2));

    match value_tuple
    {
        (Some(Token::NumToken(left)), Some(Token::Period), Some(Token::NumToken(right))) => {
            left_of_decimal = left.parse::<u64>().expect("String to u64 parse error for left in extend_yololnum!");
            
            // Checks if there are more than 4 values to the right of the decimal and ignores the extras
            if right.len() > 4
            {
                right_of_decimal = right[0..4].parse::<u64>().expect("String to u64 parse error for right in extend_yololnum!");
            }
            else
            {
                right_of_decimal = right.parse::<u64>().expect("String to u64 parse error for right in extend_yololnum!");

                // This compensates for the right value being less than 4 digits long
                // It shifts the value over so there are the correct amount of zeros at the end
                let exponent = 4 - right.len();
                right_of_decimal *= (10u64).pow(exponent.try_into().unwrap());
            }

            window.move_view(3);
        },

        (Some(Token::NumToken(left)), _, _) => {
            left_of_decimal = left.parse::<u64>().expect("String to u64 parse error for left (single-case) in extend_yololnum!");
            window.move_view(1);
        },

        _ => panic!("Tried to tokenize_yololnum but didn't match a number pattern! This should be impossible!")
    };

    let yolol_num = YololNumber::from_split(left_of_decimal, right_of_decimal);
    Some(Token::YololNum(yolol_num))
}