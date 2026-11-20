use crate::error::KvError;

/// Abstract key-value store service.
///
/// Implementations:
/// - `handl-plugin-lib-kv-hashmap`'s `HashKeyValueStore` — in-memory `HashMap`
/// - Future backends: sled, SQLite, Redis, etc.
///
/// The `kv.*` ops are implemented in [`crate::ops`] and registered through
/// [`crate::kv_op_registry`]; the host imports the kernel
/// [`handl_ops::OpRegistry`] at module-build time.
pub trait KeyValueStore: Send + Sync {
    fn get(&self, key: &str) -> Result<Option<String>, KvError>;
    fn set(&self, key: &str, value: &str) -> Result<(), KvError>;
    fn remove(&self, key: &str) -> Result<(), KvError>;
    fn clear(&self) -> Result<(), KvError>;
    fn has(&self, key: &str) -> Result<bool, KvError>;
    fn size(&self) -> Result<usize, KvError>;
}
