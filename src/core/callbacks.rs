pub type GenerateWitnessCallback = unsafe extern "C" fn(
    circuit_buffer: *const u8,
    circuit_size: usize,
    json_buffer: *const u8,
    json_size: usize,
    wtns_buffer: *mut u8,
    wtns_size: *mut usize,
    error_msg: *mut u8,
    error_msg_maxsize: usize,
) -> i32;

pub type GenerateProofCallback = unsafe extern "C" fn(
    zkey_buffer: *const u8,
    zkey_size: usize,
    wtns_buffer: *const u8,
    wtns_size: usize,
    proof_buffer: *mut u8,
    proof_size: *mut usize,
    public_buffer: *mut u8,
    public_size: *mut usize,
    error_msg: *mut u8,
    error_msg_maxsize: usize,
) -> i32;
