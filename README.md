# rustgoogle

Example crates for the blog post
[Setting up a Rust crate for Google's style guide](https://noos.blog/posts/google-rust-style-guide-in-a-crate/).
For the background, see
[Google's Rust style guide next to the official one](https://noos.blog/posts/google-rust-style-guide-vs-official/).

Everything here was tested with Rust 1.100.0, plus nightly-2026-11-12 for rustfmt only.

## The crate at the root

The root is `gstyle`, the crate from the post with all the fixes applied:

- `rustfmt.toml`: the options from Google's formatting list, plus `imports_granularity`
- `Cargo.toml`: the `[lints]` table
- `clippy.toml`: `disallowed-methods`, `allow-unwrap-in-tests`, `avoid-breaking-exported-api`
- `.github/workflows/style.yml`: the CI job

```bash
rustup toolchain install 1.100.0 --profile minimal --component clippy
rustup toolchain install nightly-2026-11-12 --profile minimal --component rustfmt

cargo +nightly-2026-11-12 fmt --check
cargo +1.100.0 clippy --all-targets -- -D warnings
cargo +1.100.0 test
```

All three pass. To see an `#[expect]` go stale, add `#[non_exhaustive]` to `Mode` in
`src/lib.rs` and run the Clippy command again. It fails with
"this lint expectation is unfulfilled".

## experiments/

Each folder is its own small crate. Run this inside it unless the table says otherwise:

```bash
cargo +1.100.0 clippy --all-targets --message-format=short
```

| Folder | What it tries | What I got on 1.100.0 |
|---|---|---|
| `before` | The rule-breaking version, with the same config files as the root | 12 warnings; exits with 101 when you add `-- -D warnings` |
| `fmt-sample` | A file left unformatted on purpose. Run `cargo +nightly-2026-11-12 fmt --check` | Nightly splits the merged `use`, lowercases `0xFF` and tidies `$x : expr`; stable 1.100.0 only fixes the hex literal |
| `named-lints` | The seven Clippy lints Google's guide names | 5 warnings with defaults. Set `avoid-breaking-exported-api = false` in its `clippy.toml` and `wrong_self_convention` makes 6 |
| `lint-edges` | `string_add` on just two strings, `use super::*` in a non-test module | Both flagged |
| `invisible` | A zero-width joiner and a zero-width space in string literals; 3 and 4 `bool` parameters | U+200B is an error, U+200D passes; `fn_params_excessive_bools` fires at 4 |
| `lazy-messages` | `.expect("failed")`, an assert message that repeats the condition, `// SAFETY: trust me` | No warnings |
| `omitted-patterns` | `non_exhaustive_omitted_patterns` on stable | "unknown lint": the lint is unstable |

`invisible/src/lib.rs` contains invisible characters on purpose, so GitHub shows a
hidden-character warning for it.

## Notes

- Each experiment has its own `clippy.toml`, because Clippy also looks in parent
  directories and would otherwise pick up the root one. If you add a `clippy.toml`
  where there was none, run `cargo clean` first. Cargo replayed old results for me.
- Tested on macOS only.

## License

MIT. See [LICENSE](LICENSE).
