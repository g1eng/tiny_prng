mod utils;

use wasm_bindgen::prelude::*;
use tiny_prng::{mt64, xorshift, pcg};

#[wasm_bindgen(js_name = Pcg)]
pub struct Pcg {
    generator: pcg::PcgXslRr,
}

#[wasm_bindgen(js_class = Pcg)]
impl Pcg {
    #[wasm_bindgen(constructor)]
    pub fn new(seed: u32) -> Self {
        Pcg {
            generator: pcg::PcgXslRr::with_seed(seed as u128),
        }
    }
    #[wasm_bindgen(js_name = generate)]
    pub fn generate(&mut self) -> u64 {
        self.generator.generate()
    }

    #[wasm_bindgen(js_name = generate_list)]
    pub fn generate_list(&mut self, count: usize) -> Vec<u64> {
        let mut v = Vec::with_capacity(count);
        for _ in 0..count {
            v.push(self.generator.generate());
        }
        v
    }
}

#[wasm_bindgen(js_name = Xorshift64)]
pub struct Xorshift64 {
    generator: xorshift::Xorshift64,
}

#[wasm_bindgen(js_class = Xorshift64)]
impl Xorshift64 {
    #[wasm_bindgen(constructor)]
    pub fn new(seed: u32) -> Self {
        Self {
            generator: xorshift::Xorshift64::with_seed(seed as u64),
        }
    }

    #[wasm_bindgen(js_name = generate)]
    pub fn generate(&mut self) -> u64 {
        self.generator.generate()
    }

    #[wasm_bindgen(js_name = generate_list)]
    pub fn generate_list(&mut self, count: usize) -> Vec<u64> {
        let mut v = Vec::with_capacity(count);
        for _ in 0..count {
            v.push(self.generator.generate());
        }
        v
    }
}

#[wasm_bindgen(js_name = Mt64)]
pub struct Mt64 {
    generator: mt64::Mt19937,
}

#[wasm_bindgen(js_class = Mt64)]
impl Mt64 {
    #[wasm_bindgen(constructor)]
    pub fn new(seed: u32) -> Self {
        Self {
            generator: mt64::Mt19937::with_array(vec![seed as u64])
        }
    }

    #[wasm_bindgen(js_name = generate)]
    pub fn generate(&mut self) -> u64 {
        self.generator.generate()
    }

    #[wasm_bindgen(js_name = generate_list)]
    pub fn generate_list(&mut self, count: usize) -> Vec<u64> {
        let mut v = Vec::with_capacity(count);
        for _ in 0..count {
            v.push(self.generator.generate());
        }
        v
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_pcg_list() {
        let mut p = Pcg::new(5);
        let v = p.generate_list(10);
        assert_eq!(v.len(), 10);
    }

    #[test]
    fn test_pcg() {
        let mut p = Pcg::new(5);
        let a = p.generate();
        let b = p.generate();
        assert_ne!(a, b);
    }

    #[test]
    fn test_xorshift64_list() {
        let mut p = Xorshift64::new(5);
        let v = p.generate_list(10);
        assert_eq!(v.len(), 10);
    }

    #[test]
    fn test_xorshift64() {
        let mut p = Xorshift64::new(5);
        let a = p.generate();
        let b = p.generate();
        assert_ne!(a, b);
    }

    #[test]
    fn test_mt64_list() {
        let mut p = Mt64::new(5);
        let v = p.generate_list(10);
        assert_eq!(v.len(), 10);
    }

    #[test]
    fn test_mt64() {
        let mut p = Mt64::new(5);
        let a = p.generate();
        let b = p.generate();
        assert_ne!(a, b);
    }
}