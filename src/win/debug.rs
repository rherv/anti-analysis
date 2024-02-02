// anti debug testing
pub fn calculate_function_checksum(critical_function: fn(), after_critical_function: fn()) -> u32 {
    let mut checksum: u32 = 0;
    let function_start_ptr = critical_function as *const u8;
    let function_end_ptr = after_critical_function as *const u8;
    let mut current_function_ptr = function_start_ptr;

    while current_function_ptr < function_end_ptr {
        checksum = checksum.wrapping_add(unsafe { *current_function_ptr as u32 });
        current_function_ptr = unsafe { current_function_ptr.add(1) };
    }

    checksum
}
