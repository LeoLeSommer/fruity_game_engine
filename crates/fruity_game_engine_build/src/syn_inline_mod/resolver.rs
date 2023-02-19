use crate::syn_inline_mod::Error;
use std::path::Path;

/// A resolver that can turn paths into `syn::File` instances.
pub(crate) trait FileResolver {
    /// Check if `path` exists in the backing data store.
    fn path_exists(&self, path: &Path) -> bool;

    /// Resolves the given path into a file.
    ///
    /// Returns an error if the file couldn't be loaded or parsed as valid Rust.
    fn resolve(&mut self, path: &Path) -> Result<syn::File, Error>;
}

#[derive(Clone)]
pub(crate) struct FsResolver<F> {
    on_load: F,
}

impl<F> FsResolver<F> {
    pub(crate) fn new(on_load: F) -> Self {
        Self { on_load }
    }
}

impl<F> FileResolver for FsResolver<F>
where
    F: FnMut(&Path, String),
{
    fn path_exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn resolve(&mut self, path: &Path) -> Result<syn::File, Error> {
        let src = std::fs::read_to_string(path)?;
        let res = syn::parse_file(&src);
        // Call the callback whether the file parsed successfully or not.
        (self.on_load)(path, src);
        Ok(res?)
    }
}
