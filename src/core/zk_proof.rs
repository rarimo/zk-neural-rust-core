use serde::{Deserialize, Serialize};

pub type ZkProofPubSignals = Vec<String>;

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
    pub pub_signals: ZkProofPubSignals,
}

#[derive(Serialize, Deserialize)]
pub struct UltraGrothProofPoints {
    pub pi_a: Vec<String>,
    pub pi_b: Vec<Vec<String>>,
    pub pi_f: Vec<String>,
    pub pi_r: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UltraGrothProof {
    pub proof: UltraGrothProofPoints,
    pub pub_signals: ZkProofPubSignals,
}
