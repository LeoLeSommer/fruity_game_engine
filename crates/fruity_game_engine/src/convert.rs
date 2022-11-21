use crate::FruityResult;
pub use fruity_game_engine_macro::FruityFrom;

/// A value-to-value conversion that consumes the input value.
pub trait FruityInto<T>: Sized {
  /// Performs the conversion.
  fn fruity_into(self) -> FruityResult<T>;
}

/// Simple and safe type conversions that may fail in a controlled
/// way under some circumstances.
pub trait FruityFrom<T>: Sized {
  /// Performs the conversion.
  fn fruity_try_from(value: T) -> FruityResult<Self>;
}
