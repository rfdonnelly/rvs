# Sequences

[![Build Status](https://travis-ci.org/rfdonnelly/sequence-rs.svg?branch=master)](https://travis-ci.org/rfdonnelly/sequence-rs)

A library for generating sequences of values using a DSL (Domain Specific
Language).

## Syntax

The syntax for the Sequence DSL is a series of C-like assignment
statements.  These statements both declare and initialize a sequence.

```C
<indentifier> = <expression>;
```

### Examples

#### Constant Value Sequence

The following declares the sequence `a` and initializes it to the value of
`0`

```C
a = 0;
```

Evaluations of `a` yields the following:

| Iteration |  0  |  1  |  2  |  3  |  4  |  5  | ... |
| --------- | --- | --- | --- | --- | --- | --- | --- |
| `next()`  |  0  |  0  |  0  |  0  |  0  |  0  | ... |
| `last()`  |  0  |  0  |  0  |  0  |  0  |  0  | ... |
| `done()`  |  0  |  1  |  1  |  1  |  1  |  1  | ... |

NOTE: Calling `next()` advances the sequence to the next iteration.  Calling
`last()` or `done()` does not.

#### Random Range Sequence

The following declares the sequence `b` and intializes it to the range
`[0, 1]`.

```C
b = [0, 1];
```

A possible series of evaluations of `b` could yield the following:

| Iteration |  0  |  1  |  2  |  3  |  4  |  5  | ... |
| --------- | --- | --- | --- | --- | --- | --- | --- |
| `next()`  |  1  |  0  |  0  |  1  |  1  |  0  | ... |
| `last()`  |  0  |  1  |  0  |  0  |  1  |  1  | ... |
| `done()`  |  0  |  0  |  0  |  0  |  0  |  0  | ... |

## Feature Status

* [ ] Sequence features
  * [x] `next()`
  * [x] `last()`
  * [x] `done()`

* [x] Parse from string
  * [ ] Parse from file
  * [ ] Parse from command line

* C API
  * [x] static HashMap of sequences
  * [x] `parse()`
  * [x] `lookup()`
  * [x] `next()`
  * [ ] `done()`
  * [ ] `exists()`
  * [ ] `last()`

* Grammar
  * Types
    * Meta Sequences
      * [ ] Next(id) - Returns the next value of a sequence
      * [ ] Copy(id) - Returns a copy of a sequence
      * [ ] Last(id) - Returns the last value of a sequence
      * [ ] Done(expr) - Forces the sub sequence to indicate done on every next
      * [ ] Once(expr) - Forces the sub sequence to be evaluated once
      * [ ] Populate(expr) - Returns all evaluations of the expression until done
    * Random Sequences
      * [x] Range
      * [ ] Sample
      * [ ] SampleNoRepeat
      * [ ] WeightedRandom
    * Misc Sequences
      * [ ] Pattern
      * [ ] Loop
    * Arithmetic operators
      * [x] +, -
      * [x] *, /
      * [ ] %
    * Logic operators
      * [ ] &, |, ^
      * [ ] <<, >>
  * [ ] Whitespace
  * [ ] Comments
  * [ ] Modules

### Extra

* Optimizations
  * [ ] Convert `HashMap<String, Box<Sequence>>` to `HashMap<&str, Box<Sequence>>`
  * [ ] Replace `RangeInclusive` with
    [`rand::distributions::Range::new_inclusive()`](https://github.com/rust-lang-nursery/rand/issues/188)
* [ ] Separate into multiple crates
  * [ ] Sequence Library
  * [ ] DSL (Grammar/Parser, AST)
  * [ ] Interactive binary
  * [ ] C-API
* [ ] Implement the `Iterator` trait
