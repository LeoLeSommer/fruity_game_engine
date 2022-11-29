#![allow(missing_docs)]

/// A generic result that is able to be exposed to the js
pub type FruityResult<T> = Result<T, FruityError>;

/// A generic error that is able to be exposed to the js
#[derive(Debug, Clone)]
pub enum FruityError {
    Ok(String),
    InvalidArg(String),
    ObjectExpected(String),
    StringExpected(String),
    NameExpected(String),
    FunctionExpected(String),
    NumberExpected(String),
    BooleanExpected(String),
    ArrayExpected(String),
    GenericFailure(String),
    PendingException(String),
    Cancelled(String),
    EscapeCalledTwice(String),
    HandleScopeMismatch(String),
    CallbackScopeMismatch(String),
    QueueFull(String),
    Closing(String),
    BigintExpected(String),
    DateExpected(String),
    ArrayBufferExpected(String),
    DetachableArraybufferExpected(String),
    WouldDeadlock(String),
    Unknown(String),
}
