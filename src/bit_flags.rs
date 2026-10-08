
/*
Job state is stored in one u32

Bit 0: job is active
Bit 1: job is completed
Bit 2: job has failed
Bits 3-18: job number (16 bits)
Bit 19: endianness (0 = little, 1 = big)
*/

// Check if the job is active by looking at the first bit
pub fn is_active(state: u32) -> bool {
    (state & 1) != 0
}

// Shift bit 1 to the right and check if it's set to 1
pub fn is_completed(state: u32) -> bool {
    ((state >> 1) & 1) != 0
}

// Same idea, but for the failed flag at bit 2
pub fn has_failed(state: u32) -> bool {
    ((state >> 2) & 1) != 0
}

// Get the job number stored in bits 3 through 18
pub fn get_job_number(state: u32) -> u16 {
    // Move the job number to the rightmost 16 bits
    // 0xFFFF is a mask that keeps only those 16 bits
    ((state >> 3) & 0xFFFF) as u16
}

// Bit 19 tells us which byte order to use
pub fn is_big_endian(state: u32) -> bool {
    ((state >> 19) & 1) != 0
}

// Return the job number as two bytes based on the endian flag
pub fn get_job_number_bytes(state: u32) -> [u8; 2] {
    let job_number = get_job_number(state);

    if is_big_endian(state) {
        // Most significant byte comes first
        job_number.to_be_bytes()
    } else {
        // Least significant byte comes first
        job_number.to_le_bytes()
    }
}
