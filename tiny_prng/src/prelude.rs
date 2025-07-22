#[macro_export]
macro_rules! generate_real64 {
    () => {
         pub fn generate_real(&mut self) -> f64 {
            let r = self.generate();
            let upper = (r>>32) as f64;
            let lower = (r&0xfffffffc) as f64; // 53bit
            upper * (1.0/(4294967295.0)) + lower * (1.0/(18446744073709551615.0))
        }

         pub fn generate_real_in_range(&mut self, lower: f64, upper: f64) -> f64 {
            let v = self.generate_real();
            lower + (upper - lower) * v
        }

        pub fn generate_real_closed(&mut self) -> f64{
            let r = self.generate();
            let upper = (r>>32) as f64;
            let lower = (r&0xfffffffc) as f64; // 53bit
            upper * (1.0/(4294967296.0)) + lower * (1.0/(18446744073709551616.0))
        }
    }
}

#[macro_export]
macro_rules! generate_real32 {
    () => {
    pub fn generate_real(&mut self) -> f64 {
        (self.generate() as f64) *(1.0/4294967295.0)
    }

    pub fn generate_real_in_range(&mut self, lower: f64, upper: f64) -> f64 {
        let v = self.generate_real();
        lower + (upper - lower) * v
    }

    pub fn generate_real_closed(&mut self) -> f64{
            (self.generate() as f64) *(1.0/4294967296.0)
        }
    }
}

#[macro_export]
macro_rules! generate_real128 {
    () => {
        pub fn generate_real(&mut self) -> f64 {
            let r = self.generate();
            let upper1 = (r >> 96) as f64;
            let upper2 = ((r >> 64) & 0xfffffffc) as f64;
            upper1 * (1.0 / (4294967295.0)) + upper2 * (1.0 / (18446744073709551615.0))
        }

        pub fn generate_real_in_range(&mut self, lower: f64, upper: f64) -> f64 {
            let v = self.generate_real();
            lower + (upper - lower) * v
        }

        pub fn generate_real_closed(&mut self) -> f64 {
            let r = self.generate();
            let upper1 = (r >> 96) as f64;
            let upper2 = ((r >> 64) & 0xfffffffc) as f64; // 53bit
            upper1 * (1.0 / (4294967296.0)) + upper2 * (1.0 / (18446744073709551616.0))
        }
    }
}

#[macro_export]
macro_rules! gen_delta_rate {
    () => {
        100
    };
}

#[macro_export]
macro_rules! generate_unit_test {
    ($constructor:ident, $name:ident, $primitive:ident, $seed:literal, $count:ident) => {
        #[test]
        fn $name() {
            let mut s = $constructor::with_seed($seed);
            let mut sum = 0;
            let acceptable_delta = $primitive::MAX / (gen_delta_rate!() as $primitive);
            for _ in 0..$count {
                sum += s.generate() / $count as $primitive;
            }
            let real_delta = match sum > $primitive::MAX / 2 {
                true => sum - $primitive::MAX / 2,
                false => $primitive::MAX / 2 - sum,
            };
            assert!(real_delta < acceptable_delta);
        }

    };
}


#[macro_export]
macro_rules! generate_unit_test_real1 {
    ($constructor:ident, $name:ident, $primitive:ident, $seed:literal, $count:ident) => {
        #[test]
        fn $name() {
            let mut p = $constructor::with_seed($seed);
            let mut sum = 0.0;
            let max_count = 100 * $count;
            let acceptable_delta = 1.0 / (gen_delta_rate!() as $primitive);
            for _ in 0..max_count {
                sum += p.generate_real() / max_count as $primitive;
            }
            let diff = match sum > 0.5 {
                true => sum - 0.5,
                false => 0.5 - sum,
            };
            assert_eq!(true, diff < acceptable_delta);
        }
    };
}

#[macro_export]
macro_rules! generate_unit_test_real2 {
    ($constructor:ident, $name:ident, $primitive:ident, $seed:literal, $count:ident) => {
        #[test]
        fn $name() {
            let mut p = $constructor::with_seed($seed);
            let mut sum = 0.0;
            let max_count = $count;
            let acceptable_delta = 1.0 / (gen_delta_rate!() as $primitive);
            for _ in 0..max_count {
                sum += p.generate_real_closed() / max_count as $primitive;
            }
            let diff = match sum > 0.5 {
                true => sum - 0.5,
                false => 0.5 - sum,
            };
            assert_eq!(true, diff < acceptable_delta);
        }
    };
}

#[macro_export]
macro_rules! generate_unit_test_real_ranged {
    ($constructor:ident, $name:ident, $primitive:ident, $seed:literal, $count:ident) => {
        #[test]
        fn $name() {
            let mut p = $constructor::with_seed($seed);
            let mut sum = 0.0;
            let max_count = $count;
            let acceptable_delta = 2000.0 / (gen_delta_rate!() as $primitive);
            for _ in 0..max_count {
                sum += p.generate_real_in_range(-1000.0, 1000.0) ;
            }
            let diff = match sum > 0.0 {
                true => sum / max_count as $primitive,
                false => -(sum / max_count as $primitive),
            };
            assert_eq!(true, diff < acceptable_delta);
        }
    };
}