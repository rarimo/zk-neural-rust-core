use thiserror::Error;

#[derive(Error, Debug)]
pub enum ZKNeuralError {
    #[error("Image processing error: {0}")]
    ImageProcessingError(#[from] image::ImageError),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("TensorFlow Lite error: {0}")]
    TensorFlowLiteError(#[from] tflitec::Error),

    #[error("Generate witness callback not set")]
    WitnessCallbackNotSet,
    #[error("Generate proof callback not set")]
    ProofCallbackNotSet,
    #[error("Witness generation failed: {0}")]
    WitnessGenerationFailed(String),
    #[error("Proof generation failed: {0}")]
    ProofGenerationFailed(String),
    #[error("Proving type not set")]
    ProvingTypeNotSet,

    #[error("TensorFlow Lite model does not have four dimensions")]
    ModelNotFourDimensional,
    #[error("TenserFlow Lite Model have invalid channel")]
    InvalidModelChannel,
    #[error("TenserFlow Lite Model have invalid data type")]
    InvalidModelDataType,

    #[error("Face not found")]
    FaceNotFound,
}
