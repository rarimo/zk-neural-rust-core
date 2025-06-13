#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct TensorInvoker TensorInvoker;

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

/**
 * Frees the memory allocated for the ZkNeuralCoreResult.
 *
 * # Arguments
 * * `result` - A pointer to the `ZkNeuralCoreResult` instance to free.
 */
void rs_zkneural_dealloc_result(struct ZkNeuralCoreResult *result);

/**
 * Creates a new instance of the ZKNeuralCore.
 *
 * # Returns
 *
 * Returns a pointer to a newly allocated `ZKNeuralCore` instance.
 */
struct ZKNeuralCore *rs_zkneural_new(void);

/**
 * Frees the memory allocated for the ZKNeuralCore instance.
 *
 * # Arguments
 * * `core` - A pointer to the `ZKNeuralCore` instance to free.
 */
void rs_zkneural_free(struct ZKNeuralCore *core);

/**
 * Sets the callback for generating witnesses in the ZKNeural core.
 *
 * # Arguments
 * * `core` - A pointer to the `ZKNeuralCore` instance.
 * * `callback` - The callback function to set for generating witnesses.
 */
void rs_zkneural_set_generate_witness_callback(struct ZKNeuralCore *core,
                                               GenerateWitnessCallback callback);

/**
 * Sets the callback for generating proofs in the ZKNeural core.
 *
 * # Arguments
 * * `core` - A pointer to the `ZKNeuralCore` instance.
 * * `callback` - The callback function to set for generating proofs.
 */
void rs_zkneural_set_generate_proof_callback(struct ZKNeuralCore *core,
                                             GenerateProofCallback callback);

/**
 * Generates a witness using the provided ZKNeural core, circuit, and JSON buffers.
 *
 * # Arguments
 *
 * * `core` - A pointer to the `ZKNeuralCore` instance.
 * * `circuit_buffer` - A pointer to the buffer containing the circuit data.
 * * `circuit_len` - The length of the circuit buffer in bytes.
 * * `json_buffer` - A pointer to the buffer containing the JSON data.
 * * `json_len` - The length of the JSON buffer in bytes.
 *
 * # Returns
 *
 * Returns a pointer to a `ZkNeuralCoreResult` containing the result of the witness generation.
 */
struct ZkNeuralCoreResult *rs_zkneural_generate_witness(struct ZKNeuralCore *core,
                                                        const uint8_t *circuit_buffer,
                                                        uintptr_t circuit_len,
                                                        const uint8_t *json_buffer,
                                                        uintptr_t json_len);

/**
 * Generates a proof using the provided ZKNeural core, zkey, and wtns buffers.
 *
 * # Arguments
 *
 * * `core` - A pointer to the `ZKNeuralCore` instance.
 * * `zkey_buffer` - A pointer to the buffer containing the zkey data.
 * * `zkey_len` - The length of the zkey buffer in bytes.
 * * `wtns_buffer` - A pointer to the buffer containing the wtns data.
 * * `wtns_len` - The length of the wtns buffer in bytes.
 *
 * # Returns
 *
 * Returns a pointer to a `ZkNeuralCoreResult` containing the result of the proof generation.
 */
struct ZkNeuralCoreResult *rs_zkneural_generate_proof(struct ZKNeuralCore *core,
                                                      const uint8_t *zkey_buffer,
                                                      uintptr_t zkey_len,
                                                      const uint8_t *wtns_buffer,
                                                      uintptr_t wtns_len);

/**
 * Creates a new `TensorInvoker` instance from the provided model buffer slice.
 *
 * # Panics
 *
 * If the model buffer cannot be deserialized into a valid TFLite model, this function will panic.
 *
 * # Arguments
 *
 * * `model_buffer` - A reference to a buffer containing the serialized TFLite model data.
 * * `model_len` - The length of the model buffer in bytes.
 *
 * # Returns
 *
 * Returns a `TensorInvoker` instance if successful.
 */
struct TensorInvoker *rs_zkneural_tensor_invoker_new(const uint8_t *model_buffer,
                                                     uintptr_t model_len);

void rs_zkneural_tensor_invoker_free(struct TensorInvoker *invoker);

/**
 * Invokes the TensorInvoker with the provided image buffer.
 *
 * This function prepares the image data according to the specifications of the TensorInvoker
 *
 * # Arguments
 * * `invoker` - A pointer to the `TensorInvoker` instance.
 * * `image_buffer` - A pointer to the image data buffer.
 * * `image_len` - The length of the image buffer in bytes.
 *
 * # Returns
 *
 * Returns a pointer to a `ZkNeuralCoreResult` containing the result of the invocation.
 */
struct ZkNeuralCoreResult *rs_zkneural_tensor_invoker_image_fire(struct TensorInvoker *invoker,
                                                                 const uint8_t *image_buffer,
                                                                 uintptr_t image_len);

/**
 * Allocates a buffer of the specified length.
 *
 * # Arguments
 * * `len` - The length of the buffer to allocate in bytes.
 *
 * # Returns
 *
 * Returns a pointer to the allocated buffer. If allocation fails, it returns a null pointer.
 */
uint8_t *rs_zkneural_alloc(uintptr_t len);

/**
 * Deallocates a buffer previously allocated with `rs_zkneural_alloc`.
 *
 * # Arguments
 * * `ptr` - A pointer to the buffer to deallocate.
 * * `len` - The length of the buffer in bytes.
 */
void rs_zkneural_dealloc(uint8_t *ptr, uintptr_t len);
