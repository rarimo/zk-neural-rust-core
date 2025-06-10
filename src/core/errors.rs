use thiserror::Error;

#[derive(Error, Debug)]
pub enum ZKNeuralError {
    #[error("Generate witness callback not set")]
    WitnessCallbackNotSet,
    #[error("Generate proof callback not set")]
    ProofCallbackNotSet,
    #[error("Witness generation failed: {0}")]
    WitnessGenerationFailed(String),
    #[error("Proof generation failed: {0}")]
    ProofGenerationFailed(String),
    #[error("JSON error: {0}")]
    JsonError(serde_json::Error),
    #[error("TensorFlow Lite error: {0}")]
    TensorFlowLiteError(tflitec::Error),
    #[error("TensorFlow Lite model does not have four dimensions")]
    ModelNotFourDimensional,
    #[error("Image processing error: {0}")]
    ImageProcessingError(image::ImageError),
    #[error("TenserFlow Lite Model have invalid channel")]
    InvalidModelChannel,
    #[error("TenserFlow Lite Model have invalid data type")]
    InvalidModelDataType,
}
