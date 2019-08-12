use yolol_number::YololNumber;

use crate::types::Token;

use crate::types::SlidingWindow;
use crate::types::VecWindow;

pub fn tokenize(input: String) -> Result<Vec<Token>, String>
{
    let mut output_vec: Vec<Token> = Vec::new();
    let input_chars: Vec<char> = input.chars().collect();

    let mut window = VecWindow::new(&input_chars, 0);

    while window.remaining_length() > 0
    {
        let value_tuple = (window.get_value(0), window.get_value(1));
        if cfg!(debug_assertions) { println!("[Tokenize] Matching slice: {:?}", value_tuple) }

        let (token, advance) = match value_tuple
        {
            // Comment. Everything from '//' to the end of the line
            (Some('/'), Some('/'))          => (extend_comment(&mut window), 0),

            // Identifier. Starts with an alpha, then can be alphanum
            (Some('_'), _) |
            (Some('a'..='z'), _) |
            (Some('A'..='Z'), _)            => (extend_alphanum(&mut window), 0),

            // DataField. Starts with a colon then is all alphanums
            (Some(':'), Some('_')) |
            (Some(':'), Some('0'..='9')) |
            (Some(':'), Some('a'..='z')) |
            (Some(':'), Some('A'..='Z'))    => (extend_datafield(&mut window), 0),

            // String. Starts with a quote then extends all normal ascii chars until another quote
            (Some('"'), Some(' '..='~'))    => (extend_string(&mut window), 0),

            // YololNumber. Starts with a number extends through all other numbers
            // Will match on periods so it can represent the YololNumber decimals
            (Some('0'..='9'), _)            => (extend_yololnum(&mut window), 0),
            
            // Newline. Matches on CRLF or LF
            (Some('\r'), Some('\n'))        => (Some(Token::Newline), 2),
            (Some('\n'), _)                 => (Some(Token::Newline), 1),

            // Special chars. Matches on each relevant special char
            (Some('='), _)                  => (Some(Token::Equal), 1),
            (Some('+'), _)                  => (Some(Token::Plus), 1),
            (Some('-'), _)                  => (Some(Token::Minus), 1),
            (Some('*'), _)                  => (Some(Token::Star), 1),
            (Some('/'), _)                  => (Some(Token::Slash), 1),
            (Some('('), _)                  => (Some(Token::LParen), 1),
            (Some(')'), _)                  => (Some(Token::RParen), 1),
            (Some('<'), _)                  => (Some(Token::LAngleBrak), 1),
            (Some('>'), _)                  => (Some(Token::RAngleBrak), 1),
            (Some('!'), _)                  => (Some(Token::Exclam), 1),
            (Some('^'), _)                  => (Some(Token::Caret), 1),
            (Some('%'), _)                  => (Some(Token::Percent), 1),

            // Ignores spaces because they don't matter
            (Some(' '), _) => (None, 1),

            // Matches on anything else. Returns an error and prints the window that failed matching
            c => return Err(format!("[Tokenize] Failure to match on {:?}", c))
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

    // Clears out the starting two slashes
    window.move_view(2);

    while window.remaining_length() > 0
    {
        match (window.get_value(0), window.get_value(1))
        {
            (Some('\r'), Some('\n')) |
            (Some('\n'), _) => break,

            (Some(&c), _) => char_vec.push(c),

            _ => break
        };

        window.move_view(1);
    }

    if let Some('\n') = window.get_value(0)
    {
        window.move_view(1);
    }

    let output: String = char_vec.into_iter().collect();
    Some(Token::Comment(output))
}

fn extend_alphanum(window: &mut VecWindow<char>) -> Option<Token>
{
    let mut char_vec: Vec<char> = Vec::new();

    while window.remaining_length() > 0
    {
        match window.get_value(0)
        {
            Some(&c @ '_') |
            Some(&c @ 'a'..='z') |
            Some(&c @ 'A'..='Z') |
            Some(&c @ '0'..='9') => char_vec.push(c),

            _ => break
        };

        window.move_view(1);
    }

    let output = char_vec.into_iter().collect::<String>().to_ascii_lowercase();

    let token = match output.as_str()
    {
        "goto" => Token::Goto,
        
        "if" => Token::If,
        "then" => Token::Then,
        "else" => Token::Else,
        "end" => Token::End,

        "abs" => Token::Abs,
        "sqrt" => Token::Sqrt,
        "sin" => Token::Sin,
        "cos" => Token::Cos,
        "tan" => Token::Tan,
        "arcsin" => Token::Arcsin,
        "arccos" => Token::Arccos,
        "arctan" => Token::Arctan,
        "not" => Token::Not,

        "or" => Token::Or,
        "and" => Token::And,
        
        s => Token::Identifier(String::from(s))
    };

    Some(token)
}

fn extend_datafield(window: &mut VecWindow<char>) -> Option<Token>
{
    let mut char_vec: Vec<char> = Vec::new();

    char_vec.push(':');
    window.move_view(1);

    while window.remaining_length() > 0
    {
        match window.get_value(0)
        {
            Some(&c @ '_') |
            Some(&c @ 'a'..='z') |
            Some(&c @ 'A'..='Z') |
            Some(&c @ '0'..='9') => char_vec.push(c),

            _ => break
        };

        window.move_view(1);
    }

    let output = char_vec.into_iter().collect::<String>().to_ascii_lowercase();
    Some(Token::Identifier(output))
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

            // The ascii values between space and tilde are all the regular symbolic text characters
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
    let mut digits: Vec<char> = Vec::new();

    while window.remaining_length() > 0
    {
        match window.get_value(0)
        {
            Some(&decimal @ '.') => digits.push(decimal),
            Some(&num @ '0'..='9') => digits.push(num),
            _ => break
        };

        window.move_view(1);
    }

    let string: String = digits.into_iter().collect();
    let yolol_num = string.parse::<YololNumber>().unwrap();

    Some(Token::YololNum(yolol_num))
}



