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
    generate_real32!();
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
    generate_real32!();
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
    generate_real32!();
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
    generate_real64!();
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
    generate_real64!();
}

mod tests {
    use crate::{gen_delta_rate, generate_unit_test, generate_unit_test_real1, generate_unit_test_real2, generate_unit_test_real_ranged};
    use super::{PcgXslRr6432Mcg, PcgXslRr, PcgXslRrMcg, PcgXshRs6432, PcgXshRr6432};

    const COUNT: usize = 100 * 1000;

    //pcg-xsh-rr-64/32
    generate_unit_test!(PcgXshRr6432, test_pcg_xsh_rr6432_avr100k, u32, 0x1818729182367349, COUNT);
    generate_unit_test_real1!(PcgXshRr6432, test_pcg_xsh_rr6432_real1_avr100k, f64, 0x1818729182367349, COUNT);
    generate_unit_test_real2!(PcgXshRr6432, test_pcg_xsh_rr6432_real2_avr100k, f64, 0x1818729182367349, COUNT);
    generate_unit_test_real_ranged!(PcgXshRr6432, test_pcg_xsh_rr6432_real_ranged_avr100k, f64, 0x1818729182367349, COUNT);

    //pcg-xsh-rs-64/32
    generate_unit_test!(PcgXshRs6432, test_pcg_xsh_rs6432_avr100k, u32, 0x1818729182367349, COUNT);
    generate_unit_test_real1!(PcgXshRs6432, test_pcg_xsh_rs6432_real1_avr100k, f64, 0x1818729182367349, COUNT);
    generate_unit_test_real2!(PcgXshRs6432, test_pcg_xsh_rs6432_real2_avr100k, f64, 0x2828729282367349, COUNT);
    generate_unit_test_real_ranged!(PcgXshRs6432, test_pcg_xsh_rs6432_real_ranged_avr100k, f64, 0x2828729282367349, COUNT);

    //pcg-xsl-rr-128/64
    generate_unit_test!(PcgXslRr, test_pcg_xsh_rr_avr100k_avr100k, u64, 0x1818729182367349, COUNT);
    generate_unit_test_real1!(PcgXslRr, test_pcg_xsh_rr_avr100k_real1_avr100k, f64, 0x1818729182367349, COUNT);
    generate_unit_test_real2!(PcgXslRr, test_pcg_xsh_rr_avr100k_real2_avr100k, f64, 0x1818729182367349, COUNT);
    generate_unit_test_real_ranged!(PcgXslRr, test_pcg_xsh_rr_avr100k_real_ranged_avr100k, f64, 0x1818729182367349, COUNT);

    //pcg-xsl-rr-64/32-mcg
    generate_unit_test!(PcgXslRrMcg, test_pcg_xsl_rr_mcg_avr100k, u64, 0x1818729182367349, COUNT);
    generate_unit_test_real1!(PcgXslRrMcg, test_pcg_xsl_rr_mcg_real1_avr100k, f64, 0x1818729182367349, COUNT);
    generate_unit_test_real2!(PcgXslRrMcg, test_pcg_xsl_rr_mcg_real2_avr100k, f64, 0x1818729182367349, COUNT);
    generate_unit_test_real_ranged!(PcgXslRrMcg, test_pcg_xsl_rr_mcg_real_ranged_avr100k, f64, 0x1818729182367349, COUNT);

    //pcg-xsl-rr-128/64-mcg
    generate_unit_test!(PcgXslRr6432Mcg, test_pcg_xsl_rr6432_mcg_avr100k, u32, 0x1818729182367349, COUNT);
    generate_unit_test_real1!(PcgXslRr6432Mcg, test_pcg_xsl_rr6432_mcg_real1_avr100k, f64, 0x1818729182367349, COUNT);
    generate_unit_test_real2!(PcgXslRr6432Mcg, test_pcg_xsl_rr6432_mcg_real2_avr100k, f64, 0x1818729182367349, COUNT);
    generate_unit_test_real_ranged!(PcgXslRr6432Mcg, test_pcg_xsl_rr6432_mcg_real_ranged_avr100k, f64, 0x1818729182367349, COUNT);

    #[bench]
    fn bench_pcgxslrr12864_10mil(b: &mut test::Bencher) {
        b.iter(|| {
            let mut s = PcgXslRr::with_seed(13378593);
            let mut v: u64 = 0;
            for _ in 0..100 * COUNT {
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
            for _ in 0..100 * COUNT {
                v = s.generate();
            };
            println!("{:x}", v);
        })
    }
}
