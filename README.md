# em_betareduction

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Emergence agent — applies beta reduction to an [AlephTree](https://github.com/aleph-lang/aleph-syntax-tree) intermediate representation.

This agent is the Emergence network integration of the [`betareduction`](https://crates.io/crates/betareduction) aleph-lang crate.

---

## Role in the Aleph pipeline

```
AlephTree (JSON)
      ↓  em_betareduction  (this agent)
AlephTree optimised (JSON)
```

Transformer agents sit between parsers and generators in the Aleph pipeline.

---

## Query protocol

**Input** (`body`): JSON-serialized `aleph_tree` embryo

**Output**: JSON array with one transformed `aleph_tree` embryo:
```json
[{
  "type": "aleph_tree",
  "properties": {
    "source_language": "python",
    "tree": { "..." : "..." }
  }
}]
```

---

## Capabilities

```rust
vec!["transform", "betareduction"]
```

---

## Deployment

```bash
git clone https://github.com/EmergenceSystem/em_betareduction
cd em_betareduction
cargo build --release
./target/release/em_betareduction
```

Configure via environment variables:
- `EM_DISCO_NODES` — comma-separated `host:port` list (default: `localhost:8080`)
- `EM_FILTER_JWT` — optional JWT token
- `EM_FILTER_RECONNECT_MS` — reconnect delay in ms (default: 5000)
