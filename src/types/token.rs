use codespan::Span as Codespan;
use codespan::{
    FileId,
    ByteIndex
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span
{
    span: Codespan,
    source: FileId
}

impl Span
{
    pub fn new(source: FileId, span: impl Into<Codespan>) -> Self
    {
        Self {
            span: span.into(),
            source
        }
    }
}

impl From<Span> for Codespan
{
    fn from(span: Span) -> Self
    {
        span.span
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token
{
    kind: TokenKind,
    span: Span
}

impl Token
{
    fn new(kind: TokenKind, span: Span) -> Self
    {
        Token {
            kind,
            span
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind
{
    Comment,
    Identifier,
    String,
    Number,

    Newline,
    Space,

    Equal,
    EqualEqual,

    Plus,
    PlusPlus,
    PlusEqual,

    Minus,
    MinusMinus,
    MinusEqual,

    Star,
    StarEqual,
    
    Slash,
    SlashEqual,
    
    Lesser,
    LesserEqual,
    
    Greater,
    GreaterEqual,
    
    Exclam,
    ExclamEqual,
    
    Percent,
    PercentEqual,
    
    LParen,
    RParen,
    Caret,
}

impl TokenKind
{
    pub fn to_token(self, span: Span) -> Token
    {
        Token::new(self, span)
    }

    pub fn spanned(self, span: Span) -> Token
    {
        Self::to_token(self, span)
    }
}

impl TokenKind
{
    fn break_into_two(self) -> (TokenKind, TokenKind)
    {
        use TokenKind::*;
        match self
        {
            EqualEqual => (Equal, Equal),

            PlusPlus => (Plus, Plus),
            PlusEqual => (Plus, Equal),

            MinusMinus => (Minus, Minus),
            MinusEqual => (Minus, Equal),

            StarEqual => (Star, Equal),
            SlashEqual => (Slash, Equal),
            LesserEqual => (Lesser, Equal),
            GreaterEqual => (Greater, Equal),
            ExclamEqual => (Exclam, Equal),
            PercentEqual => (Percent, Equal)
        }
    }
}