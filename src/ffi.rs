use super::core::ZKNeuralCore;

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

#[unsafe(no_mangle)]
pub extern "C" fn rs_zkneural_new() -> *mut ZKNeuralCore {
    let core = ZKNeuralCore::new();
    Box::into_raw(Box::new(core))
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_zkneural_free(core: *mut ZKNeuralCore) {
    if core.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(core));
    }
}

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

#[unsafe(no_mangle)]
pub extern "C" fn rs_zkneural_alloc(len: usize) -> *mut u8 {
    unsafe {
        let layout = Layout::from_size_align_unchecked(len, mem::align_of::<u8>());
        alloc::alloc(layout)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_zkneural_dealloc(ptr: *mut u8, len: usize) {
    unsafe {
        let layout = Layout::from_size_align_unchecked(len, mem::align_of::<u8>());
        alloc::dealloc(ptr, layout);
    }
}
