//! The `xorshift` module implements xorshift family PRNG.
//!
//! ```rust
//! use tiny_prng::xorshift::Xorshift64;
//! use std::time::SystemTime;
//!
//! const MODV: u128 = 19937 * 273;
//! const MODS: usize = 11;
//!
//! fn main(){
//!     let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
//!     // Note: That's good to refer additional source(s) to calculate the seed
//!     let seed = ((now % MODV) << MODS) + now;
//!     // A generator must be a mutable because its internal state alters at the random number generation.
//!     let mut x = Xorshift64::with_seed(seed as u64);
//!     println!("{} {} {}", x.generate(),x.generate(),x.generate());
//! }
//! ```

use crate::{generate_real64, generate_real32, generate_real128};

pub struct Xorshift32 {
    state: u32,
}

impl Xorshift32 {
    #[inline]
    pub fn with_seed(seed: u32) -> Self {
        Self { state: seed }
    }

    #[inline]
    pub fn generate(&mut self) -> u32 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 17;
        self.state ^= self.state << 5;
        self.state.clone()
    }
    generate_real32!();
}

pub struct Xorshift64 {
    state: u64,
}

impl Xorshift64 {
    #[inline]
    pub fn with_seed(seed: u64) -> Self {
        Self { state: seed }
    }

    #[inline]
    pub fn generate(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state.clone()
    }
    generate_real64!();
}

pub struct Xorshift128 {
    state: [u32; 4],
}

impl Xorshift128 {
    #[inline]
    pub fn with_seed(seed: u128) -> Self {
        Self {
            state: [
                (seed >> 96) as u32,
                ((seed >> 64) & 0xffffffff) as u32,
                ((seed >> 32) & 0xffffffff) as u32,
                (seed & 0xffffffff) as u32,
            ],
        }
    }

    #[inline]
    pub fn generate(&mut self) -> u128 {
        let mut t: u32 = self.state[3];
        let s: u32 = self.state[0];
        self.state[3] = self.state[2];
        self.state[2] = self.state[1];
        self.state[1] = s;
        t ^= t << 11;
        t ^= t >> 8;
        self.state[0] = t ^ s ^ (s >> 19);
        ((self.state[0].clone() as u128) << 96)
            | ((self.state[1].clone() as u128) << 64)
            | ((self.state[2].clone() as u128) << 32)
            | (self.state[3].clone() as u128)
    }
    generate_real128!();
}

pub struct Xorshift64star {
    state: u64,
}

impl Xorshift64star {
    #[inline]
    pub fn with_seed(seed: u64) -> Self {
        Self { state: seed }
    }

    #[inline]
    pub fn generate(&mut self) -> u64 {
        self.state ^= self.state >> 12;
        self.state ^= self.state << 25;
        self.state ^= self.state >> 27;
        self.state.wrapping_mul(0xa738f8117ca1d037)
    }
    generate_real64!();
}

pub struct Xorshift1024star {
    state: [u64; 16],
    index: usize,
}
impl Xorshift1024star {
    #[inline]
    pub fn with_seed(seed: [u64; 16]) -> Self {
        Self {
            state: seed,
            index: 0,
        }
    }

    #[inline]
    pub fn generate(&mut self) -> u64 {
        let mut index = self.index;
        let s = self.state[index];
        index += 1;
        index &= 15;
        let mut t = self.state[index];
        t ^= t << 31;
        t ^= t >> 11;
        t ^= s ^ (s >> 30);
        self.state[index] = t;
        self.index = index;
        // if self.index > 15 {
        //     self.index = 0;
        // }
        self.state[index].wrapping_mul(0xaac17d8efa43cab7)
    }

    generate_real64!();
}
mod tests {
    use crate::{generate_unit_test, gen_delta_rate, generate_unit_test_real_ranged, generate_unit_test_real1, generate_unit_test_real2};
    use super::*;

    const COUNT: usize = 100 * 1000;

    //xorshift32
    generate_unit_test!(Xorshift32, test_xorshift32_avr100k, u32, 0x1818729, COUNT);
    generate_unit_test_real1!(Xorshift32, test_xorshift32_real1_avr100k, f64, 0x1818729, COUNT);
    generate_unit_test_real2!(Xorshift32, test_xorshift32_real2_avr100k, f64, 0x1818729, COUNT);
    generate_unit_test_real_ranged!(Xorshift32, test_xorshift32_real_ranged_avr100k, f64, 0x1818729, COUNT);

    //xorshift64
    generate_unit_test!(Xorshift64, test_xorshift64_avr100k, u64, 0x1818729, COUNT);
    generate_unit_test_real1!(Xorshift64, test_xorshift64_real1_avr100k, f64, 0x1818729, COUNT);
    generate_unit_test_real2!(Xorshift64, test_xorshift64_real2_avr100k, f64, 0x1818729, COUNT);
    generate_unit_test_real_ranged!(Xorshift64, test_xorshift64_real_ranged_avr100k, f64, 0x1818729, COUNT);

    //xorshift64star
    generate_unit_test!(Xorshift64star, test_xorshift64star_avr100k, u64, 0x1818729, COUNT);
    generate_unit_test_real1!(Xorshift64star, test_xorshift64star_real1_avr100k, f64, 0x1818729, COUNT);
    generate_unit_test_real2!(Xorshift64star, test_xorshift64star_real2_avr100k, f64, 0x1818729, COUNT);
    generate_unit_test_real_ranged!(Xorshift64star, test_xorshift64star_real_ranged_avr100k, f64, 0x1818729, COUNT);

    //xorshift128
    generate_unit_test!(Xorshift128, test_xorshift128_avr100k, u128, 0x1818729, COUNT);
    generate_unit_test_real1!(Xorshift128, test_xorshift128_real1_avr100k, f64, 0x1818729, COUNT);
    generate_unit_test_real2!(Xorshift128, test_xorshift128_real2_avr100k, f64, 0x1818729, COUNT);
    generate_unit_test_real_ranged!(Xorshift128, test_xorshift128_real_ranged_avr100k, f64, 0x1818729, COUNT);

    #[test]
    fn test_xorshift64star() {
        let mut s = Xorshift64star::with_seed(1337);
        let mut sum = 0;
        let acceptable_delta = u64::MAX / 100;
        for _ in 0..10000 {
            sum += s.generate() / 10000;
        }
        let delta = match sum > u64::MAX / 2 {
            true => sum - u64::MAX / 2,
            false => u64::MAX / 2 - sum,
        };
        assert!(delta < acceptable_delta);
    }

    #[test]
    fn test_xorshift1024star() {
        let mut s = Xorshift1024star::with_seed([0xab91937581; 16]);
        let mut sum = 0;
        let acceptable_delta = u64::MAX / 100;
        for _ in 0..10000 {
            sum += s.generate() / 10000;
        }
        let delta = match sum > u64::MAX / 2 {
            true => sum - u64::MAX / 2,
            false => u64::MAX / 2 - sum,
        };
        assert!(delta < acceptable_delta);
    }

    #[bench]
    fn bench_xorshift64_10mil(b: &mut test::Bencher) {
        b.iter(|| {
            let mut s = Xorshift64::with_seed(13378593);
            let mut v: u64 = 0;
            for _ in 0..10000000usize {
                v = s.generate();
            };
            println!("{:x}", v);
        })
    }

    #[bench]
    fn bench_xorshift1024_10mil(b: &mut test::Bencher) {
        b.iter(|| {
            let mut s = Xorshift1024star::with_seed([0x13378593; 16]);
            let mut v: u64 = 0;
            for _ in 0..10000000usize {
                v = s.generate();
            };
            println!("{:x}", v);
        })
    }
}
