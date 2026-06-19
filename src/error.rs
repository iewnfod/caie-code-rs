use crate::{Op, RuntimeValue, Span, Type};

#[derive(Debug, Clone)]
pub enum CpcError {
    Lex { span: Span, message: String },
    Parse { span: Span, message: String, expected: Option<String> },
    Runtime { span: Option<Span>, kind: RuntimeErrorKind },
}

#[derive(Debug, Clone)]
pub enum RuntimeErrorKind {
    UndefinedVariable(String),
    UndefinedFunction(String),
    AssignToUndefined(String),
    TypeMismatch { expect: Type, found: Type },
    InvalidOp { op: Op, left: Type, right: Type },
    InvalidUnaryOp { op: Op, operand: Type },
    DivideByZero,
    IndexOutOfBounds { index: i64, range: (usize, usize) },
    NotIndexable(Type),
    NotCallable(Type),
    ArgCountMismatch { expect: usize, found: usize },
    IndexMustBeInt(Type),
    ReturnNotFound(String),
    InvalidIndex(RuntimeValue),
    TypeWithoutDefaultValue(Type),
    UnknownType(RuntimeValue),
    Other(String),
}

pub type CpcResult<T> = Result<T, CpcError>;

impl CpcError {
    pub fn report(&self) -> String {
        match self {
            CpcError::Lex { span, message } => {
                format!("Lexical Error at line {}, column {}: {}", span.line, span.column, message)
            }
            CpcError::Parse { span, message, expected } => {
                let mut output = format!("Parse Error at line {}, column {}: {}", span.line, span.column, message);
                if let Some(expected) = expected {
                    output.push_str(&format!("\nExpected: {}", expected));
                }
                output
            }
            CpcError::Runtime { span, kind } => {
                let mut output = format!("Runtime Error: {:?}", kind);
                if let Some(span) = span {
                    output.push_str(&format!("\nAt line {}, column {}: {:?}", span.line, span.column, kind));
                }
                output
            }
        }
    }
}
