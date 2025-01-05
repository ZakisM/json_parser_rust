#![feature(allocator_api, portable_simd)]

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub mod ast;
pub mod error;
pub mod parser;
pub mod token;
