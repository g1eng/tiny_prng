mod utils;

use wasm_bindgen::prelude::*;
use tiny_prng::{
    mt64::*, xorshift::*, pcg::*
};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

macro_rules! gen_seed {
    ($v:ident) => {
        {
            ($v << 32) ^ (($v % 0x91828376)  ^ ($v >> 1))
        }
    };

}


#[wasm_bindgen]
pub fn pcg(seed: u32, count: usize) -> Vec<u64> {
    match count  {
        x if x > 0 => {
            let mut v = Vec::with_capacity(count);
            let mut seed = seed as u128;
            seed = (seed << 58)  ^ seed;
            seed = seed.wrapping_mul(0xa081fc3719);
            let mut generator = PcgXslRr::with_seed(gen_seed!(seed));
            for _ in 0..count {
                v.push(generator.generate());
            }
            v
        },
        _ => vec![],
    }
}


#[wasm_bindgen]
pub fn xorshift64(seed: u32,count: usize) -> Vec<u64> {
    let mut v = Vec::with_capacity(count);
    let mut seed = seed as u64;
    seed = (seed << 21) + ((seed << 12) ^ seed);
    seed = seed.wrapping_mul(0xa081fc3719);
    let mut generator = Xorshift64::with_seed(gen_seed!(seed));
    for _ in 0..count {
         v.push(generator.generate());
    }
    v
}


#[wasm_bindgen]
pub fn mt64(seed: Vec<u64>, count: usize) -> Vec<u64> {
    let mut v = Vec::with_capacity(count);
    let mut generator = Mt19937::with_array(seed);
    for _ in 0..count {
        v.push(generator.generate());
    }
    v
}
