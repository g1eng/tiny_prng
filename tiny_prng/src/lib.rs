//! [!NOTE]
//! Beta release. The quality of generated pseudo random numbers is not tested at now.
//! 
//! This crate provides common psuedo random number generators written in pure Rust, which include:
//! 
//! 
//! | name                                 | supported mode                                                                              | cycle period                                                                                                | reference                                                                                                                                                                                                                    |
//! |--------------------------------------|---------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
//! | Mersenne Twister                     | `MT19937` `MT19937_64`                                                                      | 2<sup>19937</sup>-1                                                                                         | [Saitoh and Matsumoto (1997)](https://www.math.sci.hiroshima-u.ac.jp/m-mat/MT/MT2002/emt19937ar.html)                                                                                                                        |
//! | Xorshift                             | `xorshift32` <br/>`xorshift64`<br/>`xorshift128`<br/>`xorshift64*`<br/>`xorshift1024*`<br/> | 2<sup>64</sup>-1 <br/>2<sup>64</sup>-1 <br/>2<sup>128</sup>-1 <br/>2<sup>64</sup>-1 <br/>2<sup>1024</sup>-1 | [Marsaglia (2003), J. Stat. Softw. 8 (14)](https://www.jstatsoft.org/index.php/jss/article/view/v008i14/916)<br/> [Vigna (2016), ACM Trans. Math. Softw. Vol. 42 (4), 30](https://vigna.di.unimi.it/ftp/papers/xorshift.pdf) |
//! | PCG (with LCG)                       | `PCG-XSL-RR-128/64` <br/>`PCG-XSH-RS-64/32` <br/>`PCG-XSH-RR-64/32`                         | 2<sup>128</sup> <br/> 2<sup>64</sup> <br/> 2<sup>64</sup>                                                   | [O'Neil (2014), HMC-CS-2014-0905](https://www.pcg-random.org/pdf/hmc-cs-2014-0905.pdf)<br/>[Reference implementation](https://github.com/imneme/pcg-c-basic)                                                                 |

#![feature(test)]

extern crate test;

pub mod mt;
pub mod mt64;
pub mod xorshift;
pub mod pcg;
mod prelude;
