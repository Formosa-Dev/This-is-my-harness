//! `harness-core` — placeholder.
//!
//! Reserved for the Harness Core surface: reference resolution, planning,
//! snapshot/apply/verify/revert, path-safety and hashing (F4 and later).
//! It intentionally exposes **no** API yet.
//!
//! This crate exists in F2 only to lock the workspace layout and the
//! dependency direction fixed by ADR-0001:
//!
//! ```text
//! harness (bin) --> harness-validator
//! harness-core  --> harness-validator   (future; no cycles)
//! ```
//!
//! The JSON Schema set in `schemas/` stays language-neutral and remains the
//! single source of truth; nothing here redefines it.

#![forbid(unsafe_code)]
