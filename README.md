# Lisper

A basic Lisp interpreter built in Rust. Try it at [lisper.victr.com](https://lisper.victr.com).

This project explores Rust and WebAssembly, inspired by [Stepan Parunashvili](https://stopa.io/post/222), [Peter Norvig's Lispy](https://norvig.com/lispy.html), and [Ronin](https://github.com/hundredrabbits/Ronin) by [Hundred Rabbits](https://100r.co/).

## Structure

The repository contains four related parts:

1. **Lisper library** (`src/`) — the core Lisp interpreter.
2. **REPL** (`repl/`) — a command-line interface to the interpreter.
3. **WASM wrapper** (`wasm/`) — exposes the interpreter to the browser.
4. **Web app** (`lisper-web/`) — a Next.js editor powered by the WebAssembly module.

Keeping these together makes it easy to iterate on the interpreter and its clients. GitHub Actions runs Rust and browser tests and handles Vercel deployments.

## Requirements

- Rust and Cargo (CI uses Rust 1.80.1)
- For the web app: Node.js 26, npm, `wasm32-unknown-unknown` Rust target, and `wasm-pack` 0.15.1

## Run the REPL

From the repository root:

```sh
cargo run
```

Run the Rust tests and lint checks:

```sh
cargo fmt --all -- --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

## Run the web app

Build the browser-native WASM module, copy its generated assets, and start Next.js:

```sh
rustup target add wasm32-unknown-unknown
# Install wasm-pack 0.15.1 if it is not already installed.
cd wasm
wasm-pack build --out-dir ../lisper-web/pkg --target web
cd ..
mkdir -p lisper-web/public/wasm
cp lisper-web/pkg/lisper_wasm.js lisper-web/pkg/lisper_wasm_bg.wasm lisper-web/public/wasm/
cd lisper-web
npm ci
npm run dev
```

Open [http://localhost:3000](http://localhost:3000). See [web app development](lisper-web/README.md) and [browser tests](e2e/README.md) for details.

## Lisper language

The following describes the existing interpreter interface.

### Built-in functions

| Function | Example |
| --- | --- |
| `+` | `(+ x y)` |
| `-` | `(- x y)` |
| `*` | `(* x y)` |
| `/` | `(/ x y)` |
| `%` | `(% x y)` |
| `sin` | `(sin x)` |
| `cos` | `(cos x)` |
| `tan` | `(tan x)` |
| `<` | `(< x y)` |
| `>` | `(> x y)` |
| `<=` | `(<= x y)` |
| `>=` | `(>= x y)` |

Argument counts depend on the operation; not all functions accept an arbitrary number of arguments.

### Predefined constants

| Value | Name |
| --- | --- |
| π | `pi` |
| 2π | `two_pi` |
| e | `e` |

### Define a value

```lisp
(def a 10)
```

The first argument names the binding; the second is an expression evaluated to produce its value.

### Conditional

```lisp
(if condition true_expression false_expression)
```

The condition determines which branch is evaluated.

### Define a function

```lisp
(fn function_name argument_name ... function_expression)
```

Argument names are separate symbols, not a parenthesized argument list. Functions execute in a scoped environment that inherits from the parent environment, and calls must provide the defined number of arguments. For example: `(fn add-one x (+ x 1))` followed by `(add-one 4)`.
