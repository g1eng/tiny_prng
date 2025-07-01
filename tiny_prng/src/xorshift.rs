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
    pub fn with_seed(seed: u32) -> Self {
        Self { state: seed }
    }

    pub fn generate(&mut self) -> u32 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 17;
        self.state ^= self.state << 5;
        self.state.clone()
    }
    generate_real32!(self);
}

pub struct Xorshift64 {
    state: u64,
}

impl Xorshift64 {
    pub fn with_seed(seed: u64) -> Self {
        Self { state: seed }
    }

    pub fn generate(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state.clone()
    }
    generate_real64!(self);
}

pub struct Xorshift128 {
    state: [u32; 4],
}

impl Xorshift128 {
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
    generate_real128!(self);
}

pub struct Xorshift64star {
    state: u64,
}

impl Xorshift64star {
    pub fn with_seed(seed: u64) -> Self {
        Self { state: seed }
    }
    pub fn generate(&mut self) -> u64 {
        self.state ^= self.state >> 12;
        self.state ^= self.state << 25;
        self.state ^= self.state >> 27;
        self.state.wrapping_mul(0xa738f8117ca1d037)
    }
    generate_real64!(self);
}

pub struct Xorshift1024star {
    state: [u64; 16],
    index: usize,
}
impl Xorshift1024star {
    pub fn with_seed(seed: [u64; 16]) -> Self {
        Self {
            state: seed,
            index: 0,
        }
    }
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

    generate_real64!(self);
}
mod tests{
   use super::*;
    #[test]
    fn test_xorshift32() {
        let mut s = Xorshift32::with_seed(1337);
        let mut sum = 0;
        let acceptable_delta = u32::MAX / 100;
        for _ in 0..10000 {
            sum += s.generate() / 10000;
        }
        let delta = match sum > u32::MAX / 2 {
            true => sum - u32::MAX / 2,
            false => u32::MAX / 2 - sum,
        };
        assert!(delta < acceptable_delta);
    }

    #[test]
    fn test_xorshift64() {
        let mut s = Xorshift64::with_seed(1337);
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
    fn test_xorshift128() {
        let mut s = Xorshift128::with_seed(1337);
        let mut sum = 0;
        let acceptable_delta = u128::MAX / 100;
        for _ in 0..10000 {
            sum += s.generate() / 10000;
        }
        let delta = match sum > u128::MAX / 2 {
            true => sum - u128::MAX / 2,
            false => u128::MAX / 2 - sum,
        };
        assert!(delta < acceptable_delta);
    }
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

    #[test]
    fn test_xorshift32_real1_average100000() {
        let mut p = Xorshift32::with_seed(0x817236);
        let mut sum = 0.0;
        let max_count = 100000;
        let acceptable_delta = 1.0 / 100.0;
        for _ in 0..max_count {
            sum += p.generate_real() / max_count as f64;
        }
        let diff = match sum > 0.5 {
            true => sum - 0.5,
            false => 0.5 - sum,
        };
        assert!(diff < acceptable_delta);

        sum = 0.0;
        for _ in 0..max_count {
            sum += p.generate_real_closed() / max_count as f64;
        }
        let diff = match sum > 0.5 {
            true => sum - 0.5,
            false => 0.5 - sum,
        };
        assert!(diff < acceptable_delta);
    }
    #[test]
    fn test_xorshift64_real1_average100000() {
        let mut p = Xorshift64::with_seed(0x817236);
        let mut sum = 0.0;
        let max_count = 100000;
        let acceptable_delta = 1.0 / 100.0;
        for _ in 0..max_count {
            sum += p.generate_real() / max_count as f64;
        }
        let diff = match sum > 0.5 {
            true => sum - 0.5,
            false => 0.5 - sum,
        };
        assert!(diff < acceptable_delta);

        sum = 0.0;
        for _ in 0..max_count {
            sum += p.generate_real_closed() / max_count as f64;
        }
        let diff = match sum > 0.5 {
            true => sum - 0.5,
            false => 0.5 - sum,
        };
        assert!(diff < acceptable_delta);
    }

    #[test]
    fn test_xorshift128_real1_average100000() {
        let mut p = Xorshift128::with_seed(0x817236);
        let mut sum = 0.0;
        let max_count = 100000;
        let acceptable_delta = 1.0 / 100.0;
        for _ in 0..max_count {
            sum += p.generate_real() / max_count as f64;
        }
        let diff = match sum > 0.5 {
            true => sum - 0.5,
            false => 0.5 - sum,
        };
        assert!(diff < acceptable_delta);

        sum = 0.0;
        for _ in 0..max_count {
            sum += p.generate_real_closed() / max_count as f64;
        }
        let diff = match sum > 0.5 {
            true => sum - 0.5,
            false => 0.5 - sum,
        };
        assert!(diff < acceptable_delta);
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
            let mut s = Xorshift1024star::with_seed([0x13378593;16]);
            let mut v: u64 = 0;
            for _ in 0..10000000usize {
                v = s.generate();
            };
            println!("{:x}", v);
        })
    }

}
