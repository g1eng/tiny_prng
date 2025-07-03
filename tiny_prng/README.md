# Tiny PRNG

[![codecov](https://codecov.io/gh/g1eng/tiny_prng/graph/badge.svg)](https://codecov.io/gh/g1eng/tiny_prng)

> [!NOTE]
> Beta release. The quality of generated pseudo random numbers is not tested at now.

This crate provides common psuedo random number generators written in pure Rust, which include:

| name                                 | supported mode                                                                              | period                                                                                                      | reference                                                                                                                                                                                                                    |
|--------------------------------------|---------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Mersenne Twister                     | `MT19937` `MT19937_64`                                                                      | 2<sup>19937</sup>-1                                                                                         | [Saitoh and Matsumoto (1997)](https://www.math.sci.hiroshima-u.ac.jp/m-mat/MT/MT2002/emt19937ar.html)                                                                                                                        |
| Xorshift                             | `xorshift32` <br/>`xorshift64`<br/>`xorshift128`<br/>`xorshift64*`<br/>`xorshift1024*`<br/> | 2<sup>64</sup>-1 <br/>2<sup>64</sup>-1 <br/>2<sup>128</sup>-1 <br/>2<sup>64</sup>-1 <br/>2<sup>1024</sup>-1 | [Marsaglia (2003), J. Stat. Softw. 8 (14)](https://www.jstatsoft.org/index.php/jss/article/view/v008i14/916)<br/> [Vigna (2016), ACM Trans. Math. Softw. Vol. 42 (4), 30](https://vigna.di.unimi.it/ftp/papers/xorshift.pdf) |
| PCG (with LCG)                       | `PCG-XSL-RR-128/64` <br/>`PCG-XSH-RS-64/32` <br/>`PCG-XSH-RR-64/32`                         | 2<sup>128</sup> <br/> 2<sup>64</sup> <br/> 2<sup>64</sup>                                                   | [O'Neil (2014), HMC-CS-2014-0905](https://www.pcg-random.org/pdf/hmc-cs-2014-0905.pdf)<br/>[Reference implementation](https://github.com/imneme/pcg-c-basic)                                                                 |

There are deprecated implementations contained in the package:

* SIMD-oriented Fast Mersenne Twister (experimental and not correctly implemented)

## Install

```
cargo add tiny_prng
```

## Usage

* prepare a seed with a certain way
* construct a generator
* generate pseudo random number with `generate()`

```rust
use tiny_prng::xorshift::Xorshift64;
use std::time::SystemTime;

const MODV: u128 = 19937 * 273;
const MODS: usize = 11;

fn main(){
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
    /// Note: That's good to refer additional source(s) to calculate the seed
    let seed = ((now % MODV) << MODS) + now;
    /// A generator must be a mutable because its internal state alters at the random number generation.
    let mut x = Xorshift64::with_seed(seed as u64);
    println!("{} {} {}", x.generate(),x.generate(),x.generate());
}
```


# WASM support

You can use `tiny_prng` in WASM. Two types of usage patterns are supported.

* Link the crate and build the code with `wasm32-wasip2` target (and run it with wasmtime and so on.)
* Call WASM functions from JavaScript code

For web developers who need to generate many pseudo random numbers, we also provide the npm package `tiny-prng-wasm`.
Install it as follows:

```shell
npm install tiny-prng-wasm
```

In the npm package, three PRNGs (and one mode for each) are supported:

* `pcg` (PCG-XSL-RR-128/64) 
* `mt64` (MT19937_64)
* `xorshift64` (Xorshift64)

You can specify `seed` and `count` of pseudo random numbers to be generated.

# Benchmarking

## Library Routines

Any core routines in the library can generate a pseudo random number within 100 milliseconds with consumer grade 64-bit computers (e.g. low-end smartphones with ARMv9 chipset, laptop PCs with Tiger Lake Generation Celeron, etc.),
although you need to instantiate the generator with seed before it works.

See the benchmarking result for 10 million instructions of pseudo random number generation:

```shell-session
user@localhost tiny_prng $  cargo bench  | grep -v ignored
# (output omitted...)
running 34 tests
test mt64::tests::bench_mt19937_10mil          ... bench:  26,298,141.70 ns/iter (+/- 2,349,687.12)
test mt::tests::bench_mt19937_32_10mil         ... bench:  28,526,208.40 ns/iter (+/- 345,140.73)
test pcg::tests::bench_pcgxshrr6432_10mil      ... bench:   9,377,162.50 ns/iter (+/- 17,100.03)
test pcg::tests::bench_pcgxslrr12864_10mil     ... bench:  15,629,216.70 ns/iter (+/- 836,248.20)
test xorshift::tests::bench_xorshift1024_10mil ... bench:  21,973,179.10 ns/iter (+/- 875,331.16)
test xorshift::tests::bench_xorshift64_10mil   ... bench:  18,755,408.30 ns/iter (+/- 51,057.99)
```

Execution environment:

* OS: macOS Sequoia 15.5 
* CPU: arm64 (Apple M1)
* Memory: 8GB

> [!NOTE]
> This result is measured with benchmark tests in the library.
> We are planning further performance evaluations and investigations in the future, in more different execution environments with variety of benchmarking conditions.


## Frontend Benchmarking

As the simplest web benchmarking environment, you can try the online benchmarking in your browser.

Simply run `make` under the `wasm_web` directory and open http://localhost:8080.

# Author 

Suzume Nomura <SuzuMe[att]ea.g1e．org>