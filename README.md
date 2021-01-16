# Lisper
Basic Lisp interpreter, implemented in Rust.

This is a project to learn Rust, inspired and based on the great work of [Stepan Parunashvili](https://stopa.io/post/222) and the original post from [Peter Norvig](http://norvig.com/lispy.html). There is also a fair bit of foundational curiosity that came from [Ronin](https://github.com/hundredrabbits/Ronin), by the great [Hundred Rabbits](https://100r.co/).

## Structure

The project has four parts:

1. The lisper lib - the core functionality of the lisp interpreter
2. REPL app - for using the lib from the commandline
3. WASM lib - that packages the lib as a WASM module
4. A web app - a next.js app that uses the WASM module

It probably isn't the best idea to have all this in one repo, but for now it allows for fast iteration on various interdependant parts.


## Install and use

Just clone the repo and get going, there are no dependencies beyond rust and cargo.

To build and run:
```
$ cargo run
```

Currently just runs a few example lisp statements and prints the results.

To run the tests:
```
$ cargo test
```

## Lisper doc

These are the currently defined functions implemented for Lisper, more to come.

| Function | Example |
| -------- | ------- |
| +        | (+ x y) |
| -        | (- x y) |
| \*       | (\* x y)|
| /        | (/ x y) |
| %        | (% x y) |
| sin      | (sin x) |
| cos      | (cos x) |
| tan      | (tan x) |
| <        | (< x y) |
| >        | (> x y) |
| <=       | (<= x y)|
| >=       | (>= x y)|

All functions currently take n amount of arguments, e.g. (+ 1 1 1 1 1 ...).


There are also a few predefined constants:

| Constants | Value |
| --------- | ----- |
| pi        | π     |
| two_pi    | π * 2 |
| e         | e     |

Comming soon: def, if, and fn.
