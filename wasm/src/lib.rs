#![no_std]

extern crate alloc;

/// SAFETY: The runtime environment must be single-threaded WASM.
#[global_allocator]
static ALLOCATOR: talc::TalckWasm = unsafe { talc::TalckWasm::new_global() };

pub mod utils;
pub mod store;
pub mod opfs;
pub mod linked;
pub mod day;
pub mod provider;
pub mod local_storage;