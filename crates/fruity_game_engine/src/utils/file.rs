use crate::{FruityError, FruityResult};

/// Asynchronously reads a file from the given path and returns its contents as a String
#[cfg(not(target_arch = "wasm32"))]
pub async fn read_file_to_string_async(file_path: &str) -> FruityResult<String> {
    use std::fs::File;
    use std::io::Read;

    // Attempt to open the file
    let mut file =
        File::open(&file_path).map_err(|error| FruityError::GenericFailure(error.to_string()))?;

    // Create a String instance to hold the file's content.
    let mut contents = String::new();

    // Read the entire contents of a file into a String instance
    file.read_to_string(&mut contents)
        .map_err(|error| FruityError::GenericFailure(error.to_string()))?;

    // Return the content of the file
    Ok(contents)
}

/// Asynchronously reads a file from the given path and returns its contents as a String
#[cfg(target_arch = "wasm32")]
pub async fn read_file_to_string_async(file_path: &str) -> FruityResult<String> {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::Response;

    // Uses `web_sys` to fetch a file's contents from a URL
    // This will only work if the code is compiled with WebAssembly (Wasm)
    // Specifically, this checks if the code is being run in a browser environment that supports Wasm
    let response: Response = JsFuture::from(
        web_sys::window()
            .ok_or(FruityError::GenericFailure(
                "couldn't find global var windows".to_string(),
            ))?
            .fetch_with_str(&file_path),
    )
    .await
    .map_err(|error| FruityError::from(error))?
    .dyn_into()
    .map_err(|error| FruityError::from(error))?;

    // Returns the response contents as a string
    let text = JsFuture::from(response.text().map_err(|error| FruityError::from(error))?)
        .await?
        .as_string()
        .ok_or(FruityError::GenericFailure(
            "Couldn't get a string from js".to_string(),
        ))?;

    Ok(text)
}

/// Asynchronously reads a file from the given path and returns its contents as a vec of bytes
#[cfg(not(target_arch = "wasm32"))]
pub async fn read_file_to_bytes_async(file_path: &str) -> FruityResult<Vec<u8>> {
    use std::fs::File;
    use std::io::Read;

    // Attempt to open the file
    let mut file =
        File::open(&file_path).map_err(|error| FruityError::GenericFailure(error.to_string()))?;

    // Create a Vec instance to hold the file's content.
    let mut contents = Vec::new();

    // Read the entire contents of a file into a Vec instance
    file.read_to_end(&mut contents)
        .map_err(|error| FruityError::GenericFailure(error.to_string()))?;

    // Return the content of the file
    Ok(contents)
}

/// Asynchronously reads a file from the given path and returns its contents as a vec of bytes
#[cfg(target_arch = "wasm32")]
pub async fn read_file_to_bytes_async(file_path: &str) -> FruityResult<Vec<u8>> {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::Response;

    // Uses `web_sys` to fetch a file's contents from a URL
    // This will only work if the code is compiled with WebAssembly (Wasm)
    // Specifically, this checks if the code is being run in a browser environment that supports Wasm
    let response = JsFuture::from(
        web_sys::window()
            .ok_or(FruityError::GenericFailure(
                "couldn't find global var windows".to_string(),
            ))?
            .fetch_with_str(&file_path),
    )
    .await?
    .dyn_into::<Response>()
    .map_err(|error| FruityError::from(error))?;

    // Returns the response contents as a vec of bytes
    let buffer = JsFuture::from(
        response
            .array_buffer()
            .map_err(|error| FruityError::from(error))?,
    )
    .await?
    .dyn_into::<js_sys::Uint8Array>()
    .map_err(|error| FruityError::from(error))?;

    Ok(buffer.to_vec())
}
