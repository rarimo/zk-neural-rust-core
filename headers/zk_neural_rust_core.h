#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct ZKNeuralCore ZKNeuralCore;

typedef struct ZKNeuralError ZKNeuralError;

typedef struct ZkNeuralCoreResult {
  uint8_t *value;
  uintptr_t value_size;
  const char *error;
} ZkNeuralCoreResult;

typedef int32_t (*GenerateWitnessCallback)(const uint8_t *circuit_buffer,
                                           uintptr_t circuit_size,
                                           const uint8_t *json_buffer,
                                           uintptr_t json_size,
                                           uint8_t *wtns_buffer,
                                           uintptr_t *wtns_size,
                                           uint8_t *error_msg,
                                           uintptr_t error_msg_maxsize);

typedef int32_t (*GenerateProofCallback)(const uint8_t *zkey_buffer,
                                         uintptr_t zkey_size,
                                         const uint8_t *wtns_buffer,
                                         uintptr_t wtns_size,
                                         uint8_t *proof_buffer,
                                         uintptr_t *proof_size,
                                         uint8_t *public_buffer,
                                         uintptr_t *public_size,
                                         uint8_t *error_msg,
                                         uintptr_t error_msg_maxsize);

void rs_zkneural_dealloc_result(struct ZkNeuralCoreResult *result);

struct ZKNeuralCore *rs_zkneural_new(void);

void rs_zkneural_free(struct ZKNeuralCore *core);

void rs_zkneural_set_generate_witness_callback(struct ZKNeuralCore *core,
                                               GenerateWitnessCallback callback);

void rs_zkneural_set_generate_proof_callback(struct ZKNeuralCore *core,
                                             GenerateProofCallback callback);

struct ZkNeuralCoreResult *rs_zkneural_generate_witness(struct ZKNeuralCore *core,
                                                        const uint8_t *circuit_buffer,
                                                        uintptr_t circuit_len,
                                                        const uint8_t *json_buffer,
                                                        uintptr_t json_len);

struct ZkNeuralCoreResult *rs_zkneural_generate_proof(struct ZKNeuralCore *core,
                                                      const uint8_t *zkey_buffer,
                                                      uintptr_t zkey_len,
                                                      const uint8_t *wtns_buffer,
                                                      uintptr_t wtns_len);

uint8_t *rs_zkneural_alloc(uintptr_t len);

void rs_zkneural_dealloc(uint8_t *ptr, uintptr_t len);
