use std::str::Chars;
use std::iter::Peekable;
use std::convert::TryInto;


use codespan::{
    FileId,
    Files,
    ByteIndex
};

use crate::types::{Token, TokenKind, Span};

trait CharExt
{
    fn is_ident(&self) -> bool;
    
    fn is_ident_start(&self) -> bool;
}

impl CharExt for char
{
    fn is_ident(&self) -> bool
    {
        self.is_ascii_alphabetic() || self == &'_'
    }
    
    fn is_ident_start(&self) -> bool
    {
        self.is_ident() || self == &':'
    }
}

// TODO: Yolol only accepts ascii input specifically, so we can optimize by just using u8's as ascii chars.

// TODO: Additional checks during lexing to ensure a larger token doesn't contain a newline is needed.

pub struct Tokenizer<'a>
{
    file: FileId,
    chars: Peekable<Chars<'a>>,
    initial_len: usize
}

impl<'a> Tokenizer<'a>
{
    pub fn new(files: &'a Files<String>, file: FileId) -> Tokenizer<'a>
    {
        Tokenizer {
            file,
            chars: files.source(file).chars().peekable(),
            initial_len: files.source(file).len()
        }
    }

    /// Returns the current byte index through the source.
    fn index(&self) -> ByteIndex
    {
        // This would normally be a problem, since Iterator::size_hint() is an approximation at best.
        // But although Chars doesn't implement ExactSizedIterator, it can actually know its exact size, in bytes at least.
        // The implementation specifically returns the upper bound as the length of the underlying &[u8] slice.
        let (_, Some(len)) = self.chars.size_hint();
        let idx: u32 = (self.initial_len - len).try_into().unwrap();

        ByteIndex::from(idx)
    }

    /// Consumes characters as long as the check is passed.
    fn eat_while<F>(&mut self, check: F)
    where
        F: Fn(&char) -> bool,
    {
        // We loop by peeking the next char, seeing if the check validates,
        // and eating the peeked char if it does.

        // This map_or handles the case where peek returns None by stopping the loop.
        while self.chars.peek().map_or(false, check)
        {
            self.chars.next();
        }
    }

    fn next_string(&mut self, start_idx: ByteIndex) -> Option<Token>
    {
        // Loop until we find the end of the string
        self.eat_while(|&c| c != '"');

        // Ensure we didn't goof and in fact are eating the last quote
        debug_assert!(self.chars.peek() == Some(&'"'));
        // Advance once to consume the last quote found
        self.chars.next();

        Some(Token {
            kind: TokenKind::String,
            span: self.span_from(start_idx)
        })
    }

    fn next_ident(&mut self, start_idx: ByteIndex) -> Option<Token>
    {
        // Loop as long as it's still a valid identifier
        self.eat_while(char::is_ident);

        Some(Token {
            kind: TokenKind::Identifier,
            span: self.span_from(start_idx)
        })
    }

    fn next_number(&mut self, start_idx: ByteIndex) -> Option<Token>
    {
        // Loop as long as there are still valid digits
        self.eat_while(char::is_ascii_digit);

        if let Some('.') = self.chars.peek()
        {
            // Skip the peeked decimal
            self.chars.next();

            // Loop again as long as there are valid digits after the decimal
            self.eat_while(char::is_ascii_digit);
        }

        Some(Token {
            kind: TokenKind::Number,
            span: self.span_from(start_idx)
        })
    }

    fn next_comment(&mut self, start_idx: ByteIndex) -> Option<Token>
    {
        // Loop as long as we've not yet hit the end of the line
        self.eat_while(|&c| c != '\n');

        Some(Token {
            kind: TokenKind::Comment,
            span: self.span_from(start_idx)
        })
    }

    fn span_from(&self, start_idx: ByteIndex) -> Span
    {
        Span::new(self.file, start_idx..self.index())
    }
}

impl Iterator for Tokenizer<'_>
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item>
    {
        use TokenKind as Kind;

        // Makes it much easier to write nested matches for rules requiring peeks
        macro_rules! match_peek {
            (
                $(Some($ci:literal) => $ki:expr,)+
                _ => $k:expr
                $(,)*
            ) => {
                match self.chars.peek() {
                    $(
                        Some($ci) => {
                            self.chars.next();
                            $ki
                        },
                    )*
                    _ => $k
                }
            };
        }

        let start_idx = self.index();

        // Spans a token kind by the start index into an actual token
        macro_rules! spanned {
            ($k:expr) => {
                $k.spanned(self.span_from(start_idx))
            };
        }

        let token = match self.chars.next()?
        {
            '"' => self.next_string(start_idx)?,

            c if c.is_ascii_digit() => self.next_number(start_idx)?,

            c if c.is_ident_start() => self.next_ident(start_idx)?,

            '\n' => spanned!(Kind::Newline),
            ' ' => spanned!(Kind::Space),

            '=' => match_peek! {
                Some('=') => spanned!(Kind::EqualEqual),
                _ => spanned!(Kind::Equal)
            },

            '+' => match_peek! {
                Some('+') => spanned!(Kind::PlusPlus),
                Some('=') => spanned!(Kind::PlusEqual),
                _ => spanned!(Kind::Plus),
            },

            '-' => match_peek! {
                Some('-') => spanned!(Kind::MinusMinus),
                Some('=') => spanned!(Kind::MinusEqual),
                _ => spanned!(Kind::Minus)
            },

            '*' => match_peek! {
                Some('=') => spanned!(Kind::StarEqual),
                _ => spanned!(Kind::Star)
            },

            '/' => match_peek! {
                Some('/') => {
                    self.next_comment(start_idx)?
                },
                Some('=') => spanned!(Kind::SlashEqual),
                _ => spanned!(Kind::Slash)
            },

            '<' => match_peek! {
                Some('=') => spanned!(Kind::LesserEqual),
                _ => spanned!(Kind::Lesser)
            },

            '>' => match_peek! {
                Some('=') => spanned!(Kind::GreaterEqual),
                _ => spanned!(Kind::Greater)
            },

            '!' => match_peek! {
                Some('=') => spanned!(Kind::ExclamEqual),
                _ => spanned!(Kind::Exclam)
            },

            '%' => match_peek! {
                Some('=') => spanned!(Kind::PercentEqual),
                _ => spanned!(Kind::Percent)
            },

            '(' => spanned!(Kind::LParen),
            ')' => spanned!(Kind::RParen),
            '^' => spanned!(Kind::Caret),

            _ => todo!("Handle unknown token input!")
        };
        
        Some(token)
    }
}