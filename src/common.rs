use failure::Fail;
use std::collections::HashMap;

use crate::peg;

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum Expression {
    // Single items
    String(String),
    Floating(f64),
    Variable(String),
    Object(String), // currently just functions
    True,
    False,
    Call(String, Vec<Expression>),
    Nothing,
    Null,
    Mysterious,
    Pronoun,
    Not(Box<Expression>),

    // needed by loops
    Break,
    Continue,

    // binary operators
    Is(Box<Expression>, Box<Expression>),
    Aint(Box<Expression>, Box<Expression>),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Times(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Nor(Box<Expression>, Box<Expression>),
    GreaterThanOrEqual(Box<Expression>, Box<Expression>),
    GreaterThan(Box<Expression>, Box<Expression>),
    LessThanOrEqual(Box<Expression>, Box<Expression>),
    LessThan(Box<Expression>, Box<Expression>),
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum SymbolType {
    Is,
    Up,
    Down,
    Until,
    While,
    Return,
    GreaterThanOrEqual,
    GreaterThan,
    LessThan,
    LessThanOrEqual,
    Add,
    Subtract,
    Times,
    Aint,
    Divide,
    Empty,
    VariableList(Vec<String>),
    ArgsList(Vec<Expression>),
    ExpressionList(Vec<Expression>),
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub line: usize,
    pub symbol: SymbolType,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub commands: Vec<CommandLine>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Command {
    Assignment {
        target: Expression,
        value: Expression,
    },
    Until {
        expression: Expression,
        block: Block,
    },
    While {
        expression: Expression,
        block: Block,
    },
    If {
        expression: Expression,
        then: Option<Block>,
        otherwise: Option<Block>,
    },
    Increment {
        target: String,
        count: f64,
    },
    Decrement {
        target: String,
        count: f64,
    },
    Continue,
    Break,
    Say {
        value: Expression,
    },
    Listen {
        target: Option<String>,
    },
    FunctionDeclaration {
        name: String,
        args: Vec<String>,
        block: Block,
    },
    Return {
        return_value: Expression,
    },
    Call {
        name: String,
        args: Vec<Expression>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub args: Vec<String>,
    pub block: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CommandLine {
    pub cmd: Command,
    pub line: usize,
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub commands: Vec<CommandLine>,
    pub functions: HashMap<String, Function>,
}

#[derive(Debug, Fail)]
#[allow(dead_code)]
pub enum MaidenError {
    #[fail(display = "parsing error: {:?}", kind)]
    Pest { kind: pest::error::Error<peg::Rule> },
    #[fail(display = "IO Error")]
    Io {
        #[fail(cause)]
        io_error: std::io::Error,
    },
    #[fail(display = "Unparsed text '{}'", text)]
    UnparsedText { text: String, line: usize },
    #[fail(display = "Missing variable '{}'", name)]
    MissingVariable { name: String, line: usize },
    #[fail(display = "Missing function '{}'", name)]
    MissingFunction { name: String, line: usize },
    #[fail(
        display = "Wrong argument count to function (expected {}, got {})",
        expected, got
    )]
    WrongArgCount {
        expected: usize,
        got: usize,
        line: usize,
    },
    #[fail(display = "Unbalanced expression {}", expression)]
    UnbalancedExpression { expression: String, line: usize },
    #[fail(display = "Bad boolean resolve: {:?}", expression)]
    BadBooleanResolve { expression: String, line: usize },
    #[fail(display = "Don't recognise command sequence {:?}", sequence)]
    BadCommandSequence {
        sequence: Vec<SymbolType>,
        line: usize,
    },
    #[fail(display = "Unparsable number: '{}'", number)]
    ParseNumberError { number: String, line: usize },
    #[fail(display = "Bad 'is' section: {:?}", sequence)]
    BadIs {
        sequence: Vec<SymbolType>,
        line: usize,
    },
    #[fail(display = "Bad 'put' section: {:?}", sequence)]
    BadPut {
        sequence: Vec<SymbolType>,
        line: usize,
    },
    #[fail(display = "No end of if statement")]
    NoEndOfIf { line: usize },
    #[fail(display = "Else with no if statement")]
    ElseWithNoIf { line: usize },
    #[fail(display = "More than one else statement")]
    MultipleElse { line: usize },
    #[fail(display = "No end of function")]
    NoEndFunction { line: usize },
    #[fail(display = "No end of loop")]
    NoEndLoop { line: usize },
    #[fail(display = "Continue outside of a loop")]
    ContinueOutsideLoop { line: usize },
    #[fail(display = "Break outside of a loop")]
    BreakOutsideLoop { line: usize },
    #[fail(display = "Next outside of a loop")]
    NextOutsideLoop { line: usize },
    #[fail(display = "Unimplemented: {}", description)]
    Unimplemented { description: String, line: usize },
    #[fail(display = "Exceeded maximum allowed stack depth of {}", depth)]
    StackOverflow { depth: u32, line: usize },
    #[fail(display = "Hit instruction limit of 10,000,000. Infinite loop?")]
    InstructionLimit { line: usize },
    #[fail(display = "Got to a pronoun, but no variable defined")]
    UndefinedPronoun { line: usize },
    #[fail(display = "Got infinity on divide between {} and {}", x, y)]
    Infinity { x: String, y: String, line: usize },

    #[fail(display = "Expected another item, but didn't get one")]
    Incomplete { line: usize },

    #[fail(display = "Bad string. Expected length at least 2 and got {}", length)]
    BadString { length: usize, line: usize },
}

pub type Result<T> = ::core::result::Result<T, MaidenError>;

impl From<std::io::Error> for MaidenError {
    fn from(err: std::io::Error) -> MaidenError {
        return MaidenError::Io { io_error: err };
    }
}
