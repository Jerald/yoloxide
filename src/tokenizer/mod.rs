#[derive(Debug, PartialEq)]
pub enum Token
{
    Command(String),
    AlphaNumToken(String),
    NumToken(u64),
    Quote,
    Equal,
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Period,
    LAngleBrak,
    RAngleBrak,
    Exclam,
    Caret,
    Percent,
    Colon,
    Newline
}

pub fn tokenize(input: String) -> Vec<Token>
{
    let mut output_vec: Vec<Token> = Vec::new();
    let input_chars: Vec<char> = input.chars().collect();

    let mut i = 0;
    while i < input_chars.len()
    {
        // println!("i: {}", i);
        let item = input_chars[i];

        let (new_value, index) = match item
        {
            'a'..='z' | 'A'..='Z'| '0'..='9' => extend_alpha_or_num_token(&input_chars, i),
            '"'  => (Token::Quote, i+1),
            '='  => (Token::Equal, i+1),
            '+'  => (Token::Plus, i+1),
            '-'  => (Token::Minus, i+1),
            '*'  => (Token::Star, i+1),
            '('  => (Token::LParen, i+1),
            ')'  => (Token::RParen, i+1),
            '.'  => (Token::Period, i+1),
            '<'  => (Token::LAngleBrak, i+1),
            '>'  => (Token::RAngleBrak, i+1),
            '!'  => (Token::Exclam, i+1),
            '%'  => (Token::Percent, i+1),
            ':'  => (Token::Colon, i+1),
            '\n' => (Token::Newline, i+1),
            _    => { i+=1; continue }
        };

        i = index;
        output_vec.push(new_value);
    }

    output_vec
}

fn extend_alpha_or_num_token(input: &Vec<char>, index: usize) -> (Token, usize)
{
    let mut output: Vec<char> = Vec::new();
    let mut is_alpha_num_token = if let '0'..='9' = input[index] { false } else { true };
    let mut is_zeros = if let '0' = input[index] { true } else { false };

    let mut out_index = input.len();
    for i in index..input.len()
    {
        let item = input[i];
        // println!("Current char: '{}', i: '{}'", item, i);

        match item
        {
            'a'..='z' | 'A'..='Z' => {
                is_alpha_num_token = true;
                output.push(item);
            },
            '1'..='9' => {
                if is_zeros { out_index = i; break; }
                output.push(item);
            },
            '0' => output.push(item),
            _ => { out_index = i; break }
        }
    }

    if is_alpha_num_token
    {
        (Token::AlphaNumToken(output.into_iter().collect()), out_index)
    }
    else
    {
        let output_string: String = output.into_iter().collect();
        let number = output_string.parse::<u64>().expect("Tokenizing error from converting what we think is a num token into a u64!");
        (Token::NumToken(number), out_index)
    } 
}