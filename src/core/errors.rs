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
}
