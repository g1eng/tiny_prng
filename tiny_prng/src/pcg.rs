//! The `pcg` module implements Permutational Computation Generator (PCG) family with Linear Congruential Generator (LCG).
//!
//! ```rust
//! use tiny_prng::pcg::PcgXshRr6432;
//! use std::time::SystemTime;
//!
//! const MODV: u128 = 19937 * 273;
//! const MODS: usize = 11;
//!
//! fn main(){
//!     let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
//!     /// Note: That's good to refer additional source(s) to calculate the seed
//!     let seed = ((now % MODV) << MODS) + now;
//!     /// A generator must be a mutable because its internal state alters at the random number generation.
//!     let mut x = PcgXshRr6432::with_seed(seed as u64);
//!     println!("{} {} {}", x.generate(),x.generate(),x.generate());
//! }
//!

use std::ops::{BitAnd, BitOr, Shl, Shr};
use crate::{generate_real64, generate_real32};

pub static MULTIPLIER: u64 = 1957840684519283055;
pub static MULTIPLIER128: u128 = 0x1957840684519283055;
pub static INCREMENT: u64 = 3571826365018266039;
pub static INCREMENT128: u128 = 0x3571826365018266039;


macro_rules! rotr32 {
    ($x:expr, $r:expr) => {{
        ($x >> $r) | ($x << ((32-$r) & 31))
    }};
}

macro_rules! rotr64 {
    ($x:expr, $r:expr) => {{
        ($x >> $r) | ($x << ((64-$r) & 63))
    }}
}


// The generator for PCG-XSH-RR-64/32 with LCG.
pub struct PcgXshRr6432 {
    state: u64,
}

impl PcgXshRr6432 {
    #[inline]
    pub fn with_seed(seed: u64) -> Self {
        Self { state: seed }
    }

    #[inline]
    // generate a pseudo random number with the current state of the generator
    pub fn generate(&mut self) -> u32 {
        let mut x = self.state;
        let count = (self.state >> 59) as u32;
        self.state = x.wrapping_mul(MULTIPLIER)
            .wrapping_add(INCREMENT);
        x ^= x >> 18;
        rotr32!((x>>27) as u32, count)
    }
    generate_real32!(self);
}


// The generator for PCG-XSL-RR-64/32 with MCG.
pub struct PcgXslRr6432Mcg {
    state: u64,
}

impl PcgXslRr6432Mcg {
    #[inline]
    pub fn with_seed(seed: u64) -> Self {
        Self { state: seed }
    }

    #[inline]
    // generate a pseudo random number with the current state of the generator
    pub fn generate(&mut self) -> u32 {
        let mut x = self.state;
        let count = (self.state >> 59) as u32;
        self.state = x.wrapping_mul(MULTIPLIER);
        x ^= x >> 18;
        rotr32!((x>>27) as u32, count)
    }
    generate_real32!(self);
}


// The generator for PCG-XSH-RS-64/32 with LCG.
pub struct PcgXshRs6432 {
    state: u64,
}

impl PcgXshRs6432 {
    #[inline]
    pub fn with_seed(seed: u64) -> Self {
        Self { state: seed }
    }
    #[inline]
    // generate a pseudo random number with the current state of the generator
    pub fn generate(&mut self) -> u32 {
        let mut x = self.state;
        let count = 22 + (self.state >> 61) as u32;
        self.state = x.wrapping_mul(MULTIPLIER)
            .wrapping_add(INCREMENT);
        x ^= x >> 22;
        (x >> count) as u32
    }
    generate_real32!(self);
}


// The generator for PCG-XSL-RS-128/64 with LCG.
pub struct PcgXslRr {
    state: u128,
}


impl PcgXslRr {
    #[inline]
    pub fn with_seed(seed: u128) -> Self {
        Self { state: seed }
    }
    #[inline]
    // generate a pseudo random number with the current state of the generator
    pub fn generate(&mut self) -> u64 {
        let mut x = self.state;
        let count = (self.state >> 122) as u64;
        self.state = x.wrapping_mul(MULTIPLIER128)
            .wrapping_add(INCREMENT128);
        x ^= x >> 64;
        rotr64!(x as u64, count)
    }
    generate_real64!(self);
}

pub struct PcgXslRrMcg {
    state: u128,
}

// The generator for PCG-XSL-RR-128/64 with MCG.
impl PcgXslRrMcg {
    #[inline]
    pub fn with_seed(seed: u128) -> Self {
        Self { state: seed }
    }

    #[inline]
    // generate a pseudo random number with the current state of the generator
    pub fn generate(&mut self) -> u64 {
        let mut x = self.state;
        let count = (self.state >> 122) as u64;
        self.state = x.wrapping_mul(MULTIPLIER128);
        x ^= x >> 64;
        rotr64!(x as u64, count)
    }
    generate_real64!(self);
}

mod tests {
    use super::*;
    #[test]
    fn test_pcg_xsh_rr6432() {
        let mut x = PcgXshRr6432::with_seed(0x1818729182367349);
        let acceptable_delta = u32::MAX / 100;
        let mut sum = 0;
        for _ in 0..10000 {
            sum += x.generate() / 10000;
        }
        let delta = match sum > u32::MAX / 2 {
            true => sum - u32::MAX / 2,
            false => u32::MAX / 2 - sum
        };
        assert!(delta < acceptable_delta);
    }

    #[test]
    fn test_pcg_xsh_rs6432() {
        let mut x = PcgXshRs6432::with_seed(0x1818729182367349);
        let acceptable_delta = u32::MAX / 100;
        let mut sum = 0;
        for _ in 0..10000 {
            sum += x.generate() / 10000;
        }
        let delta = match sum > u32::MAX / 2 {
            true => sum - u32::MAX / 2,
            false => u32::MAX / 2 - sum
        };
        assert!(delta < acceptable_delta);
    }

    #[test]
    fn test_pcg_xsl_rr() {
        let mut x = PcgXslRr::with_seed(0x1818729182367349);
        let acceptable_delta = u64::MAX / 100;
        let mut sum = 0;
        for _ in 0..10000 {
            sum += x.generate() / 10000;
        }
        let delta = match sum > u64::MAX / 2 {
            true => sum - u64::MAX / 2,
            false => u64::MAX / 2 - sum
        };
        assert!(delta < acceptable_delta);
    }

    #[test]
    fn test_pcg_xsl_rr_mcg() {
        let mut x = PcgXslRrMcg::with_seed(0x1818729182367349);
        let acceptable_delta = u64::MAX / 100;
        let mut sum = 0;
        for _ in 0..10000 {
            sum += x.generate() / 10000;
        }
        let delta = match sum > u64::MAX / 2 {
            true => sum - u64::MAX / 2,
            false => u64::MAX / 2 - sum
        };
        assert!(delta < acceptable_delta);
    }


    #[test]
    fn test_xsh_rr6432_real1_average100000() {
        let mut p = PcgXshRr6432::with_seed(0x817236);
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
        assert_eq!(true, diff < acceptable_delta);
    }

    #[test]
    fn test_xsh_rr6432_real2_average100000() {
        let mut p = PcgXshRr6432::with_seed(0x817236);
        let mut sum = 0.0;
        let max_count = 100000;
        let acceptable_delta = 1.0 / 100.0;
        for _ in 0..max_count {
            sum += p.generate_real_closed() / max_count as f64;
        }
        let diff = match sum > 0.5 {
            true => sum - 0.5,
            false => 0.5 - sum,
        };
        assert_eq!(true, diff < acceptable_delta);
    }
    #[test]
    fn test_xsh_rr6432_real_ranged_average100000() {
        let mut p = PcgXshRr6432::with_seed(0x817236);
        let mut sum = 0.0;
        let max_count = 100000;
        let acceptable_delta = 2000.0 / 100.0;
        for _ in 0..max_count {
            sum += p.generate_real_in_range(-1000.0, 1000.0) / max_count as f64;
        }
        let diff = match sum > 0.5 {
            true => sum - 0.5,
            false => 0.5 - sum,
        };
        assert_eq!(true, diff < acceptable_delta);
    }

    #[test]
    fn test_xsh_rs6432_real1_average100000() {
        let mut p = PcgXshRr6432::with_seed(0x817236);
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
        assert_eq!(true, diff < acceptable_delta);
    }

    #[test]
    fn test_xsh_rs6432_real2_average100000() {
        let mut p = PcgXshRr6432::with_seed(0x817236);
        let mut sum = 0.0;
        let max_count = 100000;
        let acceptable_delta = 1.0 / 100.0;
        for _ in 0..max_count {
            sum += p.generate_real_closed() / max_count as f64;
        }
        let diff = match sum > 0.5 {
            true => sum - 0.5,
            false => 0.5 - sum,
        };
        assert_eq!(true, diff < acceptable_delta);
    }
    #[test]
    fn test_xsh_rs6432_real_ranged_average100000() {
        let mut p = PcgXshRs6432::with_seed(0x817236);
        let mut sum = 0.0;
        let max_count = 100000;
        let acceptable_delta = 2000.0 / 100.0;
        for _ in 0..max_count {
            sum += p.generate_real_in_range(-1000.0, 1000.0) / max_count as f64;
        }
        let diff = match sum > 0.5 {
            true => sum - 0.5,
            false => 0.5 - sum,
        };
        assert_eq!(true, diff < acceptable_delta);
    }

    #[test]
    fn test_xsl_rr_real1_average100000() {
        let mut p = PcgXslRr::with_seed(0x817236);
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
    }

    #[test]
    fn test_xsl_rr_real2_average100000() {
        let mut p = PcgXslRr::with_seed(0x817236);
        let mut sum = 0.0;
        let max_count = 100000;
        let acceptable_delta = 1.0 / 100.0;
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
    fn test_xsl_rr_real_ranged_average100000() {
        let mut p = PcgXslRr::with_seed(0x817236);
        let mut sum = 0.0;
        let max_count = 100000;
        let acceptable_delta = 2000.0 / 100.0;
        for _ in 0..max_count {
            sum += p.generate_real_in_range(-1000.0, 1000.0) / max_count as f64;
        }
        let diff = match sum > 0.5 {
            true => sum - 0.5,
            false => 0.5 - sum,
        };
        assert_eq!(true, diff < acceptable_delta);
    }

    #[test]
    fn test_xsl_rr_mcg_real1_average100000() {
        let mut p = PcgXslRrMcg::with_seed(0x817236);
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
        assert_eq!(true, diff < acceptable_delta);
    }

    #[test]
    fn test_xsl_rr_mcg_real2_average100000() {
        let mut p = PcgXslRrMcg::with_seed(0x817236);
        let mut sum = 0.0;
        let max_count = 100000;
        let acceptable_delta = 1.0 / 100.0;
        for _ in 0..max_count {
            sum += p.generate_real_closed() / max_count as f64;
        }
        let diff = match sum > 0.5 {
            true => sum - 0.5,
            false => 0.5 - sum,
        };
        assert_eq!(true, diff < acceptable_delta);
    }
    #[test]
    fn test_xsl_rr_mcg_real_ranged_average100000() {
        let mut p = PcgXslRrMcg::with_seed(0x817236);
        let mut sum = 0.0;
        let max_count = 100000;
        let acceptable_delta = 2000.0 / 100.0;
        for _ in 0..max_count {
            sum += p.generate_real_in_range(-1000.0, 1000.0) / max_count as f64;
        }
        let diff = match sum > 0.5 {
            true => sum - 0.5,
            false => 0.5 - sum,
        };
        assert_eq!(true, diff < acceptable_delta);
    }


    #[test]
    fn test_xsl_rr6432_mcg_real1_average100000() {
        let mut p = PcgXslRr6432Mcg::with_seed(0x817236);
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
        assert_eq!(true, diff < acceptable_delta);
    }

    #[test]
    fn test_xsl_rr6432_mcg_real2_average100000() {
        let mut p = PcgXslRr6432Mcg::with_seed(0x817236);
        let mut sum = 0.0;
        let max_count = 100000;
        let acceptable_delta = 1.0 / 100.0;
        for _ in 0..max_count {
            sum += p.generate_real_closed() / max_count as f64;
        }
        let diff = match sum > 0.5 {
            true => sum - 0.5,
            false => 0.5 - sum,
        };
        assert_eq!(true, diff < acceptable_delta);
    }
    #[test]
    fn test_xsl_rr6432_mcg_real_ranged_average100000() {
        let mut p = PcgXslRr6432Mcg::with_seed(0x817236);
        let mut sum = 0.0;
        let max_count = 100000;
        let acceptable_delta = 2000.0 / 100.0;
        for _ in 0..max_count {
            sum += p.generate_real_in_range(-1000.0, 1000.0) / max_count as f64;
        }
        let diff = match sum > 0.5 {
            true => sum - 0.5,
            false => 0.5 - sum,
        };
        assert_eq!(true, diff < acceptable_delta);
    }


    #[bench]
    fn bench_pcgxslrr12864_10mil(b: &mut test::Bencher) {
        b.iter(|| {
            let mut s = PcgXslRr::with_seed(13378593);
            let mut v: u64 = 0;
            for _ in 0..10000000usize {
                v = s.generate();
            };
            println!("{:x}", v);
        })
    }

    #[bench]
    fn bench_pcgxshrr6432_10mil(b: &mut test::Bencher) {
        b.iter(|| {
            let mut s = PcgXshRr6432::with_seed(0x89178726ab1f8ab3);
            let mut v: u32 = 0;
            for _ in 0..10000000usize {
                v = s.generate();
            };
            println!("{:x}", v);
        })
    }
}
