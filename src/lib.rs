//! # handl-lib-kv
//!
//! Key-value store abstraction for the handl runtime.
//!
//! Defines the [`KeyValueStore`] trait and [`KvError`]. The `kv.*` ops are
//! declared in `handl-packages/kv/kv.hnl` and implemented in [`ops`] as
//! [`handl_ops::Op`]s behind [`kv_op_registry`] — a kernel
//! [`handl_ops::OpRegistry`] the host imports via
//! `handl_plugin_lib_handl::OpRegistry::import_kernel`.
//!
//! Unlike platform services (filesystem, env, http), a key-value store is a
//! stateful data abstraction — not tied to any OS capability. Backends may
//! include in-memory HashMaps, sled, SQLite, Redis, etc.
//!
//! - `handl-plugin-lib-kv-hashmap` — in-memory `HashMap` backend
//!
//! This crate has been migrated to the `op`/`handl` declaration form
//! (ADR-0077 Phase 4) and to the kernel op ABI (`handl-ops`): it carries no
//! host (`handl-platform`/`handl-plugin-*`) dependencies. Module/plugin
//! wrappers live in consumer repos.

#![allow(clippy::module_inception)]

mod error;
mod key_value_store;
pub mod ops;

pub use error::KvError;
pub use key_value_store::KeyValueStore;
pub use ops::{KV_OPS, kv_op_registry};
