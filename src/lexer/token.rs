use std::{collections::HashMap};

use crate::{ParseConfig, Span};

pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    Int(i64),
    Float(f64),
    Str(String),
    Char(char),
    Identifier(String),
    Keyword(Keyword),
    Assign,
    Operator(Operator),
    LParen, RParen,
    LBracket, RBracket,
    LBrace, RBrace,
    Comma, Semicolon, Colon,
    EOL, EOF,
}

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Add, Sub, Mul, Div, Mod, FullDiv,
    And, Or, Not,
    Eq, Neq, Lt, Lte, Gt, Gte,
    Concat,
}

#[derive(Debug, Clone, Copy)]
pub enum Keyword {
    If, Then, Else, EndIf,
    While, Repeat, Until, EndWhile,
    For, To, Next, Step,
    Func, Return, Procedure, EndFunc, EndProcedure,
    Continue, Break,
    Declare, Constant,
    Input, Output,
    True, False, Null,
    ByRef, ByVal,
}

pub fn get_keywords_map(config: &ParseConfig) -> HashMap<&'static str, Keyword> {
    let mut keywords = HashMap::from([
        ("IF", Keyword::If),
        ("THEN", Keyword::Then),
        ("ELSE", Keyword::Else),
        ("ENDIF", Keyword::EndIf),
        ("WHILE", Keyword::While),
        ("REPEAT", Keyword::Repeat),
        ("UNTIL", Keyword::Until),
        ("ENDWHILE", Keyword::EndWhile),
        ("FOR", Keyword::For),
        ("TO", Keyword::To),
        ("NEXT", Keyword::Next),
        ("STEP", Keyword::Step),
        ("FUNCTION", Keyword::Func),
        ("PROCEDURE", Keyword::Procedure),
        ("ENDFUNCTION", Keyword::EndFunc),
        ("ENDPROCEDURE", Keyword::EndProcedure),
        ("DECLARE", Keyword::Declare),
        ("CONSTANT", Keyword::Constant),
        ("INPUT", Keyword::Input),
        ("OUTPUT", Keyword::Output),
        ("TRUE", Keyword::True),
        ("FALSE", Keyword::False),
        ("NULL", Keyword::Null),
    ]);
    if config.allow_func_return {
        keywords.insert("RETURN", Keyword::Return);
    }
    if config.allow_loop_control {
        keywords.insert("CONTINUE", Keyword::Continue);
        keywords.insert("BREAK", Keyword::Break);
    }
    return keywords;
}

pub fn get_operators_map(config: &ParseConfig) -> HashMap<&'static str, TokenKind> {
    let mut operators = HashMap::from([
        ("←", TokenKind::Assign),
        ("<-", TokenKind::Assign),
        ("+", TokenKind::Operator(Operator::Add)),
        ("-", TokenKind::Operator(Operator::Sub)),
        ("*", TokenKind::Operator(Operator::Mul)),
        ("/", TokenKind::Operator(Operator::Div)),
        ("MOD", TokenKind::Operator(Operator::Mod)),
        ("AND", TokenKind::Operator(Operator::And)),
        ("OR", TokenKind::Operator(Operator::Or)),
        ("NOT", TokenKind::Operator(Operator::Not)),
        ("DIV", TokenKind::Operator(Operator::FullDiv)),
        ("<>", TokenKind::Operator(Operator::Neq)),
        ("<", TokenKind::Operator(Operator::Lt)),
        ("<=", TokenKind::Operator(Operator::Lte)),
        (">", TokenKind::Operator(Operator::Gt)),
        (">=", TokenKind::Operator(Operator::Gte)),
        ("&", TokenKind::Operator(Operator::Concat)),
        ("(", TokenKind::LParen),
        (")", TokenKind::RParen),
        ("[", TokenKind::LBracket),
        ("]", TokenKind::RBracket),
        ("{", TokenKind::LBrace),
        ("}", TokenKind::RBrace),
        (",", TokenKind::Comma),
        (";", TokenKind::Semicolon),
        (":", TokenKind::Colon),
    ]);
    if config.allow_equal_assign {
        operators.insert("=", TokenKind::Assign);
        operators.insert("==", TokenKind::Operator(Operator::Eq));
    } else {
        operators.insert("=", TokenKind::Operator(Operator::Eq));
    }
    return operators;
}
