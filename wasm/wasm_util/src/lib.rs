

extern "C" {
    pub fn get_input_size() -> i32;
    pub fn set_input(ptr: i32);
    pub fn get_output(ptr: i32, len: i32);
}

#[no_mangle]
pub extern "C" fn alloc(size: i32) -> *mut u8 {
    let mut buf = Vec::with_capacity(size as usize);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn dealloc(ptr: *mut u8, size: i32) {
    let data = Vec::from_raw_parts(ptr, size as usize, size as usize);
    drop(data);
}


