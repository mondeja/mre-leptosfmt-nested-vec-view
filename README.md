# Nested vector `view!` leptosfmt formatting bug MRE

This is a minimal reproducible example of a bug in the `leptosfmt` package.

A nested `view!` inside a vector with `vec![ ... ]` is formatted incorrectly. In this example, the next snippet

```rust
vec![
    view!{ cx, <button class="button-foo button-bar button-baz">"button 1"</button>},
]
```

is converted to

```rust
vec![
    view! { cx, < button class = "button-foo button-bar button-baz" > "button 1" </
    button > },
]
```

Or unwrapping lines:

```rust
vec![
    view!{ cx, <button class="button-foo">"button 1"</button>},
]
```

```rust
vec![view! { cx, < button class = "button-foo" > "button 1" </ button > },]
```

## To reproduce

See the previous content of `src/lib.rs`. Then execute:

```bash
cargo install leptosfmt
leptosfmt src
```

See the new content of `src/lib.rs`.

## To run the app

```bash
cargo install wasm-pack simple-http-server
wasm-pack build --target web
simple-http-server . --index --open
```
