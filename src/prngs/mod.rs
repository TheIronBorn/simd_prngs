mod ars;
pub use self::ars::*;

mod sfc;
pub use self::sfc::*;

#[cfg(feature = "candidate_rngs")]
pub mod sfc_alt;
// too many items to list in documentation
#[doc(hidden)]
#[cfg(feature = "candidate_rngs")]
pub use sfc_alt::*;

#[cfg(feature = "candidate_rngs")]
pub mod vf;
// too many items to list in documentation
#[doc(hidden)]
#[cfg(feature = "candidate_rngs")]
pub use vf::*;

mod jsf;
pub use self::jsf::*;

mod xorshift;
pub use self::xorshift::*;

mod xorshift128plus;
pub use self::xorshift128plus::*;

mod xoroshiro;
pub use self::xoroshiro::*;

mod xoshiro;
pub use self::xoshiro::*;

mod lcg;
pub use self::lcg::*;

mod pcg;
pub use self::pcg::*;

mod pcg_fixed;
pub use self::pcg_fixed::*;

mod lfsr;
pub use self::lfsr::*;

mod mwc;
pub use self::mwc::*;

mod xsm;
pub use self::xsm::*;

mod intel_lcg;
pub use self::intel_lcg::*;

mod chacha;
pub use self::chacha::*;

mod aes_rand;
pub use self::aes_rand::*;
