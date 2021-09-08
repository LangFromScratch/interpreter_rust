//! 抽象構文木(AST)を行うmodです。

use std::fmt;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>
}

impl fmt::Display for Program {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for statement in self.statements.iter() {
            write!(formatter, "{}\n", statement)?;
        }
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd)]
pub enum Statement {
    // let
    LetStatement{
        identifier: Expression,
        value: Expression
    },
    // expression
    ExpressionStatement(Expression),
    // return
    Return(Expression),
    // code block
    Block(Vec<Statement>),
}

impl fmt::Display for Statement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // let
            Statement::LetStatement{
                identifier,
                value
            } => write!(formatter, "let {} = {};", identifier, value),

            // expression
            Statement::ExpressionStatement(Expression) => write!(formatter, "{}", Expression),

            // return
            Statement::Return(Expression) => write!(formatter, "return {}", Expression),

            // Block
            Statement::Block(Statement) => {
                for statement in Statement.iter() {
                    write!(formatter, "{}", statement)?;
                }
                Ok(())
            },

            // None
            _ => write!(formatter, "none")
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd)]
pub enum Expression {
    Identifier(String),
    String(String),
    Integer(i32),
    Array(Vec<Expression>),
    Boolean(bool),
    HashMap(BTreeMap<Box<Expression>, Box<Expression>>),
    LParen(String),

    // Index
    IndexExpression{
        array: Box<Expression>,
        subscript: Box<Expression>
    },

    // Prefix
    PrefixExpression{
        operator: String,
        rightExpression: Box<Expression>
    },

    // Infix
    InfixExpression{
        leftExpression: Box<Expression>,
        operator: String,
        rightExpression: Box<Expression>
    },

    // if
    IfExpression{
        condition: Box<Expression>,
        consequence: Box<Statement>,
        alternative: Option<Box<Statement>>
    },

    // Function
    FunctionLiteral{
        parameters: Vec<Expression>,
        body: Box<Statement>
    },

    // Call
    CallExpression{
        function: Box<Expression>,
        body: Vec<Expression>
    },
    Null
}

impl fmt::Display for Expression {
    fn fmt(&self, fomatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Identifier(value) => write!(fomatter, "{}", &value),
            // String
            Expression::String(value) => write!(fomatter, "{}", &value),
            // Integer
            Expression::Integer(value) => write!(fomatter, "{}", value),
            // Array
            Expression::Array(value) => write!(fomatter, "[{}]", value.iter().map(|expression| format!("{}", &expression)).collect::<Vec<_>>().join(", ")),
            // Boolean
            Expression::Boolean(value) => write!(fomatter, "{}", value),
            // HashMap
            Expression::HashMap(tree) => {
                match tree {
                    tree => write!(fomatter, "{{{}}}", tree.iter().map(|(key, value)| format!("{}: {}", key, value)).collect::<Vec<_>>().join(", ")),
                    _ => unreachable!()
                }
            },
            // LParen
            Expression::LParen(value) => write!(fomatter, "{}", value),
            // Index
            Expression::IndexExpression{
                array,
                subscript
            } => write!(fomatter, "{}[{}]", array, subscript),
            // Prefix
            Expression::PrefixExpression{
                operator,
                rightExpression
            } => write!(fomatter, "{}{}", operator, rightExpression),
            // Infix
            Expression::InfixExpression{
                leftExpression,
                operator,
                rightExpression
            } => write!(fomatter, "{} {} {}", leftExpression, operator, rightExpression),
            // if
            Expression::IfExpression{
                condition,
                consequence,
                alternative
            } => {
                match alternative {
                    Some(alternative) => write!(fomatter, "if ({}) {{{}}} else {{{}}}", condition, consequence, alternative),
                    None => write!(fomatter, "if ({}) {{{}}}", condition, consequence),
                }
            }
            // Function
            Expression::FunctionLiteral{
                parameters,
                body
            } => write!(
                fomatter, 
                "function ({}) {{{}}}", 
                parameters.iter().map(|expression| -> &str {
                    match expression {
                        Expression::Identifier(identifier) => identifier,
                        _ => unreachable!(),
                    }
                }).collect::<Vec<_>>().join(", "),
                body
            ),
            // Call
            Expression::CallExpression{
                function,
                body
            } => write!(fomatter, "{}({});", function, body.iter().map(|expression| format!("{}", &expression)).collect::<Vec<_>>().join(", ")),
            // Null
            Null => write!(fomatter, "null")
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Precedence {
    EQUALS,         // ==
    LESSGREATER,    // > or <
    SUM,            // + or -
    PRODUCT,        // * or /
    PREFIX,         // !x or -x
    LBRANCKET,      // []
    CALL,           // function(x){}
    LOWEST
}