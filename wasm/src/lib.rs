#![no_std]

extern crate alloc;

use lol_alloc::{AssumeSingleThreaded, FreeListAllocator};

// SAFETY: This application is single threaded, so using AssumeSingleThreaded is allowed.
#[global_allocator]
static ALLOCATOR: AssumeSingleThreaded<FreeListAllocator> =
    unsafe { AssumeSingleThreaded::new(FreeListAllocator::new()) };

pub mod utils;
pub mod store;
pub mod opfs;
pub mod linked;
pub mod day;
pub mod provider;
pub mod local_storage;