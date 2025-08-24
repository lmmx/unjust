# ✨ `unjust` Project Specification (with Workspace Plan)

## Core Vision

A Rust CLI + TUI (`ratatui`) to **reuse Justfile recipes & variables across projects**, powered by `gh`, `git`, and `just`.
Cached locally in a repo called **`unjustfiles`**, deduped, and refreshable with minimal network overhead.

The tool itself is named **`unjust`**.

---

## Workspace Layout

### Workspace Root

`unjust/` → Cargo workspace root.

```toml
# unjust/Cargo.toml
[workspace]
members = [
    "unjust-cli",
    "unjust-core",
    "unjust-sync",
]
```

---

### Crates

#### 1. `unjust-cli`

* **Purpose**: End-user CLI binary (`unjust`).
* **Responsibilities**:

  * Parse CLI args (with `clap`).
  * Run subcommands (`sync`, `list`, `show`, `add`, `tui`).
  * Render TUI (`ratatui`).
* **Depends on**:

  * `unjust-core` (for cache types + recipe management).
  * `unjust-sync` (for syncing repos).

---

#### 2. `unjust-core`

* **Purpose**: Core library of types + business logic.
* **Responsibilities**:

  * Define types:

    * `Recipe`, `Variable`, `RepoMeta` (JSONL struct), etc.
  * Recipe & variable extraction (`just --dump-format json`).
  * Cache management (read/write recipes + JSONL).
  * Deduplication logic (resolve duplicate names by recency).
* **May be published** later for reuse.

---

#### 3. `unjust-sync`

* **Purpose**: Isolated repo sync engine.
* **Responsibilities**:

  * Manage sparse/cone git clones.
  * Track metadata (`meta.jsonl`).
  * Perform parallel sync with `rayon`.
  * Expose API for `unjust-core` / `unjust-cli`.
* **Optional CLI binary** for testing sync in isolation:

  * `cargo run -p unjust-sync -- --user lmmx --repo unjustfiles sync`.
* **Not published** (internal tooling).

---

## Crate Dependencies (Initial Draft)

### Shared

* `serde`, `serde_json` → JSON parsing (dump + JSONL).
* `thiserror` or `anyhow` → error handling.
* `tokio` OR `rayon` → concurrency (rayon for CPU-ish tasks, tokio for async git commands).

### unjust-cli

* `clap` → CLI arg parsing.
* `ratatui` → TUI rendering.
* `crossterm` → terminal events.
* `indicatif` (optional) → progress indicators (during sync).

### unjust-core

* `serde` / `serde_json`.
* `xxhash-rust` → file hashing (`xxh64`).
* `walkdir` (optional) → file discovery inside cache.

### unjust-sync

* `git2` → libgit2 bindings (for programmatic clone, sparse-checkout).
* Or: spawn `git` via `std::process::Command` (simpler, fewer deps).
* `rayon` → parallel sync.
* `anyhow` for error handling.

---

## Example Flow Across Crates

1. User runs:

   ```bash
   unjust sync --user lmmx --repo unjustfiles
   ```

   * **CLI**: parses args, calls into `unjust-sync`.
   * **Sync**: runs sparse `git fetch/pull`, updates `meta.jsonl`, writes updated Justfile.

2. User runs:

   ```bash
   unjust list --recipes
   ```

   * **CLI**: parses args, calls `unjust-core`.
   * **Core**: loads `Justfile` from cache, runs `just --dump-format json`, dedupes, returns recipe names.

3. User runs TUI:

   ```bash
   unjust tui
   ```

   * **CLI**: launches `ratatui`.
   * **Core**: provides list of recipes + metadata + preview.
   * **Sync** (background): refreshes repos in parallel, updating recency order.

---

## Metadata (JSONL Recap)

Each Justfile has a `meta.jsonl` storing file-level metadata along with recipe-level metadata.
The recipe metadata is the git commit [commit hash] it was seen at, and the recipe's hash
(of its dumped JSON). This would allow us to check if the commit is fresh, and if not then if the
file is fresh, and if not then if the recipe is fresh, so we can tell when something changes:

```json
{"commit": "abc123...", "file_hash": "xxh64:deadbeef...", "timestamp": "2025-08-24T15:00:00Z"}
```

* **Latest entry** = current state.
* **Ordering recipes by recency**: use `timestamp` from JSONL’s last line.
