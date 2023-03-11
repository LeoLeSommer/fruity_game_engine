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
    NoExternalBuffersAllowed(String),
    Unknown(String),
}

impl ToString for FruityError {
    fn to_string(&self) -> String {
        match self {
            FruityError::Ok(message) => message.to_string(),
            FruityError::InvalidArg(message) => message.to_string(),
            FruityError::ObjectExpected(message) => message.to_string(),
            FruityError::StringExpected(message) => message.to_string(),
            FruityError::NameExpected(message) => message.to_string(),
            FruityError::FunctionExpected(message) => message.to_string(),
            FruityError::NumberExpected(message) => message.to_string(),
            FruityError::BooleanExpected(message) => message.to_string(),
            FruityError::ArrayExpected(message) => message.to_string(),
            FruityError::GenericFailure(message) => message.to_string(),
            FruityError::PendingException(message) => message.to_string(),
            FruityError::Cancelled(message) => message.to_string(),
            FruityError::EscapeCalledTwice(message) => message.to_string(),
            FruityError::HandleScopeMismatch(message) => message.to_string(),
            FruityError::CallbackScopeMismatch(message) => message.to_string(),
            FruityError::QueueFull(message) => message.to_string(),
            FruityError::Closing(message) => message.to_string(),
            FruityError::BigintExpected(message) => message.to_string(),
            FruityError::DateExpected(message) => message.to_string(),
            FruityError::ArrayBufferExpected(message) => message.to_string(),
            FruityError::DetachableArraybufferExpected(message) => message.to_string(),
            FruityError::WouldDeadlock(message) => message.to_string(),
            FruityError::NoExternalBuffersAllowed(message) => message.to_string(),
            FruityError::Unknown(message) => message.to_string(),
        }
    }
}
