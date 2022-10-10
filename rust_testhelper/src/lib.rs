extern crate libc;
use libc::size_t;

extern "C"{
    pub fn pwm_qhash(algorithm : size_t, input : *const u8, input_length : size_t, output : *mut u8, output_capacity : size_t) -> size_t;
}
