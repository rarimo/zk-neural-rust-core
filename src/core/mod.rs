pub mod callbacks;
pub mod constants;
pub mod errors;
pub mod tensor;
pub mod zk_proof;

use callbacks::{GenerateProofCallback, GenerateWitnessCallback};
use constants::{PROOF_SIZE, PUB_SIGNALS_SIZE, WITNESS_ERROR_MSG_MAXSIZE, WITNESS_SIZE};
use errors::ZKNeuralError;

use crate::core::zk_proof::{GrothZkProof, GrothZkProofPoints, GrothZkProofPubSignals};

pub struct ZKNeuralCore {
    generate_witness_callback: Option<GenerateWitnessCallback>,
    generate_proof_callback: Option<GenerateProofCallback>,
}

impl ZKNeuralCore {
    pub fn new() -> Self {
        ZKNeuralCore {
            generate_witness_callback: None,
            generate_proof_callback: None,
        }
    }

    pub fn set_generate_witness_callback(&mut self, callback: GenerateWitnessCallback) {
        self.generate_witness_callback = Some(callback);
    }

    pub fn set_generate_proof_callback(&mut self, callback: GenerateProofCallback) {
        self.generate_proof_callback = Some(callback);
    }

    pub fn generate_witness(
        &self,
        circuit_buffer: &[u8],
        json_buffer: &[u8],
    ) -> Result<Vec<u8>, ZKNeuralError> {
        if let Some(callback) = self.generate_witness_callback {
            let mut wtns_buffer = vec![0u8; WITNESS_SIZE];
            let mut wtns_size = 0;
            let mut error_msg = vec![0u8; WITNESS_ERROR_MSG_MAXSIZE];

            let result = unsafe {
                callback(
                    circuit_buffer.as_ptr(),
                    circuit_buffer.len(),
                    json_buffer.as_ptr(),
                    json_buffer.len(),
                    wtns_buffer.as_mut_ptr(),
                    &mut wtns_size,
                    error_msg.as_mut_ptr(),
                    error_msg.len(),
                )
            };

            if result != 0 {
                let error_message = String::from_utf8_lossy(&error_msg)
                    .trim_end_matches('\0')
                    .to_string();

                return Err(ZKNeuralError::WitnessGenerationFailed(error_message));
            }

            wtns_buffer.truncate(wtns_size);
            Ok(wtns_buffer)
        } else {
            Err(ZKNeuralError::WitnessCallbackNotSet)
        }
    }

    pub fn generate_proof(
        &self,
        zkey_buffer: &[u8],
        wtns_buffer: &[u8],
    ) -> Result<Vec<u8>, ZKNeuralError> {
        if let Some(callback) = self.generate_proof_callback {
            let mut proof_buffer = vec![0u8; PROOF_SIZE];
            let mut proof_size = 0;
            let mut public_buffer = vec![0u8; PUB_SIGNALS_SIZE];
            let mut public_size = 0;
            let mut error_msg = vec![0u8; WITNESS_ERROR_MSG_MAXSIZE];

            let result = unsafe {
                callback(
                    zkey_buffer.as_ptr(),
                    zkey_buffer.len(),
                    wtns_buffer.as_ptr(),
                    wtns_buffer.len(),
                    proof_buffer.as_mut_ptr(),
                    &mut proof_size,
                    public_buffer.as_mut_ptr(),
                    &mut public_size,
                    error_msg.as_mut_ptr(),
                    error_msg.len(),
                )
            };

            if result == 2 {
                return Err(ZKNeuralError::ProofGenerationFailed(
                    "Proof or public signals buffer is too short".to_string(),
                ));
            }

            if result != 0 {
                let error_message = String::from_utf8_lossy(&error_msg)
                    .trim_end_matches('\0')
                    .to_string();

                return Err(ZKNeuralError::ProofGenerationFailed(error_message));
            }

            proof_buffer.truncate(proof_size);
            public_buffer.truncate(public_size);

            let proof = serde_json::from_slice::<GrothZkProofPoints>(&proof_buffer)
                .map_err(ZKNeuralError::JsonError)?;

            let pub_signals = serde_json::from_slice::<GrothZkProofPubSignals>(&public_buffer)
                .map_err(ZKNeuralError::JsonError)?;

            let groth_proof = GrothZkProof { proof, pub_signals };

            let proof = serde_json::to_vec(&groth_proof).map_err(ZKNeuralError::JsonError)?;

            Ok(proof)
        } else {
            Err(ZKNeuralError::ProofCallbackNotSet)
        }
    }
}
