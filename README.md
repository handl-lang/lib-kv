# lib-kv

`handl-plugin-lib-kv` — the key-value store abstraction for the handl
runtime: the `kv.*` function surface plus the Rust trait backends
implement.

A key-value store is a stateful data abstraction, not an OS capability —
unlike the platform services (filesystem, env, http) it is not tied to
any host feature, so backends range from in-memory `HashMap` to sled,
SQLite, or Redis. The interface is declared once in
`handl-packages/kv/kv.hnl` (`cap` declarations, ADR-0077) and the Rust
side keeps only the trait + module adapter; concrete backends live in
sibling crates (e.g. `handl-lang/lib-kv-hashmap`'s `HashKeyValueStore`).

The `kv.*` surface callable from handl expressions:

| op | signature | semantics |
|---|---|---|
| `kv.get` | `key -> value?` | retrieve a value by key (`null` when absent) |
| `kv.set` | `(key, value)` | set a key-value pair |
| `kv.remove` | `key` | remove a key |
| `kv.clear` | `()` | clear all entries |
| `kv.has` | `key -> bool` | key-exists check |
| `kv.size` | `() -> int` | number of entries |

## Rust API

- `KeyValueStore` — the backend trait (`get`/`set`/`remove`/`clear`/
  `has`/`size` over `&str` keys and values, `KvError` on failure).
- `KeyValueStoreModule` — the `Module` adapter: wraps any
  `KeyValueStore` impl in a `HandlModule` built from `kv.hnl`, so the
  `kv.*` ops dispatch to the store.
- `KvLibraryPlugin` — the `Library` plugin form (wraps the generic
  `HandlLibraryPlugin`; plugin id `lib.kv`).

```rust
use std::sync::Arc;
use handl_plugin_lib_kv::{KeyValueStoreModule, KvLibraryPlugin};
use handl_plugin_lib_kv_hashmap::HashKeyValueStore;

let module = KeyValueStoreModule::new(HashKeyValueStore::default());
// register `module` in the runtime's ModuleRegistry / ModuleRuntime —
// `kv.get("k")` now reaches the store.
```

## Layout

```text
handl-packages/kv/kv.hnl   the interface — cap declarations (single source of truth)
src/lib.rs                 KvLibraryPlugin + re-exports
src/key_value_store.rs     KeyValueStore trait, KeyValueStoreModule, the kv.* op registry
src/error.rs               KvError
examples/kv/main.hnl       a runnable .hnl example
scripts/hnl-gate.sh        the shared hnl typecheck/lint/test gate
ci.sh                      bupha entry — delegates to hnl-gate.sh
```

Sibling-repo deps are git dependencies (`branch = "main"`); `[patch]`
tables resolve them from sibling checkouts.

## Gate

```bash
cargo test        # the Rust suite (plugin id/capabilities)
bash ci.sh        # the bupha gate — hnl typecheck/lint/test over .hnl
                  # sources (the kv.hnl surface is arrow-dialect, so the
                  # canonical detector skips it by design)
```

## License

MIT.
