//! The `mt64` module provides 64bit implementation for Mersenne Twister 19937.
//!
//! ```rust
//! use tiny_prng::mt64::Mt19937;
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
//!     let mut x = Mt19937::with_array(vec![123,234,345,456]);
//!     println!("{} {} {}", x.generate(),x.generate(),x.generate());
//! }
//! ```

use crate::generate_real64;

const N :usize = 312;
const M :usize = 156;
const MATRIX_A: u64 = 0xB5026F5AA96619E9;
const UPPER_MASK: u64 =  0xFFFFFFFF80000000;
const LOWER_MASK: u64 = 0x7FFFFFFF;

pub struct Mt19937 {
    state: [u64; N],
    index: usize,
}

impl Mt19937 {
    fn init_genrand(&mut self, s :u64) {
        self.state[0] = s & 0xffffffffffffffffu64;
        self.index = 1;
        while self.index < N {
            self.state[self.index] = 6364136223846793005u64.wrapping_mul  (self.state[self.index-1] ^ (self.state[self.index-1] >> 30)) + (self.index as u64);
            self.state[self.index] &= 0x5555555555555555u64;
            self.index += 1;
        }
    }

    pub fn generate(&mut self) -> u64 {
        let mut y :u64;
        const MAG01: [u64;2] = [0x0u64,MATRIX_A];
        while self.index >= N {
            if self.index == N+1 {
                self.init_genrand(5489u64);
            }
            for kk in 0..N-M {
                y = (self.state[kk]&UPPER_MASK)|(self.state[kk+1]&LOWER_MASK);
                self.state[kk] = self.state[kk+M] ^ (y >> 1) ^ MAG01[(y as usize) & 0x1usize];
            }
            for kk in N-M..N-1 {
                y = (self.state[kk]&UPPER_MASK)|(self.state[kk+1]&LOWER_MASK);
                self.state[kk] = self.state[kk+M-N] ^ (y >> 1) ^ MAG01[(y as usize) & 0x1usize];
            }
            y = (self.state[N-1]&UPPER_MASK)|(self.state[0]&LOWER_MASK);
            self.state[N-1] = self.state[M-1] ^ (y >> 1) ^ MAG01[(y as usize) & 0x1usize];

            self.index = 0;
        }
        y = self.state[self.index];
        self.index += 1;


        // tempering
        y ^= y >> 11;
        y ^= (y << 7) & 0x9d2c5680;
        y ^= (y << 15) & 0xefc60000;
        y ^= y >> 18;
        y as u64
    }


    pub fn with_array(init_key: Vec<u64>) -> Self {
        let  mut i :usize;
        let mut j :usize;
        let k :usize;
        let key_len = init_key.len();
        let mut mt = Self {
            state: [0;N],
            index: 0,
        };
        mt.init_genrand(19650218);
        i=1;
        j=0;
        k = match N>key_len {
            true => N,
            false => key_len.clone(),
        };
        for _ in 0..k {
            mt.state[i] = (mt.state[i] ^ ((mt.state[i - 1] ^ (mt.state[i - 1] >> 30)) * 1664525))
                + init_key[j] + (j as u64);
            mt.state[i] &= 0xffffffff;
            i += 1;
            j += 1;
            if i >= N {
                mt.state[0] = mt.state[N - 1];
                i = 1;
            }
            if j >= key_len {
                j=0;
            }
        }
        for _ in 0..N-1 {
            mt.state[i] = (mt.state[i] ^ ((mt.state[i-1] ^ (mt.state[i-1] >> 30)) * 1566083941))
                - (i as u64);
            mt.state[i] &= 0xffffffff;
            i+=1;
            if i>=N { mt.state[0] = mt.state[N-1]; i=1; }
        }
        mt
    }

    generate_real64!(self);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mt64_init_and_generate_with_long_array() {
        let mut mt = Mt19937::with_array ([0x1;N+32].to_vec());
        assert_ne!(0x1,mt.generate());
    }
    #[test]
    fn test_mt64_quality_generate_average100000() {
        let mut mt = Mt19937::with_array (vec![0x123, 0x234, 0x345, 0x456]);
        let mut sum :u64 = 0;
        let max_count = 100000;
        let acceptable_delta = u64::MAX / 100;
        for _ in 0..max_count {
            sum += mt.generate() / max_count;
        }
        let diff = match sum > u64::MAX / 2 {
            true => sum - u64::MAX / 2 ,
            false => u64::MAX / 2 - sum ,
        };
        assert_eq!(true, diff < acceptable_delta);
    }


    #[test]
    fn test_quality_genrand_real1_average100000() {
        let mut mt = Mt19937::with_array (vec![0x123, 0x234, 0x345, 0x456]);
        let mut sum = 0.0;
        let max_count = 100000;
        let acceptable_delta = 1.0 / 100.0;
        for _ in 0..max_count {
            sum += mt.generate_real() / max_count as f64;
        }
        let diff = match sum > 0.5 {
            true => sum - 0.5,
            false => 0.5 - sum ,
        };
        assert_eq!(true, diff < acceptable_delta);
    }

    #[test]
    fn test_quality_genrand_real2_average100000() {
        let mut mt = Mt19937::with_array (vec![0x123, 0x234, 0x345, 0x456]);
        let mut sum = 0.0;
        let max_count = 100000;
        let acceptable_delta = 1.0 / 100.0;
        for _ in 0..max_count {
            sum += mt.generate_real_closed() / max_count as f64;
        }
        let diff = match sum > 0.5 {
            true => sum - 0.5,
            false => 0.5 - sum ,
        };
        assert_eq!(true, diff < acceptable_delta);
    }

    #[bench]
    fn bench_mt19937_10mil(b: &mut test::Bencher) {
        b.iter(|| {
            let mut s = Mt19937::with_array(vec![0x123, 0x456, 0x789, 0xabc, 0xdef]);
            let mut v: u64 = 0;
            for _ in 0..10000000usize {
                v = s.generate();
            };
            println!("{:x}", v);
        })
    }
}
