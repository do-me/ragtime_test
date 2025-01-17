# Tauri + Vanilla

This template should help get you started developing with Tauri in vanilla HTML, CSS and Javascript.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Error Log on MacOS

```shell
(base) ➜  ragtime_test git:(main) cargo tauri dev                                
warning: unused manifest key: target.cfg(target_os = "macos").rustflags
    Info Watching /Users/dome/work/general/tauri/ragtime_test/ragtime_test/src-tauri for changes...
   Compiling llama-cpp-sys-2 v0.1.87
   Compiling llama-cpp-2 v0.1.87
   Compiling ragtime v0.2.0
error[E0432]: unresolved import `ort::Session`
 --> /Users/dome/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ragtime-0.2.0/src/lib.rs:4:5
  |
4 | use ort::Session;
  |     ^^^^^^^^^^^^ no `Session` in the root
  |
help: a similar name exists in the module (notice the capitalization difference)
  |
4 | use ort::session;
  |          ~~~~~~~
help: consider importing this struct instead
  |
4 | use ort::session::Session;
  |     ~~~~~~~~~~~~~~~~~~~~~

error[E0432]: unresolved imports `ort::Session`, `ort::SessionOutputs`
 --> /Users/dome/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ragtime-0.2.0/src/bge_m3/onnx.rs:7:19
  |
7 | use ort::{inputs, Session, SessionOutputs};
  |                   ^^^^^^^  ^^^^^^^^^^^^^^ no `SessionOutputs` in the root
  |                   |
  |                   no `Session` in the root
  |                   help: a similar name exists in the module (notice the capitalization): `session`
  |
  = help: consider importing one of these items instead:
          crate::Session
          ort::session::Session
  = help: consider importing this struct instead:
          ort::session::SessionOutputs

error[E0432]: unresolved import `ort::Session`
 --> /Users/dome/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ragtime-0.2.0/src/gte_large_en/onnx.rs:7:19
  |
7 | use ort::{inputs, Session};
  |                   ^^^^^^^
  |                   |
  |                   no `Session` in the root
  |                   help: a similar name exists in the module (notice the capitalization): `session`
  |
  = help: consider importing one of these items instead:
          crate::Session
          ort::session::Session

error[E0432]: unresolved imports `ort::DynValue`, `ort::Session`, `ort::SessionInputValue`
 --> /Users/dome/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ragtime-0.2.0/src/phi3/onnx.rs:5:11
  |
5 | use ort::{DynValue, Session, SessionInputValue};
  |           ^^^^^^^^  ^^^^^^^  ^^^^^^^^^^^^^^^^^ no `SessionInputValue` in the root
  |           |         |
  |           |         no `Session` in the root
  |           |         help: a similar name exists in the module (notice the capitalization): `session`
  |           no `DynValue` in the root
  |
  = help: consider importing this type alias instead:
          ort::value::DynValue
  = help: consider importing one of these items instead:
          crate::Session
          ort::session::Session
  = help: consider importing this enum instead:
          ort::session::SessionInputValue

error[E0433]: failed to resolve: could not find `GraphOptimizationLevel` in `ort`
  --> /Users/dome/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ragtime-0.2.0/src/lib.rs:27:39
   |
27 |         .with_optimization_level(ort::GraphOptimizationLevel::Level3)?
   |                                       ^^^^^^^^^^^^^^^^^^^^^^ could not find `GraphOptimizationLevel` in `ort`
   |
help: consider importing this enum
   |
1  + use ort::session::builder::GraphOptimizationLevel;
   |
help: if you import `GraphOptimizationLevel`, refer to it directly
   |
27 -         .with_optimization_level(ort::GraphOptimizationLevel::Level3)?
27 +         .with_optimization_level(GraphOptimizationLevel::Level3)?
   |

error[E0308]: mismatched types
   --> /Users/dome/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ragtime-0.2.0/src/llama.rs:131:29
    |
131 |             .with_n_threads(t.inner.args.threads)
    |              -------------- ^^^^^^^^^^^^^^^^^^^^ expected `i32`, found `u32`
    |              |
    |              arguments to this method are incorrect
    |
note: method defined here
   --> /Users/dome/.cargo/registry/src/index.crates.io-6f17d22bba15001f/llama-cpp-2-0.1.87/src/context/params.rs:394:12
    |
394 |     pub fn with_n_threads(mut self, n_threads: i32) -> Self {
    |            ^^^^^^^^^^^^^^
help: you can convert a `u32` to an `i32` and panic if the converted value doesn't fit
    |
131 |             .with_n_threads(t.inner.args.threads.try_into().unwrap())
    |                                                 ++++++++++++++++++++

error[E0308]: mismatched types
   --> /Users/dome/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ragtime-0.2.0/src/llama.rs:132:35
    |
132 |             .with_n_threads_batch(t.inner.args.threads)
    |              -------------------- ^^^^^^^^^^^^^^^^^^^^ expected `i32`, found `u32`
    |              |
    |              arguments to this method are incorrect
    |
note: method defined here
   --> /Users/dome/.cargo/registry/src/index.crates.io-6f17d22bba15001f/llama-cpp-2-0.1.87/src/context/params.rs:410:12
    |
410 |     pub fn with_n_threads_batch(mut self, n_threads: i32) -> Self {
    |            ^^^^^^^^^^^^^^^^^^^^
help: you can convert a `u32` to an `i32` and panic if the converted value doesn't fit
    |
132 |             .with_n_threads_batch(t.inner.args.threads.try_into().unwrap())
    |                                                       ++++++++++++++++++++

error[E0599]: no method named `with_seed` found for struct `LlamaContextParams` in the current scope
   --> /Users/dome/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ragtime-0.2.0/src/llama.rs:134:14
    |
127 |           let ctx_params = LlamaContextParams::default()
    |  __________________________-
128 | |             .with_n_ctx(Some(
129 | |                 NonZeroU32::new(n_ctx).ok_or_else(|| anyhow!("trained context size is zero"))?,
130 | |             ))
...   |
133 | |             .with_embeddings(embed)
134 | |             .with_seed(t.inner.args.seed)
    | |             -^^^^^^^^^ method not found in `LlamaContextParams`
    | |_____________|
    |

error[E0599]: no method named `sample_token_softmax` found for mutable reference `&mut LlamaContext<'static>` in the current scope
   --> /Users/dome/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ragtime-0.2.0/src/llama.rs:188:17
    |
188 |             ctx.sample_token_softmax(&mut a);
    |                 ^^^^^^^^^^^^^^^^^^^^ method not found in `&mut LlamaContext<'static>`

Some errors have detailed explanations: E0308, E0432, E0433, E0599.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `ragtime` (lib) due to 9 previous errors
```
