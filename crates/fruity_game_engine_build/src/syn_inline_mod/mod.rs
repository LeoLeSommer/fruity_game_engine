//! Utility to traverse the file-system and inline modules that are declared as references to
//! other Rust files.

use proc_macro2::Span;
use std::{
    error, fmt, io,
    path::{Path, PathBuf},
};
use syn::spanned::Spanned;
use syn::ItemMod;

mod mod_path;
mod resolver;
mod visitor;

pub(crate) use mod_path::*;
pub(crate) use resolver::*;
pub(crate) use visitor::Visitor;

/// Parse the source code in `src_file` and return a `syn::File` that has all modules
/// recursively inlined.
///
/// This is equivalent to using an `InlinerBuilder` with the default settings.
///
/// # Panics
///
/// This function will panic if `src_file` cannot be opened or does not contain valid Rust
/// source code.
///
/// # Error Handling
///
/// This function ignores most error cases to return a best-effort result. To be informed of
/// failures that occur while inlining referenced modules, create an `InlinerBuilder` instead.
pub fn parse_and_inline_modules(src_file: &Path) -> syn::File {
    InlinerBuilder::default()
        .parse_and_inline_modules(src_file)
        .unwrap()
        .output
}

/// A builder that can configure how to inline modules.
///
/// After creating a builder, set configuration options using the methods
/// taking `&mut self`, then parse and inline one or more files using
/// `parse_and_inline_modules`.
#[derive(Debug)]
pub struct InlinerBuilder {
    root: bool,
}

impl Default for InlinerBuilder {
    fn default() -> Self {
        InlinerBuilder { root: true }
    }
}

impl InlinerBuilder {
    /// Parse the source code in `src_file` and return an `InliningResult` that has all modules
    /// recursively inlined.
    pub fn parse_and_inline_modules(&self, src_file: &Path) -> Result<InliningResult, Error> {
        self.parse_internal(src_file, &mut FsResolver::new(|_: &Path, _| {}))
    }

    fn parse_internal<R: FileResolver>(
        &self,
        src_file: &Path,
        resolver: &mut R,
    ) -> Result<InliningResult, Error> {
        // XXX There is no way for library callers to disable error tracking,
        // but until we're sure that there's no performance impact of enabling it
        // we'll let downstream code think that error tracking is optional.
        let mut errors = Some(vec![]);
        let result = Visitor::<R>::new(src_file, self.root, errors.as_mut(), resolver).visit()?;
        Ok(InliningResult::new(result, errors.unwrap_or_default()))
    }
}

/// An error that was encountered while reading, parsing or inlining a module.
///
/// Errors block further progress on inlining, but do not invalidate other progress.
/// Therefore, only an error on the initially-passed-in-file is fatal to inlining.
#[derive(Debug)]
pub enum Error {
    /// An error happened while opening or reading the file.
    Io(io::Error),

    /// Errors happened while using `syn` to parse the file.
    Parse(syn::Error),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Io(err) => Some(err),
            Error::Parse(err) => Some(err),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<syn::Error> for Error {
    fn from(err: syn::Error) -> Self {
        Error::Parse(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(_) => write!(f, "IO error"),
            Error::Parse(_) => write!(f, "parse error"),
        }
    }
}

/// The result of a best-effort attempt at inlining.
///
/// This struct guarantees that the origin file was readable and valid Rust source code, but
/// `errors` must be inspected to check if everything was inlined successfully.
pub struct InliningResult {
    output: syn::File,
    errors: Vec<InlineError>,
}

impl InliningResult {
    /// Create a new `InliningResult` with the best-effort output and any errors encountered
    /// during the inlining process.
    pub(crate) fn new(output: syn::File, errors: Vec<InlineError>) -> Self {
        InliningResult { output, errors }
    }
}

impl fmt::Debug for InliningResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.errors.fmt(f)
    }
}

impl fmt::Display for InliningResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Inlining partially completed before errors:")?;
        for error in &self.errors {
            writeln!(f, " * {}", error)?;
        }

        Ok(())
    }
}

/// An error that happened while attempting to inline a module.
#[derive(Debug)]
pub struct InlineError {
    src_path: PathBuf,
    src_span: Span,
    path: PathBuf,
    kind: Error,
}

impl InlineError {
    pub(crate) fn new(
        src_path: impl Into<PathBuf>,
        item_mod: &ItemMod,
        path: impl Into<PathBuf>,
        kind: Error,
    ) -> Self {
        Self {
            src_path: src_path.into(),
            src_span: item_mod.span(),
            path: path.into(),
            kind,
        }
    }
}

impl fmt::Display for InlineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let start = self.src_span.start();
        write!(
            f,
            "{}:{}:{}: error while including {}: {}",
            self.src_path.display(),
            start.line,
            start.column,
            self.path.display(),
            self.kind
        )
    }
}
