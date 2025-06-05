use serde::{Deserialize, Serialize};

pub type GrothZkProofPubSignals = Vec<String>;

#[derive(Serialize, Deserialize)]
pub struct GrothZkProofPoints {
    pub pi_a: Vec<String>,
    pub pi_b: Vec<Vec<String>>,
    pub pi_c: Vec<String>,
    pub proof_protocol: String,
}

#[derive(Serialize, Deserialize)]
pub struct GrothZkProof {
    pub proof: GrothZkProofPoints,
    pub pub_signals: GrothZkProofPubSignals,
}
