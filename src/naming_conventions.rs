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
//!
//! ## Operations
//! There's many operations that can be performed. When possible, `safe_arch`
//! tries to follow normal Rust naming, but if an operation doesn't normally
//! exist at all in Rust then we just go with a best effort and try to stay as
//! consistent as possible about it.
//!
//! Many operations have more than one variant, such as `add` and
//! `add_saturating`. In this case, we choose to put the "core operation" first
//! and then any modifiers go after, which isn't how you might normally say it
//! in English, but it makes the list of functions sort better.
//!
//! ## Operation Glossary
//! Here follows the list of all the operations and their explanations.
//!
//! * TODO
