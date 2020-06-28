#![cfg(target_feature = "aes")]

use super::*;

/// "Perform one round of AES decryption flow on `a` using the `round_key`."
/// ```
/// # use safe_arch::*;
/// // TODO
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "aes")))]
pub fn aes_decrypt_m128i(a: m128i, round_key: m128i) -> m128i {
  m128i(unsafe { _mm_aesdec_si128(a.0, round_key.0) })
}

/// "Perform the last round of AES decryption flow on `a` using the
/// `round_key`."
/// ```
/// # use safe_arch::*;
/// // TODO
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "aes")))]
pub fn aes_decrypt_last_m128i(a: m128i, round_key: m128i) -> m128i {
  m128i(unsafe { _mm_aesdeclast_si128(a.0, round_key.0) })
}

/// "Perform one round of AES encryption flow on `a` using the `round_key`."
/// ```
/// # use safe_arch::*;
/// // TODO
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "aes")))]
pub fn aes_encrypt_m128i(a: m128i, round_key: m128i) -> m128i {
  m128i(unsafe { _mm_aesenc_si128(a.0, round_key.0) })
}

/// "Perform the last round of AES encryption flow on `a` using the
/// `round_key`."
/// ```
/// # use safe_arch::*;
/// // TODO
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "aes")))]
pub fn aes_encrypt_last_m128i(a: m128i, round_key: m128i) -> m128i {
  m128i(unsafe { _mm_aesenclast_si128(a.0, round_key.0) })
}

/// "Perform the InvMixColumns transform on `a`."
/// ```
/// # use safe_arch::*;
/// // TODO
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "aes")))]
pub fn aes_inv_mix_columns_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_aesimc_si128(a.0) })
}

/// ?
/// ```
/// # use safe_arch::*;
/// // TODO
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "aes")))]
macro_rules! aes_key_gen_assist_m128i {
  ($a:expr, $imm:expr) => {{
    let a: $crate::m128i = $a;
    const IMM: ::core::primitive::i32 =
      ($imm & 0b1111_1111) as ::core::primitive::i32;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm_aeskeygenassist_si128;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm_aeskeygenassist_si128;
    m128i(unsafe { _mm_aeskeygenassist_si128(a.0, IMM) })
  }};
}
