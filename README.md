# Rvs

[![Build Status](https://travis-ci.org/rfdonnelly/rvs.svg?branch=master)](https://travis-ci.org/rfdonnelly/rvs)

Rvs (pronounced r-v-s) is a C API library for defining and evaluating random
variables using a simple DSL (Domain Specific Language).

## Rvs Syntax

The syntax for the Rvs DSL is a series of C-like assignment statements.  These
statements both declare and define a variable.

```C
<indentifier> = <expression>;
```

### Examples

#### Constant Value Variable

The following declares the variable `a` and defines it to the value of `0`

```C
a = 0;
```

Evaluations of `a` yields the following:

| Iteration |  0  |  1  |  2  |  3  |  4  |  5  | ... |
| --------- | --- | --- | --- | --- | --- | --- | --- |
| `next()`  |  0  |  0  |  0  |  0  |  0  |  0  | ... |
| `prev()`  |  0  |  0  |  0  |  0  |  0  |  0  | ... |
| `done()`  |  0  |  1  |  1  |  1  |  1  |  1  | ... |

NOTE: Calling `next()` advances the variable to the next iteration.  Calling
`prev()` or `done()` does not.

#### Random Range Variable

The following declares the variable `b` and defines it to the range `[0,
1]`.

```C
b = [0, 1];
```

A possible series of evaluations of `b` could yield the following:

| Iteration |  0  |  1  |  2  |  3  |  4  |  5  | ... |
| --------- | --- | --- | --- | --- | --- | --- | --- |
| `next()`  |  1  |  0  |  0  |  1  |  1  |  0  | ... |
| `prev()`  |  0  |  1  |  0  |  0  |  1  |  1  | ... |
| `done()`  |  0  |  0  |  0  |  0  |  0  |  0  | ... |

## Feature Status

* [ ] Rv features
  * [x] `next()`
  * [x] `prev()`
  * [x] `done()`
  * [ ] `reset()`
  * [ ] `Display`

* [x] Parse from string
  * [x] Parse from file
  * [x] Parsing error reporting
  * [ ] Overriding existing variable definitions

* C API
  * [x] `rvs_context_new()`
  * [x] `rvs_context_free()`
  * [x] `rvs_seed()`
  * [x] `rvs_parse()`
  * [x] `rvs_find()`
  * [x] `rvs_next()`
  * [x] `rvs_done()`
  * [x] `rvs_prev()`
  * [ ] `rvs_reset()`
  * [ ] `rvs_write_definitions()`

* Grammar
  * Types
    * Meta Types
      * [ ] Next(id) - Returns the next value of a variable
      * [ ] Copy(id) - Returns a copy of a variable
      * [ ] Last(id) - Returns the last value of a variable
      * [ ] Done(expr) - Forces the sub expression to indicate done on every next
      * [ ] Once(expr) - Forces the sub expression to be evaluated once
      * [ ] Populate(expr) - Returns all evaluations of the expression until done
    * Random Types
      * [x] Range
      * [ ] Sample
      * [ ] SampleNoRepeat
      * [ ] WeightedRandom
    * Misc Types
      * [ ] Pattern
      * [ ] Loop
    * Arithmetic operators
      * [x] +, -
      * [x] *, /
      * [x] %
    * Logic operators
      * [x] &, |, ^
      * [x] <<, >>
  * [x] Whitespace
  * [x] Comments
  * [ ] Modules
  * [ ] Filename in errors
  * [x] Line numbers in errors

### Extra

* Optimizations
  * [ ] Convert `HashMap<String, Box<Rv>>` to `HashMap<&str, Box<Rv>>`
  * [ ] Replace `RangeInclusive` with
    [`rand::distributions::Range::new_inclusive()`](https://github.com/rust-lang-nursery/rand/issues/188)
* [ ] Separate into multiple crates
  * [ ] Rvs Library
  * [ ] DSL (Grammar/Parser, AST)
  * [ ] Interactive binary
  * [ ] C-API
* [ ] Implement the `Iterator` trait
