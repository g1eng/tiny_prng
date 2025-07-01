//! Test suite for the Web and headless browsers.
//!

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use tiny_prng_wasm::{pcg, mt64, xorshift64};

wasm_bindgen_test_configure!(run_in_browser);

const VEC_SIZE: usize = 1000000;

#[wasm_bindgen_test]
fn test_pcg() {
    assert_eq!(pcg(VEC_SIZE).len(), VEC_SIZE);
}


#[wasm_bindgen_test]
fn test_mt64() {
    assert_eq!(mt64(VEC_SIZE).len(), VEC_SIZE);
}


#[wasm_bindgen_test]
fn test_xorshift() {
    assert_eq!(xorshift64(VEC_SIZE).len(), VEC_SIZE);
}
