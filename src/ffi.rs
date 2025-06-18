use super::core::ZKNeuralCore;
use super::core::tensor::TensorInvoker;

use super::core::{
    callbacks::{GenerateProofCallback, GenerateWitnessCallback},
    errors::ZKNeuralError,
};

use std::alloc::{self, Layout};
use std::ffi::{CString, c_char};
use std::mem;

#[repr(C)]
pub struct ZkNeuralCoreResult {
    pub value: *mut u8,
    pub value_size: usize,
    pub error: *const c_char,
}

impl ZkNeuralCoreResult {
    pub fn from_rust_result(result: Result<Vec<u8>, ZKNeuralError>) -> *mut ZkNeuralCoreResult {
        match result {
            Ok(value) => {
                let ptr = rs_zkneural_alloc(value.len());
                if ptr.is_null() {
                    return std::ptr::null_mut();
                }
                unsafe {
                    std::ptr::copy_nonoverlapping(value.as_ptr(), ptr, value.len());
                }

                let value_size = value.len();
                Box::into_raw(Box::new(ZkNeuralCoreResult {
                    value: ptr,
                    value_size,
                    error: std::ptr::null(),
                }))
            }

            Err(e) => {
                let error_msg = CString::new(e.to_string()).unwrap();
                Box::into_raw(Box::new(ZkNeuralCoreResult {
                    value: std::ptr::null_mut(),
                    value_size: 0,
                    error: error_msg.into_raw(),
                }))
            }
        }
    }
}

/// Frees the memory allocated for the ZkNeuralCoreResult.
///
/// # Arguments
/// * `result` - A pointer to the `ZkNeuralCoreResult` instance to free.
#[unsafe(no_mangle)]
pub extern "C" fn rs_zkneural_dealloc_result(result: *mut ZkNeuralCoreResult) {
    if result.is_null() {
        return;
    }

    unsafe {
        let res = Box::from_raw(result);
        if !res.value.is_null() {
            alloc::dealloc(
                res.value,
                Layout::from_size_align_unchecked(res.value_size, mem::align_of::<u8>()),
            );
        }
    }
}

/// Creates a new instance of the ZKNeuralCore.
///
/// # Returns
///
/// Returns a pointer to a newly allocated `ZKNeuralCore` instance.
#[unsafe(no_mangle)]
pub extern "C" fn rs_zkneural_new() -> *mut ZKNeuralCore {
    let core = ZKNeuralCore::new();
    Box::into_raw(Box::new(core))
}

/// Frees the memory allocated for the ZKNeuralCore instance.
///
/// # Arguments
/// * `core` - A pointer to the `ZKNeuralCore` instance to free.
#[unsafe(no_mangle)]
pub extern "C" fn rs_zkneural_free(core: *mut ZKNeuralCore) {
    if core.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(core));
    }
}

/// Sets the callback for generating witnesses in the ZKNeural core.
///
/// # Arguments
/// * `core` - A pointer to the `ZKNeuralCore` instance.
/// * `callback` - The callback function to set for generating witnesses.
#[unsafe(no_mangle)]
pub extern "C" fn rs_zkneural_set_generate_witness_callback(
    core: *mut ZKNeuralCore,
    callback: GenerateWitnessCallback,
) {
    if core.is_null() {
        return;
    }
    unsafe {
        let core = &mut *core;
        core.set_generate_witness_callback(callback);
    }
}

/// Sets the callback for generating proofs in the ZKNeural core.
///
/// # Arguments
/// * `core` - A pointer to the `ZKNeuralCore` instance.
/// * `callback` - The callback function to set for generating proofs.
#[unsafe(no_mangle)]
pub extern "C" fn rs_zkneural_set_generate_proof_callback(
    core: *mut ZKNeuralCore,
    callback: GenerateProofCallback,
) {
    if core.is_null() {
        return;
    }
    unsafe {
        let core = &mut *core;
        core.set_generate_proof_callback(callback);
    }
}

/// Generates a witness using the provided ZKNeural core, circuit, and JSON buffers.
///
/// # Arguments
///
/// * `core` - A pointer to the `ZKNeuralCore` instance.
/// * `circuit_buffer` - A pointer to the buffer containing the circuit data.
/// * `circuit_len` - The length of the circuit buffer in bytes.
/// * `json_buffer` - A pointer to the buffer containing the JSON data.
/// * `json_len` - The length of the JSON buffer in bytes.
///
/// # Returns
///
/// Returns a pointer to a `ZkNeuralCoreResult` containing the result of the witness generation.
#[unsafe(no_mangle)]
pub extern "C" fn rs_zkneural_generate_witness(
    core: *mut ZKNeuralCore,
    circuit_buffer: *const u8,
    circuit_len: usize,
    json_buffer: *const u8,
    json_len: usize,
) -> *mut ZkNeuralCoreResult {
    if core.is_null() {
        return std::ptr::null_mut();
    }

    let circuit_slice = unsafe { std::slice::from_raw_parts(circuit_buffer, circuit_len) };
    let json_slice = unsafe { std::slice::from_raw_parts(json_buffer, json_len) };

    let core = unsafe { &*core };
    let result = core.generate_witness(circuit_slice, json_slice);

    ZkNeuralCoreResult::from_rust_result(result)
}

/// Generates a proof using the provided ZKNeural core, zkey, and wtns buffers.
///
/// # Arguments
///
/// * `core` - A pointer to the `ZKNeuralCore` instance.
/// * `zkey_buffer` - A pointer to the buffer containing the zkey data.
/// * `zkey_len` - The length of the zkey buffer in bytes.
/// * `wtns_buffer` - A pointer to the buffer containing the wtns data.
/// * `wtns_len` - The length of the wtns buffer in bytes.
///
/// # Returns
///
/// Returns a pointer to a `ZkNeuralCoreResult` containing the result of the proof generation.
#[unsafe(no_mangle)]
pub extern "C" fn rs_zkneural_generate_proof(
    core: *mut ZKNeuralCore,
    zkey_buffer: *const u8,
    zkey_len: usize,
    wtns_buffer: *const u8,
    wtns_len: usize,
) -> *mut ZkNeuralCoreResult {
    if core.is_null() {
        return std::ptr::null_mut();
    }

    let zkey_slice = unsafe { std::slice::from_raw_parts(zkey_buffer, zkey_len) };
    let wtns_slice = unsafe { std::slice::from_raw_parts(wtns_buffer, wtns_len) };

    let core = unsafe { &*core };
    let result = core.generate_proof(zkey_slice, wtns_slice);

    ZkNeuralCoreResult::from_rust_result(result)
}

/// Creates a new `TensorInvoker` instance from the provided model buffer slice.
///
/// # Panics
///
/// If the model buffer cannot be deserialized into a valid TFLite model, this function will panic.
///
/// # Arguments
///
/// * `model_buffer` - A reference to a buffer containing the serialized TFLite model data.
/// * `model_len` - The length of the model buffer in bytes.
///
/// # Returns
///
/// Returns a `TensorInvoker` instance if successful.
#[unsafe(no_mangle)]
pub extern "C" fn rs_zkneural_tensor_invoker_new(
    model_buffer: *const u8,
    model_len: usize,
) -> *mut TensorInvoker {
    let model_slice = unsafe { std::slice::from_raw_parts(model_buffer, model_len).to_vec() };
    let invoker =
        TensorInvoker::new(&model_slice).expect("Failed to create TensorInvoker from model buffer");

    Box::into_raw(Box::new(invoker))
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_zkneural_tensor_invoker_free(invoker: *mut TensorInvoker) {
    if invoker.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(invoker));
    }
}

/// Invokes the TensorInvoker with the provided image buffer.
///
/// This function prepares the image data according to the specifications of the TensorInvoker
///
/// # Arguments
/// * `invoker` - A pointer to the `TensorInvoker` instance.
/// * `image_buffer` - A pointer to the image data buffer.
/// * `image_len` - The length of the image buffer in bytes.
///
/// # Returns
///
/// Returns a pointer to a `ZkNeuralCoreResult` containing the result of the invocation.
#[unsafe(no_mangle)]
pub extern "C" fn rs_zkneural_tensor_invoker_image_fire(
    invoker: *mut TensorInvoker,
    image_buffer: *const u8,
    image_len: usize,
) -> *mut ZkNeuralCoreResult {
    if invoker.is_null() {
        return std::ptr::null_mut();
    }

    let image_data = unsafe { std::slice::from_raw_parts(image_buffer, image_len) };

    let invoker = unsafe { &mut *invoker };

    let prepared_image_data = match invoker.prepare_image_by_spec(image_data) {
        Ok(data) => data,
        Err(e) => {
            return ZkNeuralCoreResult::from_rust_result(Err(e));
        }
    };

    let result = invoker.fire(&prepared_image_data, true);

    ZkNeuralCoreResult::from_rust_result(result)
}

/// Allocates a buffer of the specified length.
///
/// # Arguments
/// * `len` - The length of the buffer to allocate in bytes.
///
/// # Returns
///
/// Returns a pointer to the allocated buffer. If allocation fails, it returns a null pointer.
#[unsafe(no_mangle)]
pub extern "C" fn rs_zkneural_alloc(len: usize) -> *mut u8 {
    unsafe {
        let layout = Layout::from_size_align_unchecked(len, mem::align_of::<u8>());
        alloc::alloc(layout)
    }
}

/// Deallocates a buffer previously allocated with `rs_zkneural_alloc`.
///
/// # Arguments
/// * `ptr` - A pointer to the buffer to deallocate.
/// * `len` - The length of the buffer in bytes.
#[unsafe(no_mangle)]
pub extern "C" fn rs_zkneural_dealloc(ptr: *mut u8, len: usize) {
    unsafe {
        let layout = Layout::from_size_align_unchecked(len, mem::align_of::<u8>());
        alloc::dealloc(ptr, layout);
    }
}
