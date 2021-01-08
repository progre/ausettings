use serde_json::Value;

pub enum AUCaptureOffsetsError {
    ParseFailed(json5::Error),
    FetchFailed(reqwest::Error),
}

pub struct AUCaptureOffsets {
    json: Value,
}

impl AUCaptureOffsets {
    pub fn fetch(url: &str) -> Result<Self, AUCaptureOffsetsError> {
        let resp = reqwest::blocking::get(url)
            .map_err(|err| AUCaptureOffsetsError::FetchFailed(err))?
            .text()
            .map_err(|err| AUCaptureOffsetsError::FetchFailed(err))?;
        let json: Value =
            json5::from_str(&resp).map_err(|err| AUCaptureOffsetsError::ParseFailed(err))?;
        Ok(Self { json })
    }

    pub fn game_options_offset(&self, sha256: &str) -> Option<u32> {
        self.json[sha256]["GameOptionsOffset"]
            .as_u64()
            .map(|x| x as u32)
    }
}
