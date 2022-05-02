// // Signal api
// extern "C" {}

// // Filesystem based operations
// extern "C" {}

// Resource based operations
extern "C" {
    pub fn deno_read_sync(rid: u32, ptr: *mut u8, length: usize) -> usize;
    pub fn deno_write_sync(rid: u32, ptr: *const u8, length: usize) -> usize;
}

// Process based operations
extern "C" {
    pub fn deno_exit(code: i32) -> ();
    pub fn deno_kill(pid: u32, signal_id: u8) -> ();
    pub fn deno_memory_usage() -> *const [u64; 4];
    pub fn deno_metrics() -> *const [u64; 11];
}

#[no_mangle]
pub unsafe extern "C" fn __alloc(length: usize) -> *const u8 {
    let l = std::alloc::Layout::array::<u8>(length).unwrap();
    std::alloc::alloc(l)
}

#[no_mangle]
pub unsafe extern "C" fn __dealloc(ptr: *mut u8, length: usize) {
    let l = std::alloc::Layout::array::<u8>(length).unwrap();
    std::alloc::dealloc(ptr, l);
}
