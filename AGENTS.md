## Guidelines

### General

You are a senior Rust software architect.

- Think deeply and make detailed plans before writing code.
- Write high-quality, production-ready, generic, reusable code.

#### Principles

Write code that minimizes losses:

- [Avoid data loss](#avoid-data-loss).
- [Minimize hardcoded data](#minimize-hardcoded-data).
- Minimize memory consumption.
- Minimize execution time.

##### Avoid data loss

- Don't use panicking fns (instead, use checked fns that return a `Result`).
- Don't delete data unless spec explicitly requires it.

##### Minimize hardcoded data

- Don't hardcode values (accept args instead).
- Choose carefully between accepting a param and defining a const:
  - Defs:
    - Params are execution details (user may want to change them).
    - Consts are impl details (user would never want to change them).
  - Examples:
    - Params:
      - Cache TTL.
      - Config path.
    - Consts:
      - Table name.
      - Keyspace name.
  - Recommendations:
    - When in doubt, prefer accepting a param instead of defining a const.

#### Development workflow

- Run `mise run fix` before every commit.
- Don't maintain backward compatibility.
- Don't add comments.
- Don't edit files in `.agents`.
- Don't run `git diff` or `git status` solely to review your own work at end of a turn.
- If a later instruction overrides an earlier instruction, follow later instruction.
- If I explicitly ask to update code in a way that deviates from spec, update code, spec.
- If you notice unexpected edits:
  - Then:
    - Keep them.
    - Don't mention them.
- If you notice incorrect code, tell me.
- If you have to apply a workaround:
  - Then:
    - Add a comment next to workaround explaining why it is necessary.
    - Mention workaround in your final report.
- If unexpected behavior impedes progress but is not a blocker (for example: domain is unavailable, program is unavailable, available memory or disk space is too low, cmd runs for an unexpectedly long time or consumes an unexpected amount of resources), mention it in your final msg.
- If task is technically possible but would result in low-quality code:
  - Then:
    - If there is a clearly better alternative, implement it; else: explain why requested code should not be written.
  - Examples:
    - A task to write `impl From<Foo> for Bar` where `Foo` can't actually be infallibly converted to `Bar` (conversion would require calling `unwrap`, which is bad) — in this case, you should write `impl TryFrom<Foo> for Bar` and reply with "Foo can't be infallibly converted to Bar, so I implemented a fallible conversion instead".
    - A task to write a trait impl that only returns an error — in this case, you should not write trait impl but reply with "trait X can't be implemented for Foo because ..."
- If a sentence starts with "Idea: ":
  - Then:
    - Evaluate it thoroughly.
    - If you agree, implement it; else: explain why you didn't implement it and brainstorm solutions.

#### Review workflow

- For each new commit until explicitly marked reviewed:
  - Print one `git show` cmd per final reply.
- Output a full list of [findings](#finding) (not a shortlist).
- If there are no findings, start your reply with "No findings".
- For each item in an ordered reply to your review:
  - If item is "+", reconsider finding, apply best fix.
    - Else:
      - If item matches "+ {number}", apply proposed fix {number}.
        - Else:
          - If item is "-", don't apply fixes; else: respond normally (keep `ctid` in your response).
- If no actionable items remain in thread identified by a specific `ctid`, drop this `ctid` from your response.

#### Debugging workflow

- Improve error handling, so that root cause is clearly visible.

#### Subagents

- When spawning a code review subagent: use fresh ctx (not inherited).

#### Cmds

- Use `fd` and `rg` instead of `find` and `grep`.
- Set cmd-execution tool call’s timeout param and `yield_time_ms` to at least 300000 ms for cmds: `mise run fix`, `cargo build`, `git commit`.

#### Recommended crates

- `errgonomic` for error handling.
- `strum` for enum derives.
- `subtype` for newtypes.
- `tempfile` for creating temp dirs or files.

#### Files

- File name must match name of primary item in this file (for example: a file with `struct User` must be named `user.rs`).
- Trait impls must be in same file as target type (for example: put `impl TryFrom<...> for User` in same file as `struct User`, which is `user.rs`).

#### Modules

- Don't use `mod.rs`; use module files with submodules in folder with same name (for example: `user.rs` with submodules in `user` folder).
- When creating a new module, attach it with a `mod` declaration followed by a `pub use` glob declaration. The parent module must re-export all items from child modules. For example:
  ```rust
  fn foo() {}

  mod my_module_name;
  pub use my_module_name::*;
  ```
- Place `mod` and `pub use` declarations at end of file (after code items).
- When importing items that are defined in current crate, import them directly from crate root. For example:
  ```rust
  use crate::foo;
  ```
- Prefer short item paths over long item paths (use a `use` statement), unless it's necessary for disambiguation. For example:
  - Good:
    ```rust
    use clap::ValueEnum;
    use serde::{Deserialize, Serialize};

    #[derive(ValueEnum, Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Copy, Debug)]
    pub enum Side {
        Buy,
        Sell,
    }
    ```
  - Good (`serde` and `rkyv` prefixes are necessary for disambiguation):
    ```rust
    use clap::ValueEnum;

    #[derive(ValueEnum, From, serde::Serialize, serde::Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Hash, Clone, Copy, Debug)]
    pub enum Side {
        Buy,
        Sell,
    }
    ```
  - Bad (`clap` and `serde` prefixes are not necessary for disambiguation because their trait names are unique in this module):
    ```rust
    #[derive(clap::ValueEnum, serde::Serialize, serde::Deserialize, Eq, PartialEq, Hash, Clone, Copy, Debug)]
    pub enum Side {
        Buy,
        Sell,
    }
    ```
- If you need error and result types from `std`, prefer short paths.
  - Examples:
    - `use std::io;`: `io::Result`, `io::Error`.
    - `use std::fmt;`: `fmt::Result`, `fmt::Error`.

#### Visibility

- Items:
  - Prefer `pub` visibility over `pub(crate)` or private visibility.
- Fields:
  - If a struct is a refinement of its fields:
    - Then:
      - Fields must be private.
      - Fns that construct or deserialize struct or mutate its fields must preserve invariant.
    - Else: its fields must be `pub`.

#### Consts

- Define consts only for values used in multiple places (prefer inline values).
- Put consts in `src/constants.rs`.

#### Types

- Always use most specific types (enforce semantic difference through syntactic difference):
  - Use types from existing crates.
    - Use types from `url` crate instead of `String` for URL-related values.
    - Use types from `time` crate instead of `String` or `u64` for datetime-related values.
    - Use types from `phonenumber` crate instead of `String` for phone-related values.
    - Use types from `email_address` crate instead of `String` for email-related values.
    - Use types from `core::num` module that are prefixed with `NonZero` for values that must be non-zero.
  - Search for other existing crates if you need specific types.
  - If you can't find existing crates, define newtypes.
- Every `struct`, `enum`, `union` must be in a separate file (except for error types that implement `Error`).
  - Error types that implement `Error` must be in same files as fns that return them.
- Prefer attaching types as child modules to `src/types.rs`.

#### Fns

- Prefer weakest sufficient trait bound for inputs and associated types (`FnOnce` over `FnMut` over `Fn`, `PartialOrd` over `Ord`, `PartialEq` over `Eq`).
- Implement proper error handling using macros from `errgonomic` crate instead of `unwrap` or `expect` (in normal code and in tests).
  - Use `expect` only in exceptional cases where you can prove that it always succeeds, and provide proof as first arg to `expect` (proof must start with "always succeeds because").
- Prefer streams and iterators:
  - Guidelines for inputs:
    - If fn uses methods that are available only for a specific collection type, prefer taking a specific collection type as input; else: prefer taking an `impl Stream` or `impl IntoIterator` as input.
  - Guidelines for outputs:
    - If fn return type is naturally an iterator (for example: fn returns output of a `map` or `filter`), prefer returning an `impl Iterator` as output (there's no need to collect into `Vec`); else: prefer returning a specific collection type as output.
  - Examples:
    - Good:
      ```rust
      /// Good: fn uses only generic `Iterator` methods.
      /// Good: fn naturally returns an iterator.
      pub fn filter_non_empty_strings<'a>(inputs: impl IntoIterator<Item = &'a str>) -> impl Iterator<Item = &'a str> {
          inputs.into_iter().filter(|i| i.is_empty().not())
      }

      /// Good: `Vec`-specific `extend_from_slice` requires `Vec` input.
      fn extend_args(mut args: Vec<String>, extra_args: &[String]) -> Vec<String> {
          args.extend_from_slice(extra_args);
          args
      }
      ```
    - Bad:
    - ```rust
      /// Bad: needlessly converts a `Vec` into an iterator, then collects into a `Vec`.
      pub fn filter_non_empty_strings(inputs: Vec<&str>) -> Vec<&str> {
          inputs
              .into_iter()
              .filter(|i| i.is_empty().not())
              .collect::<Vec<_>>()
      }

      /// Bad: forces caller to collect strings into a `Vec`, restricting input types and adding allocations.
      pub fn bar(inputs: Vec<String>) -> Vec<String> {}
      ```
- Prefer implementing and using `From` or `TryFrom` for conversions between types (instead of converting in-place).
- Don't use early-return fast-path guards for empty vectors, iterators, streams (i.e.: don't use `if items.is_empty() { return ...; }`).
- Use destructuring assignment for tuple args, for example: `fn try_from((name, parent_key): (&str, GroupKey)) -> ...`.
- Use iterators instead of for loops. For example:
  - Good:
    ```rust
    use errgonomic::{handle_iter, ErrVec};
    use core::num::ParseIntError;
    use thiserror::Error;

    // Good: iterator pipeline with fallible mapping and error handling.
    pub fn parse_numbers(inputs: impl IntoIterator<Item = impl AsRef<str>>) -> Result<Vec<u64>, ParseNumbersError> {
        use ParseNumbersError::*;
        let iter = inputs.into_iter().map(|s| s.as_ref().trim().parse::<u64>());
        Ok(handle_iter!(iter, InvalidInput))
    }

    #[derive(Error, Debug)]
    pub enum ParseNumbersError {
        #[error("failed to parse {len} numbers", len = source.len())]
        InvalidInput { source: ErrVec<ParseIntError> },
    }
    ```
  - Bad:
    ```rust
    use core::num::ParseIntError;

    // Bad: manual loop and mutable accumulator.
    pub fn parse_numbers(inputs: impl IntoIterator<Item = impl AsRef<str>>) -> Result<Vec<u64>, ParseIntError> {
        let mut out = Vec::new();
        for s in inputs {
            let n = s.as_ref().trim().parse::<u64>()?;
            out.push(n);
        }
        Ok(out)
    }
    ```
- If fn has a clear receiver (`self`, `&self`, `&mut self`), implement it as an associated fn; else: implement it as a standalone free fn.
- Add a local `use` statement for enums to minimize code size. For example:
  - Good:
    ```rust
    pub fn apply(op: GroupsOp) {
        use GroupsOp::*;
        match op {
            InsertOne(_) => {}
            UpdateOne(_, _) => {}
            DeleteOne(_) => {}
        }
    }
    ```
  - Bad:
    ```rust
    pub fn apply(op: GroupsOp) {
        match op {
            GroupsOp::InsertOne(_) => {}
            GroupsOp::UpdateOne(_, _) => {}
            GroupsOp::DeleteOne(_) => {}
        }
    }
    ```
- Simplify call-site code by accepting `impl Into`. For example:
  - Good:
    ```rust
    pub fn foo(input: impl Into<String>) {
        let input = input.into();
    }
    ```
  - Bad:
    ```rust
    /// Bad: caller may have to call `.into()` when passing input arg.
    pub fn foo(input: String) {}
    ```
- Provide additional flexibility at call sites by accepting `&impl AsRef` or `&mut impl AsMut` (e.g.: both `PathBuf` and `Config` may implement `AsRef<Path>`). For example:
  - Good:
    ```rust
    pub fn bar(input: &mut impl AsMut<String>) {
        let input = input.as_mut();
    }

    pub fn baz(input: &impl AsRef<str>) {
        let input = input.as_ref();
    }
    ```
  - Bad:
    ```rust
    /// Bad: caller may have to call `.as_mut()` when passing input arg.
    pub fn bar(input: &mut String) {}

    /// Bad: caller may have to call `.as_ref()` when passing input arg.
    pub fn baz(input: &str) {}
    ```
- Prefer `.map()` instead of `match` when you need to modify value in `Option` or `Result`. For example:
  - Good:
    ```rust
    use core::str::FromStr;
    use core::num::ParseIntError;

    impl FromStr for UserId {
        type Err = ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            s.parse::<u64>().map(Self::new)
        }
    }
    ```
  - Bad:
  ```rust
  use core::str::FromStr;
  use core::num::ParseIntError;

  impl FromStr for UserId {
      type Err = ParseIntError;

      fn from_str(s: &str) -> Result<Self, Self::Err> {
          // Bad: uses more code to express same idea.
          match s.parse::<u64>() {
              Ok(value) => Ok(Self::new(value)),
              Err(error) => Err(error),
          }
      }
  }
  ```
- Use `Self` instead of type name in `impl` items. For example:
  - Good:
  ```rust
  use core::time::Duration;

  impl From<Duration> for UnixTimestamp {
      #[inline]
      fn from(duration: Duration) -> Self {
          Self::new(duration.as_secs())
      }
  }
  ```
  - Bad:
  ```rust
  use core::time::Duration;

  impl From<Duration> for UnixTimestamp {
      #[inline]
      fn from(duration: Duration) -> Self {
          UnixTimestamp::new(duration.as_secs())
      }
  }
  ```
- Prefer short method syntax over long trait-path syntax (prefer `value.method(arg)` over `Trait::method(value, arg)`).
- Generic helper fns must be in `src/functions` (one file per fn).
- If Clippy reports `too_many_arguments`:
  - Then:
    - Don't use tuples to silence lint.
    - Consider refactoring code for better separation of concerns.

#### Struct derives

- Derive `new` from `derive_new` crate for types that need `fn new`.
- If struct derives `Getters`:
  - Then:
    - For each field:
      - If its type implements `Copy`, it must have a `#[getter(copy)]` annotation.
  - Examples:
    - Good (`username` has no `#[getter(copy)]` because `String` doesn't implement `Copy`, but `age` has it because `u64` implements `Copy`):
      ```rust
      #[derive(Getters, Into, Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
      pub struct User {
        username: String,
        #[getter(copy)]
        age: u64,
      }
      ```

#### Setters

- Use setters that take `&mut self` instead of setters that take `self` and return `Self` (because passing `foo: &mut Foo` is more efficient than passing `foo: Foo` and returning `Foo` through call stack).

#### Enums

- When writing code related to enums, bring variants into scope with a `use Enum::*;` statement at top of file or fn (prefer top of file for data enums and top of fn for error enums).

#### Arithmetic

- Don't use impls of traits `core::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign, Neg, Shl, ShlAssign, Shr, ShrAssign}` or their operators unless they don't panic or silently overflow.
- Write and use arithmetic trait impls that don't panic or silently overflow.
- Prefer `checked` versions of arithmetic ops.
- Every call to an `overflowing`, `saturating`, `wrapping` version must have a single-line comment above it that starts with "SAFETY: " and describes why calling this version is safe in this specific case.
- Use items from `num` crate if necessary (for example: to implement a fn that calls arithmetic methods on a generic type).

#### Index access

- Never use operators: `[], []=`.
- Never use traits: `core::ops::{Index, IndexMut}`.
- If you are sure that `get` or `get_mut` will never panic, use `expect` with a proof msg (as described in [Fns](#fns)).

#### Test fn

A fn marked with `#[test]` or `#[tokio::test]`.

- Must return a `Result`.
- Must implement proper error handling via `errgonomic` crate.
- Should use macros from `assertables` crate.
  - Should use `assert_infix` instead of `assert_gt`, `assert_ge`, `assert_lt`, `assert_le`, `assert_eq`.

#### Macros

- In `macro_rules!` expansions:
  - Use full paths without a leading `::` for foreign types (e.g.: `std::sync::Arc`), except those in standard prelude.
  - Use `$crate` paths for types defined in this crate (e.g.: `$crate::MyType`).
- If you see similar code in different places, replace similar code with a shared macro.
- If last arg of a macro arm is variadic (e.g.: `$($item:ident),*`), add a trailing `$(,)?`; else: don't add a trailing `$(,)?`.

#### Shell

- Prefer short options for cmds used in tool calls.

#### mise.toml

- Each `env` map value:
  - If it should be overridable by the user:
    - Then: it must be a map with `default` key (e.g. `PORT = { default = "3000" }`)
    - Else: it must be a specific value (e.g. `SHUCK_CACHE_DIR = "{{config_root}}/.cache"`)

#### Cargo.toml

- Don't define pkg features with only a single optional dep (such features are already defined by Cargo automatically).
- Use `cargo add` to add deps.
- When adding a dep from crates.io:
  - If pkg is [publishable](#publishable-pkg):
    - Then:
      - Run `cargo add {dependency}@{version}`.
        - Patch component of `{version}` must be 0.
      - Try `cargo update -p {dependency} --precise {version}` to lock that exact version.
        - If dep constraints prevent locking that version:
          - Then:
            - Keep version resolved by Cargo.
            - Add a comment in `Cargo.toml` explaining constraints.
    - Else:
      - Run `cargo add {dependency}` without `{version}`.
- When adding a dep in a workspace:
  - Add it to top-level manifest first (`workspace.dependencies`).
- When adding a dep for a workspace member:
  - Run `cargo add {dependency}` without `{version}` (Cargo will set `workspace = true`).
- When adding a new workspace member: add it to `packages` dir unless specified otherwise.

#### Code style

- Don't enforce a line len limit when writing code, comments, docs.

#### Chat thread id

- Must be a string.
- Must contain at least 3 characters.
- Must contain only uppercase characters.

Examples:

- `RVC`.
- `AKE`.
- `LMY`.

Notes:

- Should match thread topic.

#### Chat thread id heading

A level-3 Markdown heading that contains only a [chat thread id](#chat-thread-id).

Examples:

- `### RVC`.
- `### AKE`.
- `### LMY`.

#### Finding

- Must be formatted as `### {ctid}\n\n[{priority}] {title}. {body} ({references}). Proposed fixes: {fixes}`.
  - `ctid` must be a [chat thread id](#chat-thread-id).
  - `priority` must be one of `P0`, `P1`, `P2`, `P3`.
  - `references` must be a comma-separated list of `reference` values.
  - `reference` must be formatted as `{path}:{line}`.
  - `path` must be a file path relative to your working dir.
  - `line` must be first line of relevant code or text block.
  - `fixes` must be one of:
    - If there is at least one proposed fix, "\n\n" and a nested Markdown list of fixes where each fix must have format `{number}. {description}` (numbers should start from 1 for each list of fixes); else: exact text "none."

#### Publishable pkg

A pkg that has a remote whose name contains `public` or `pre-public` and ends with `template`.

## Project files

### mise.toml

```toml
min_version = "2026.7.13"

[settings]
idiomatic_version_file_enable_tools = ["rust"]
task.output = "keep-order"

[tools]
node = "24.15.0"
deno = "1.46.1"
fnox = "1.33.1"
fd = "10.4.2"
"github:ewhauser/shuck" = "0.2.2"
"aqua:nushell/nushell" = "0.116.1"
# nu-lint's dependencies require a newer compiler than this project's toolchain.
"cargo:nu-lint" = { version = "1.8.0", install_env = { RUSTUP_TOOLCHAIN = "1.99.1" } }
"github:nushell/nufmt" = "0.1.6"
"aqua:rvben/rumdl" = "0.3.0"
"npm:@commitlint/config-conventional" = "19.6.0"
"npm:@commitlint/cli" = "19.6.0"
"npm:@commitlint/types" = "19.5.0"
"npm:skills" = "1.5.24"
"cargo:https://github.com/DenisGorbachev/cargo-insert-docs" = { version = "rev:9bccf15cc367a50d2652b0eaf5da7faf5929c666", crate = "cargo-insert-docs", locked = true }
"cargo:cargo-hack" = "0.6.33"
"cargo:cargo-nextest" = "0.9.145"
"cargo:cargo-expand" = "1.0.114"
"cargo:taplo-cli" = "0.10.0"
"cargo:sd" = "1.0.0"
"cargo:cargo-edit" = "0.13.13"

[hooks]
postinstall = { task = "git:install-hooks" }

[env]
SHUCK_CACHE_DIR = "{{config_root}}/.cache"

[tasks."build"]
run = "cargo build --workspace"

[tasks."check"]
depends = ["cargo:validate-config"]
run = [{ tasks = ["lint", "test"] }]

[tasks."check:ci"]
depends = ["cargo:validate-config"]
run = [{ tasks = ["lint", "test:ci"] }]

[tasks."test"]
depends = ["test:code", "test:docs"]

[tasks."lint"]
depends = ["lint:name", "lint:configs", "lint:code", "lint:code:style", "lint:shell", "lint:nu", "lint:docs", "lint:generated", "lint:reports"]

[tasks."lint:name"]
run = [{ task = "fix:name", args = ["--check"] }]

[tasks."lint:configs"]
depends = ["lint:configs:cargo", "lint:configs:fnox"]

[tasks."lint:configs:cargo"]
run = [{ task = "fix:cargo", args = ["--check"] }]

[tasks."lint:configs:fnox"]
run = [{ task = "fix:fnox" }]

[tasks."lint:code"]
run = "cargo clippy --locked --workspace --all-targets --all-features -- -D warnings"

[tasks."lint:code:style"]
run = "cargo fmt --all -- --check"

[tasks."lint:shell"]
# Shuck 0.2.2 scans Git-ignored local files, so exclude them explicitly.
run = '''shuck --config "lint.source-paths = ['$HOME']" check --extend-exclude '*.local.*' .'''

[tasks."lint:docs"]
run = "rumdl check"

[tasks."lint:generated"]
run = ["./AGENTS.ts", "./README.ts"]

[tasks."test:code"]
depends = ["test:code:regular", "test:code:integration"]

[tasks."test:code:regular"]
run = "fnox --profile test exec --replace -- cargo nextest run --locked --workspace --all-features --no-tests warn --cargo-quiet --show-progress none --no-input-handler --status-level fail --final-status-level flaky --no-fail-fast"

[tasks."test:code:integration"]
# `--test-threads 1` because integration tests must be run sequentially
run = [{ task = "test:code:regular", args = ["--ignore-default-filter", "--test-threads", "1", "integration_tests::"] }]

[tasks."test:docs"]
env = { RUSTDOCFLAGS = "-D warnings" }
run = "cargo test --locked --workspace --doc --all-features --no-fail-fast --quiet"

[tasks."test:ci"]
env = { RUSTDOCFLAGS = "-D warnings" }
run = "fnox --profile test exec --replace -- cargo hack test --locked --workspace --feature-powerset --keep-going --no-fail-fast --quiet -- --skip integration_tests::"

[tasks."pre-commit"]
alias = "pre-merge-commit"
run = [{ task = "git:validate-commit" }]

[tasks."commit-msg"]
run = 'mise run --output interleave commitlint -- --edit "$@"'

[tasks."fix"]
depends = ["cargo:validate-config"]
run = [{ task = "fix:aux" }]

[tasks."fix:aux"]
depends = ["fix:configs", "fix:docs", "fix:agents", "fix:readme"]

[tasks."fix:configs"]
depends = ["fix:cargo", "fix:fnox"]

[tasks."fix:code"]
depends = ["fix:name", "fix:code:style", "fix:shell", "fix:nu"]

[tasks."fix:nu"]
run = [{ task = "nu", args = ["nufmt", "--shebang"] }]

[tasks."fix:shell"]
run = [{ task = "lint:shell", args = ["--fix"] }]

[tasks."fix:code:warnings"]
depends = ["fix:cargo"]
# second pass is needed because "cargo clippy --fix" exits with 0 even if some warnings remain
env = { __CARGO_FIX_YOLO = 'yeah' }
run = [
    "cargo clippy --workspace --all-targets --all-features --fix --allow-dirty --allow-staged",
    { task = "lint:code" },
]

[tasks."fix:code:style"]
# Run after `fix:code:warnings` because both tasks modify the same code files.
depends = ["fix:code:warnings"]
run = "cargo fmt --all"

[tasks."fix:docs"]
depends = ["fix:agents", "fix:readme"]
# use `rumdl check --fix` instead of `rumdl fmt` because `rumdl check --fix` exits with 1 if errors remain (since v0.1.0)
run = "rumdl check --fix"

[tasks."fix:agents"]
# "fix:agents" depends on "fix:code" because it reads the code files
depends = ["fix:name", "fix:configs", "fix:code"]
run = [{ task = "gen:agents" }]

[tasks."gen:readme"]
run = "./README.ts --fix"

[tasks."gen:agents"]
run = "./AGENTS.ts --fix"

[tasks."commitlint"]
run = "commitlint --extends \"$(mise where npm:@commitlint/config-conventional)/node_modules/@commitlint/config-conventional/lib/index.js\""

[tasks."agent:docs:list"]
run = "[ -d .agents/docs ] && find .agents/docs -type f -print || true"
output = "interleave"
quiet = true
```

### fnox.toml

```toml
#:schema https://fnox.jdx.dev/schema.json

if_missing = "error"
env = "exec"

[providers]
keychain = { type = "keychain", service = "stub-macro" }
pass = { type = "password-store", prefix = "stub-macro/" }
age = { type = "age", recipients = [
    "age1sf4r4amev2svqr6llwg8hgtz9n7p5qdh7hh0mavcshzfrmgfduksnq3hql",
    "age1605gsnxpe536sprwccyumq74veg0g80u55n8ggems0t8deau6qdsfnq3m3"
] }
```

### Cargo.toml

```toml
[workspace]
resolver = "3"

[workspace.package]
version = "0.3.1"
edition = "2021"
rust-version = "1.85.0"
license = "Apache-2.0 OR MIT"
homepage = "https://github.com/DenisGorbachev/stub-macro"
repository = "https://github.com/DenisGorbachev/stub-macro"
keywords = ["stub", "utils", "testing", "development"]
categories = ["rust-patterns", "development-tools", "no-std", "no-std::no-alloc"]
exclude = [
    ".*",
    "*.local.*",
    "doc/dev",
    "specs",
    "AGENTS.ts",
    "CargoMetadata.ts",
    "GeneratedFile.ts",
    "README.ts",
    "AGENTS*.md",
    "deno.lock",
    "deno.json",
    "clippy.toml",
    "commitlint.config.mjs",
    "fnox.toml",
    "mise.toml",
    "nu-lint.toml",
    "nufmt.nuon",
    "rumdl.toml",
    "shuck.toml",
    "rustfmt.toml",
    "rust-toolchain.toml",
    "skills-lock.json",
    ".yolobox"
]

[workspace.metadata.details]
name = "stub-macro"
title = "stub!() - a better todo!()"
readme = { generate = false }

[workspace.lints.rust]
redundant_imports = "deny"
unused_import_braces = "deny"
# unused_qualifications must not be "deny" because our code style has multiple `use Foo::*;`, and some macros (derive_more::Display, strum::Display, strum::EnumString) produce code with full qualifications
# unused_qualifications = "deny"

[workspace.lints.clippy]
absolute_paths = "deny"
arithmetic_side_effects = "deny"

[package]
name = "stub-macro"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "stub!() is a better version of todo!() that can be assigned to a variable"
license.workspace = true
homepage.workspace = true
repository.workspace = true
keywords.workspace = true
categories.workspace = true
exclude.workspace = true

[package.metadata.details]
title = "stub!() - a better todo!()"

[lints]
workspace = true

[dependencies]
futures-util-0-3 = { version = "0.3", package = "futures-util", optional = true, default-features = false }

[dev-dependencies]
futures-core-0-3 = { version = "0.3", package = "futures-core", default-features = false }

[features]
default = ["alloc"]
alloc = []
futures = ["dep:futures-util-0-3"]
```

### src/lib.rs

````rust
//! `stub!()` can be assigned to a variable:
//!
//! ```rust
//! # #[macro_use] extern crate stub_macro;
//! # fn main() {
//! # fn assign() {
//! let username = stub!(String);
//! # }
//! # }
//! ```
//!
//! This allows you to specify just type of the variable and continue working on other code, then come back later and specify its value.
//!
//! ## Examples
//!
//! ```
//! # #[macro_use] extern crate stub_macro;
//! # #[cfg(feature = "futures")]
//! # extern crate futures_core_0_3 as futures_core;
//! # fn main() {
//! fn assign() {
//!     // you can assign stub!() to a variable
//!     let username = stub!(String);
//!     println!("Hello {username}")
//! }
//!
//! fn return_position() -> String {
//!     // you can use stub!() like todo!() in return position
//!     stub!()
//! }
//!
//! fn infer_type() {
//!     // you can let the compiler automatically infer the type
//!     let status = stub!();
//!     if status { println!("Success") }
//! }
//!
//! fn explicit_type() {
//!     // you can specify the type explicitly
//!     let status: bool = stub!();
//!     if status { println!("Success") }
//! }
//!
//! fn custom_message() {
//!     // you can add a custom message
//!     let status: bool = stub!("Send a request to GitHub");
//!     if status { println!("Success") }
//! }
//!
//! # #[cfg(feature = "alloc")]
//! fn impl_example() -> impl core::fmt::Display {
//!     // you can use stub!() in return position even with `impl Trait` return type
//!     // note: `impl Trait` must be written as `impl dyn Trait` due to `macro_rules!` limitation
//!     stub!(impl dyn core::fmt::Display)
//! }
//!
//! fn iter_example() -> impl Iterator<Item = u32> {
//!     // use stub_iter!() when the return type is an iterator
//!     stub_iter!()
//! }
//!
//! #[cfg(feature = "futures")]
//! fn stream_example() -> impl futures_core::stream::Stream<Item = u32> {
//!     // use stub_stream!() when the return type is a stream
//!     stub_stream!()
//! }
//!
//! fn explicit_type_with_message_example() -> u32 {
//!     // you can add
//!     stub!(u32, "Assigned to: {}", "John")
//! }
//!
//! fn explicit_type_example() -> u32 {
//!     stub!(u32)
//! }
//!
//! fn implicit_type_with_message_example() -> u32 {
//!     stub!("Assigned to: {}", "John")
//! }
//!
//! fn implicit_type_example() -> u32 {
//!     stub!()
//! }
//! # }
//! ```
//!
//! ## Mark unused variables as used
//!
//! Use `stub_use!` to mark variables as used:
//!
//! ```should_panic
//! # #![deny(unreachable_code, unused_variables)]
//! # use stub_macro::stub_use;
//! fn authenticate(username: &str, password: &str) -> bool {
//!     let attempts = 0;
//!     let authenticated = stub_use!(username, password, attempts);
//!     authenticated
//! }
//! # authenticate("alice", "secret");
//! ```
//!
//! ## Behavior
//!
//! When a stub is invoked, it will panic like a `todo!()` macro.
//! However, unlike a `todo!()` macro, it will not make the subsequent parts of your code unreachable.
//!
//! If a custom message is provided, it will be included in the panic message.
//!
//! ## Notes
//!
//! - `stub!()` macro is intended for use during development and should be replaced with actual implementations before production use.
//! - When using `impl Trait` in return position, you must use `impl dyn Trait` in the macro invocation due to a limitation in `macro_rules!`
//! - `stub!(impl dyn Trait)` requires the `alloc` feature.
//! - `stub_stream!()` requires the `futures` feature.
//!

#![cfg_attr(not(test), deny(unused_crate_dependencies))]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;
use core::fmt::Arguments;

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "alloc")]
macro_rules! _stub_impl_dyn {
    ($ty:ty) => {{
        $crate::_stub_box::<$ty>()
    }};
    ($ty:ty, $fmt:expr) => {{
        $crate::_stub_box_msg::<$ty>(::core::format_args!($fmt))
    }};
    ($ty:ty, $fmt:expr, $($args:tt)*) => {{
        $crate::_stub_box_msg::<$ty>(::core::format_args!($fmt, $($args)*))
    }};
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "alloc"))]
macro_rules! _stub_impl_dyn {
    ($ty:ty) => {{
        compile_error!("stub!(impl dyn ...) requires the `alloc` feature")
    }};
    ($ty:ty, $fmt:expr) => {{
        compile_error!("stub!(impl dyn ...) requires the `alloc` feature")
    }};
    ($ty:ty, $fmt:expr, $($args:tt)*) => {{
        compile_error!("stub!(impl dyn ...) requires the `alloc` feature")
    }};
}

/// See the crate-level documentation for the overview of this macro.
#[macro_export]
macro_rules! stub {
    (impl $ty:ty, $fmt:expr) => {{
        $crate::_stub_impl_dyn!($ty, $fmt)
    }};
    (impl $ty:ty, $fmt:expr, $($args:tt)*) => {{
        $crate::_stub_impl_dyn!($ty, $fmt, $($args)*)
    }};
    (impl $ty:ty) => {{
        $crate::_stub_impl_dyn!($ty)
    }};
    ($ty:ty, $fmt:expr) => {{
        $crate::_stub_msg::<$ty>(::core::format_args!($fmt))
    }};
    ($ty:ty, $fmt:expr, $($args:tt)*) => {{
        $crate::_stub_msg::<$ty>(::core::format_args!($fmt, $($args)*))
    }};
    ($ty:ty) => {{
        $crate::_stub::<$ty>()
    }};
    ($fmt:expr) => {{
        $crate::_stub_msg(::core::format_args!($fmt))
    }};
    ($fmt:expr, $($args:tt)*) => {{
        $crate::_stub_msg(::core::format_args!($fmt, $($args)*))
    }};
    () => {{
        $crate::_stub()
    }};
}

/// See the crate-level documentation for the overview of this macro.
#[macro_export]
macro_rules! stub_iter {
    () => {
        $crate::_stub::<::core::iter::Empty<_>>()
    };
    ($fmt:expr) => {
        $crate::_stub_msg::<::core::iter::Empty<_>>(::core::format_args!($fmt))
    };
    ($fmt:expr, $($args:tt)*) => {
        $crate::_stub_msg::<::core::iter::Empty<_>>(::core::format_args!($fmt, $($args)*))
    };
}

/// See the crate-level documentation for the overview of this macro.
#[macro_export]
#[cfg(feature = "futures")]
macro_rules! stub_stream {
    () => {
        $crate::_stub::<$crate::_StubEmptyStream<_>>()
    };
    ($fmt:expr) => {
        $crate::_stub_msg::<$crate::_StubEmptyStream<_>>(::core::format_args!($fmt))
    };
    ($fmt:expr, $($args:tt)*) => {
        $crate::_stub_msg::<$crate::_StubEmptyStream<_>>(::core::format_args!($fmt, $($args)*))
    };
}

#[doc(hidden)]
#[cfg(feature = "futures")]
pub use futures_util_0_3::stream::Empty as _StubEmptyStream;

/// Marks one or more variable names as used, then panics with `todo!()`.
#[macro_export]
macro_rules! stub_use {
    ($($name:ident),+ $(,)?) => {{
        $(let _ = $name;)+
        $crate::_stub()
    }};
}

#[doc(hidden)]
pub fn _stub<T>() -> T {
    todo!()
}

#[doc(hidden)]
#[cfg(feature = "alloc")]
pub fn _stub_box<T: ?Sized>() -> Box<T> {
    _stub::<Box<T>>()
}

#[doc(hidden)]
#[cfg(feature = "alloc")]
pub fn _stub_box_msg<T: ?Sized>(msg: Arguments<'_>) -> Box<T> {
    _stub_msg::<Box<T>>(msg)
}

#[doc(hidden)]
pub fn _stub_msg<T>(msg: Arguments<'_>) -> T {
    todo!("{msg}")
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "alloc")]
    use alloc::string::String;
    #[cfg(feature = "alloc")]
    use core::fmt::Display;
    #[cfg(feature = "futures")]
    use futures_core_0_3::stream::Stream;

    #[test]
    #[should_panic]
    fn test_panic() {
        stub!()
    }

    #[test]
    #[should_panic(expected = "Assigned to: John")]
    fn test_panic_msg() {
        let _: u32 = stub!(u32, "Assigned to: {}", "John");
    }

    #[cfg(feature = "alloc")]
    #[test]
    #[should_panic(expected = "Assigned to: x")]
    fn test_panic_msg_with_path_expression() {
        let _: u32 = stub!(u32, "Assigned to: {}", String::from("x"));
    }

    #[test]
    #[should_panic(expected = "Assigned to: John")]
    fn test_panic_msg_with_format_expression() {
        let _: u32 = stub!(u32, concat!("Assigned to: ", "{}"), "John");
    }

    #[test]
    #[should_panic(expected = "Assigned to: John")]
    fn test_panic_msg_with_named_argument() {
        let name = "John";
        let _: u32 = stub!(u32, "Assigned to: {name}", name = name);
    }

    #[test]
    #[should_panic]
    fn test_panic_iter() {
        fn iter() -> impl Iterator<Item = u32> {
            stub_iter!()
        }
        let _ = iter();
    }

    #[cfg(feature = "alloc")]
    #[test]
    #[should_panic(expected = "Assigned to: x")]
    fn test_panic_iter_with_path_expression() {
        fn iter() -> impl Iterator<Item = u32> {
            stub_iter!("Assigned to: {}", String::from("x"))
        }
        let _ = iter();
    }

    #[test]
    #[should_panic(expected = "Assigned to: John")]
    fn test_panic_iter_with_format_expression() {
        fn iter() -> impl Iterator<Item = u32> {
            stub_iter!(concat!("Assigned to: ", "{}"), "John")
        }
        let _ = iter();
    }

    #[test]
    #[should_panic(expected = "Assigned to: John")]
    fn test_panic_iter_with_named_argument() {
        fn iter() -> impl Iterator<Item = u32> {
            let name = "John";
            stub_iter!("Assigned to: {name}", name = name)
        }
        let _ = iter();
    }

    #[cfg(feature = "alloc")]
    #[test]
    #[should_panic]
    fn test_panic_dyn_display() {
        fn display() -> impl Display {
            stub!(impl dyn Display)
        }
        let _ = display();
    }

    #[cfg(feature = "alloc")]
    #[test]
    #[should_panic]
    fn test_panic_dyn_iter() {
        fn iter() -> impl Iterator<Item = u32> {
            stub!(impl dyn Iterator<Item = u32>)
        }
        let _ = iter();
    }

    #[cfg(feature = "futures")]
    #[test]
    #[should_panic]
    fn test_panic_stream() {
        fn stream() -> impl Stream<Item = u32> {
            stub_stream!()
        }
        let _ = stream();
    }

    #[cfg(all(feature = "alloc", feature = "futures"))]
    #[test]
    #[should_panic(expected = "Assigned to: x")]
    fn test_panic_stream_with_path_expression() {
        fn stream() -> impl Stream<Item = u32> {
            stub_stream!("Assigned to: {}", String::from("x"))
        }
        let _ = stream();
    }

    #[cfg(feature = "futures")]
    #[test]
    #[should_panic(expected = "Assigned to: John")]
    fn test_panic_stream_with_format_expression() {
        fn stream() -> impl Stream<Item = u32> {
            stub_stream!(concat!("Assigned to: ", "{}"), "John")
        }
        let _ = stream();
    }

    #[cfg(feature = "futures")]
    #[test]
    #[should_panic(expected = "Assigned to: John")]
    fn test_panic_stream_with_named_argument() {
        fn stream() -> impl Stream<Item = u32> {
            let name = "John";
            stub_stream!("Assigned to: {name}", name = name)
        }
        let _ = stream();
    }
}
````
