const SFMT_MEXP: usize = 19937;

const SFMT_N: usize = SFMT_MEXP / 128 + 1;

const SFMT_POS1: usize = 122;
const SFMT_SL1: usize = 18;
const SFMT_SL2: usize = 1;

const SFMT_SR1: usize = 1;

const SFMT_MSK1: u32 = 0xdfffffef;
const SFMT_MSK2: u32 = 0xddfecb7f;
const SFMT_MSK3: u32 = 0xbffaffff;
const SFMT_MSK4: u32 = 0xbffffff6;

#[derive(Clone, Copy)]
union W128 {
    u32t: [u32; 4],
    u64t: [u64; 2],
}

impl W128 {
    pub fn zero() -> Self {
        Self { u32t: [0; 4] }
    }
    pub fn one() -> Self {
        Self { u32t: [1; 4] }
    }
    pub fn init8b() -> Self {
        Self {
            u32t: [0x8b8b8b8b; 4],
        }
    }
}

const B: W128 = W128 {
    u32t: [0x8b8b8b8b; 4],
};

struct Sfmt {
    state: [W128; SFMT_N], //internal state
    seek: usize,
    idx: usize, // index counter
}

impl Sfmt {
    pub fn new() -> Self {
        Self {
            state: [W128{u32t: [0x8b8b8b8b; 4]};SFMT_N],
            seek: SFMT_N - 2,
            idx: 0,
        }
    }

    fn gen_rand_all(&mut self) {
        let mut i = 0;
        let mut r1: W128;
        let mut r2: W128;

        r2 = self.state[self.seek + 1];
        while i < SFMT_N - SFMT_POS1 {
            self.do_recursion(i, i + SFMT_POS1);
            r1 = r2;
            r2 = self.state[i];
            i += 1;
        }
        while i < SFMT_N {
            self.do_recursion(i, i + SFMT_POS1 - SFMT_N);
            r1 = r2;
            r2 = self.state[i];
            i += 1;
        }
    }

    fn do_recursion(&mut self, idx1: usize, idx2: usize) {
        let (mut x, mut y) = (W128::zero(), W128::zero());
        self.lshift128(&mut x, SFMT_SL2);
        self.rshift128(&mut y, SFMT_SL2);
        unsafe {
            self.state[idx1].u32t[0] = self.state[idx1].u32t[0]
                ^ x.u32t[0]
                ^ ((self.state[idx2].u32t[0] >> SFMT_SR1) & SFMT_MSK2)
                ^ y.u32t[0]
                ^ (&self.state[self.seek + 1].u32t[0] << SFMT_SL1);
            self.state[idx1].u32t[1] = self.state[idx1].u32t[1]
                ^ x.u32t[1]
                ^ ((self.state[idx2].u32t[1] >> SFMT_SR1) & SFMT_MSK1)
                ^ y.u32t[1]
                ^ (&self.state[self.seek + 1].u32t[1] << SFMT_SL1);
            self.state[idx1].u32t[2] = self.state[idx1].u32t[2]
                ^ x.u32t[2]
                ^ ((self.state[idx2].u32t[2] >> SFMT_SR1) & SFMT_MSK4)
                ^ y.u32t[2]
                ^ (&self.state[self.seek + 1].u32t[2] << SFMT_SL1);
            self.state[idx1].u32t[3] = self.state[idx1].u32t[3]
                ^ x.u32t[3]
                ^ ((self.state[idx2].u32t[3] >> SFMT_SR1) & SFMT_MSK3)
                ^ y.u32t[3]
                ^ (&self.state[self.seek + 1].u32t[3] << SFMT_SL1);
        };
    }

    fn lshift128(&self, out: &mut W128, shift: usize) {
        let (th, tl, oh, ol): (u64, u64, u64, u64);
        unsafe {
            th = (self.state[self.seek + 1].u64t.get_unchecked(0) << 32)
                | self.state[self.seek + 1].u64t.get_unchecked(1);
            tl = (self.state[self.seek].u64t.get_unchecked(0) << 32)
                | self.state[self.seek].u64t.get_unchecked(1);
        };

        oh = (th << (shift * 8)) | (tl >> (64 - shift * 8));
        ol = tl << (shift * 8);
        unsafe {
            out.u32t[0] = (ol >> 32) as u32;
            out.u32t[1] = ol as u32;
            out.u32t[2] = (oh >> 32) as u32;
            out.u32t[3] = oh as u32;
        }
    }

    fn rshift128(&self, out: &mut W128, shift: usize) {
        let (th, tl, oh, ol): (u64, u64, u64, u64);
        unsafe {
            th = (self.state[self.seek + 1].u64t.get_unchecked(1) << 32)
                | self.state[self.seek].u64t.get_unchecked(0);
            tl = (self.state[self.seek].u64t.get_unchecked(1) << 32)
                | self.state[self.seek].u64t.get_unchecked(0);
        };

        oh = (th >> (shift * 8)) | (th << (64 - shift * 8));
        ol = tl >> (shift * 8);
        unsafe {
            out.u32t[0] = (ol >> 32) as u32;
            out.u32t[1] = ol as u32;
            out.u32t[2] = (oh >> 32) as u32;
            out.u32t[3] = oh as u32;
        }
    }

    pub fn generate(&mut self) -> u32 {
        let r: u32;

        if self.idx >= SFMT_N {
            self.gen_rand_all();
            self.idx = 0;
        }
        let psfmt32 = unsafe { &self.state };
        r = unsafe { psfmt32[self.idx].u32t.get_unchecked(0) }.clone();
        self.idx += 1;
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genrand_int32_average100000() {
        let mut sum: u32 = 0;
        // let mut sfmt = Sfmt::new();
        let mut sfmt = Sfmt{
            state: [W128::zero();SFMT_N],
            seek: 0,
            idx: 0,
        };
        // M-series PRNG
        sfmt.state[0] = W128::one();
        let max_count = 1000000;
        let acceptable_delta = u32::MAX / 100;
        for _ in 0..max_count {
            let a = sfmt.generate();
            sum += a / max_count;
        }
        let diff = match sum > u32::MAX / 2 {
            true => sum - u32::MAX / 2,
            false => u32::MAX / 2 - sum,
        };
        assert!(diff < acceptable_delta);
    }
}
