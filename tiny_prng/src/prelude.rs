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