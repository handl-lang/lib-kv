# lib-kv

`handl-plugin-lib-kv` — the key-value store abstraction for the handl
runtime: the `kv.*` function surface plus the Rust trait backends
implement. A KV store is a stateful data abstraction, not an OS
capability — backends range from in-memory `HashMap` to sled, SQLite,
or Redis.

## Layout

| path | role |
| --- | --- |
| `handl-packages/kv/kv.hnl` | the `.hnl` surface — `cap` declarations for `kv.*` |
| `src/lib.rs`, `src/key_value_store.rs`, `src/error.rs`, `src/ops.rs` | the `KeyValueStore` trait + `KeyValueStoreModule` adapter |
| `examples/` | usage examples |
| `Cargo.toml`, `xtask/` | Rust crate + gate |
| `pkg.hnl` | manifest — `modules: ["kv"]` |
| `scripts/hnl-gate.sh`, `workflows/gate.yaml` | `.hnl` gate + CI campaign |

## Contract

- Interface declared once in `kv.hnl`; the Rust side keeps only the
  trait + module adapter — concrete backends live in sibling crates
  (e.g. `lib-kv-hashmap`).
- Distinct from `lib-kv-map` (pure functional record-value map).

## Verification

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
cargo xtask gate
```

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
