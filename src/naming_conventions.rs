//! An explanation of the crate's naming conventions.
//!
//! This crate attempts to follow the general naming scheme of `verb_type` when
//! the operation is "simple", and `verb_description_words_type` when the
//! operation (op) needs to be more specific than normal. Like this:
//! * `add_m128`
//! * `add_saturating_i8_m128i`
//!
//! ## Types
//! Currently, only `x86` and `x86_64` types are supported. Among those types:
//! * `m128` and `m256` are always considered to hold `f32` lanes.
//! * `m128d` and `m256d` are always considered to hold `f64` lanes.
//! * `m128i` and `m256i` hold integer data, but each op specifies what lane
//!   width of integers the operation uses.
//! * If the type has `_s` on the end then it's a "scalar" operation that
//!   affects just the lowest lane. The other lanes are generally copied forward
//!   from one of the inputs, though the details there vary from op to op.
//! * The SIMD types are often referred to as "registers" because each SIMD
//!   typed value represents exactly one CPU register when you're doing work.
//!
//! ## Operations
//! There's many operations that can be performed. When possible, `safe_arch`
//! tries to follow normal Rust naming (eg: adding is still `add` and left
//! shifting is still `shl`), but if an operation doesn't normally exist at all
//! in Rust then we basically have to make something up.
//!
//! Many operations have more than one variant, such as `add` and also
//! `add_saturating`. In this case, `safe_arch` puts the "core operation" first
//! and then any "modifiers" go after, which isn't how you might normally say it
//! in English, but it makes the list of functions sort better.
//!
//! As a general note on SIMD terminology: When an operation uses the same
//! indexed lane in two _different_ registers to determine the output, that is a
//! "vertical" operation. When an operation uses more than one lane in the
//! _same_ register to determine the output, that is a "horizontal" operation.
//! * Vertical: `out[0] = a[0] + b[0]`, `out[1] = a[1] + b[1]`
//! * Horizontal: `out[0] = a[0] + a[1]`, `out[1] = b[0] + b[1]`
//!
//! ## Operation Glossary
//! Here follows the list of all the operations and their explanations.
//!
//! * `abs`: Absolute value (wrapping).
//! * `add`: Addition (wrapping by default).
//! * `and`: Bitwise `&`
//! * `andnot`: Bitwise `(!a) & b`
//! * `average`: Averages the two inputs.
//! * `blend`: TODO
//! * `cast`: Convert between data types while preserving the exact bit
//!   patterns, like how [`transmute`](core::mem::transmute) works.
//! * `ceil`: "Ceiling", rounds towards positive infinity.
//! * `cmp`: Numeric comparisons of various kinds. This generally gives "mask"
//!   output where the output value is of the same data type as the inputs, but
//!   with all the bits in a "true" lane as 1 and all the bits in a "false" lane
//!   as 0. Remember that with floating point values all 1s bits is a `NaN`, and
//!   with signed integers all 1s bits is `-1`.
