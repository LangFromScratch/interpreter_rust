//! Tokenを定義するmodです。
//! astに依存しています

use super::ast::{Precedence};

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, Ord, PartialOrd)]
pub enum Token {
    EOF,        // 終端
    ILLEGAL,    // 不正なデータ

    // リテラル
    INT,        // integer
    STRING,     // 文字列
    IDENT,      // インデント

    // オペレーター
    ASSIGN,     // =

    EQUAL,      // ==
    NOT_EQ,     // !=
    ADD,        // +
    SUB,        // -
    MUL,        // *
    DIV,        // /
    LT,         // <
    GT,         // >
    NOT,        // !

    // 区切り系
    COMMA,      // ,
    SEMI_COLON, // ;
    COLON,      // .

    // 括弧
    L_PAREN,    // (
    R_PAREN,    // )
    L_BRACE,    // {
    R_BRACE,    // }
    L_BRACKET,  // [
    R_BRACKET,  // ]

    // キーワード
    FUNCTION,   // FUNCTION
    LET,        // LET
    TRUE,       // TRUE
    FALSE,      // FALSE
    IF,         // IF
    ELSE,       // ELSE
    RETURN,     // RETURN

    LOWEST,
    DEFAULT,
}

pub fn get_keyword(ident: &str) -> Token {
    match ident {
        "function" => {
            Token::FUNCTION
        }
        "let" => {
            Token::LET
        }
        "true" => {
            Token::TRUE
        }
        "false" => {
            Token::FALSE
        }
        "if" => {
            Token::IF
        }
        "else" => {
            Token::ELSE
        }
        "return" => {
            Token::RETURN
        }
        _ => {
            Token::IDENT
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd)]
pub struct Tokens {
    pub token_type: Token,
    pub literal: String
}

impl Tokens {
    pub fn get_precedence(&mut self) -> Precedence {
        match self.token_type {
            Token::EQUAL => Precedence::EQUALS,
            Token::NOT_EQ => Precedence::EQUALS,
            Token::ADD => Precedence::SUM,
            Token::SUB => Precedence::SUM,
            Token::MUL => Precedence::PRODUCT,
            Token::DIV => Precedence::PRODUCT,
            Token::LT => Precedence::LESSGREATER,
            Token::GT => Precedence::LESSGREATER,
            Token::L_PAREN => Precedence::CALL,
            Token::L_BRACKET => Precedence::LBRANCKET,
            _ => Precedence::LOWEST
        }
    }
}