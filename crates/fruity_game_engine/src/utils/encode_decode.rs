use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine,
};

use crate::{FruityError, FruityResult};

const CUSTOM_ENGINE: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::PAD);

/// Encode a byte array into a base64 string
pub fn encode_base_64(bytes: Vec<u8>) -> FruityResult<String> {
    let mut buffer = String::new();
    CUSTOM_ENGINE.encode_string(bytes, &mut buffer);

    Ok(buffer)
}

/// Decode a byte array from a base64 string
pub fn decode_base_64(string: String) -> FruityResult<Vec<u8>> {
    let mut buffer: Vec<u8> = Vec::new();
    CUSTOM_ENGINE
        .decode_vec(string, &mut buffer)
        .map_err(|err| FruityError::GenericFailure(err.to_string()))?;

    Ok(buffer)
}
