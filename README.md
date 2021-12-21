# Lisper
A basic Lisp interpreter built in Rust. Try it at [lisper.victr.com](https://lisper.victr.com).

This is a project to explore Rust and WebAssembly, inspired by the great work done by [Stepan Parunashvili](https://stopa.io/post/222) and the original post from [Peter Norvig](http://norvig.com/lispy.html). There is also a fair bit of foundational curiosity that came from playing around with [Ronin](https://github.com/hundredrabbits/Ronin), by the always interesting [Hundred Rabbits](https://100r.co/).

## Structure

The project has four parts:

1. The lisper lib - the core functionality of the lisp interpreter
2. REPL app - a command line interface for lisper
3. WASM lib - that packages the lib as a redy to be used WASM module
4. A web app - a Next.js app that offers a Lisp editor, using the WASM module

It probably isn't the best idea to have all this in one repo, but for now it allows for fast iteration on the various interdependant parts locally and deployed. Uses Github Actions to automatically build, run tests, and delpoy to Vercel.


## Install and use

Just clone the repo and get going, there are no dependencies beyond rust and cargo.

To build and run:
```
$ cargo run
```

Launches the REPL.

To run the tests:
```
$ cargo test
```

## Lisper doc

**Lisper environment functions**
These are the currently defined functions implemented for Lisper, more to come.

| Function | Example |
| -------- | ------- |
| +        | `(+ x y)` |
| -        | `(- x y)` |
| \*       | `(\* x y)`|
| /        | `(/ x y)` |
| %        | `(% x y)` |
| sin      | `(sin x)` |
| cos      | `(cos x)` |
| tan      | `(tan x)` |
| <        | `(< x y)` |
| >        | `(> x y)` |
| <=       | `(<= x y)`|
| >=       | `(>= x y)`|

All functions currently take n amount of arguments, e.g. (+ 1 1 1 1 1 ...).

**Predefined constants**

| Value | Constants |
| ----- | --------- |
| π     | `pi`        |
| π * 2 | `two_pi`    |
| e     | `e`         |

**def**

Format: `(def name value_exp)`

* name - Can be any string and non-numeric character
 * There is no validation of not being able to overwrite existing defined functions or constants
* value_exp - Any valid Lisper expression that resolves to a value

All defined values and functions are global within the current and inherited scopes.

**if**

Format: `(if if_evaluation_expr true_value false_value)`

* if_evaluation_expr - Any Lisper expression that evaluates to a number or bool
* true_value - Any valid Lisper expression that resolves to a value
* false_value - Any valid Lisper expression that resolves to a value

**fn**

Format: `(fn fn_name argument_name function_exp)`

* fn_name - Can be any string and non-numeric character
 * There is no validation of not being able to overwrite existing defined functions or constants
* argument_name - Can be any string and non-numeric character
 * There is no validation of not being able to overwrite existing defined functions or constants
* function_exp - Any valid Lisper expression that resolves to a value

Lambda functions execute within it's own scoped environment, inheriting from the parent environment.
All defined values and functions are global within the current and inherited scopes.
