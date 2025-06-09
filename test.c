#include "headers/zk_neural_rust_core.h"
#include <stdio.h>


const char MOCK_ZK_PROOF_POINTS[] = "{\"pi_a\":[],\"pi_b\":[],\"pi_c\":[],\"proof_protocol\":\"groth16\"}";
const char MOCK_ZK_PROOF_PUB_SIGNALS[] = "[]";

int32_t generate_witness_callback(
    const uint8_t *circuit_buffer,
    uintptr_t circuit_size,
    const uint8_t *json_buffer,
    uintptr_t json_size,
    uint8_t *wtns_buffer,
    uintptr_t *wtns_size,
    uint8_t *error_msg,
    uintptr_t error_msg_maxsize
) {
    
    for (uintptr_t i = 0; i < 1024; i++) {
        wtns_buffer[i] = rand() % 256;
    }

    *wtns_size = 1024;
    return 0;
}

int32_t generate_proof_callback(
   const uint8_t *zkey_buffer,
    uintptr_t zkey_size,
    const uint8_t *wtns_buffer,
    uintptr_t wtns_size,
    uint8_t *proof_buffer,
    uintptr_t *proof_size,
    uint8_t *public_buffer,
    uintptr_t *public_size,
    uint8_t *error_msg,
    uintptr_t error_msg_maxsize
) {
    uintptr_t points_size = sizeof(MOCK_ZK_PROOF_POINTS) - 1;
    if (points_size > error_msg_maxsize) {
        return 2;
    }

    for (uintptr_t i = 0; i < points_size; i++) {
        proof_buffer[i] = MOCK_ZK_PROOF_POINTS[i];
    }

    *proof_size = points_size;

    
    uintptr_t pub_signals_size = sizeof(MOCK_ZK_PROOF_PUB_SIGNALS) - 1;
    if (pub_signals_size > error_msg_maxsize) {
        return 2;
    }

    for (uintptr_t i = 0; i < pub_signals_size; i++) {
        public_buffer[i] = MOCK_ZK_PROOF_PUB_SIGNALS[i];
    }

    *public_size = pub_signals_size;
    
    return 0;
}

void test_proof_generation() {
    ZKNeuralCore* core = rs_zkneural_new();

    rs_zkneural_set_generate_witness_callback(core, generate_witness_callback);
    rs_zkneural_set_generate_proof_callback(core, generate_proof_callback);

    ZkNeuralCoreResult* wtns_result = rs_zkneural_generate_witness(
        core,
        (const uint8_t *)"circuit_data", 12,
        (const uint8_t *)"json_data", 9 
    );

    if (wtns_result->error) {
        printf("Witness generation error: %s\n", wtns_result->error);
        rs_zkneural_dealloc_result(wtns_result);
        rs_zkneural_free(core);
        return;
    }

    ZkNeuralCoreResult* proof_result = rs_zkneural_generate_proof(
        core,
        (const uint8_t *)"zkey_data", 9,
        wtns_result->value, wtns_result->value_size
    );

    if (proof_result->error) {
        printf("Proof generation error: %s\n", proof_result->error);
        rs_zkneural_dealloc_result(wtns_result);
        rs_zkneural_dealloc_result(proof_result);
        rs_zkneural_free(core);
        return;
    }

    printf("Proof generated successfully\n");
    printf("Proof: %.*s\n", (int)proof_result->value_size, proof_result->value);

    rs_zkneural_dealloc_result(wtns_result);
}

int main(void) {
    test_proof_generation();

    return 0;
}