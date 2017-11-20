# Rvs

[![Build Status](https://travis-ci.org/rfdonnelly/rvs.svg?branch=master)](https://travis-ci.org/rfdonnelly/rvs)

Rvs (pronounced r-v-s) is a C API library for defining and evaluating random
variables using a simple DSL (Domain Specific Language).

## Examples

```C
// An enumeration defintion with implicit value members.
enum Command {
    Read,
    Write,
    Erase,
}

// A variable that yields the repeating pattern: 2, 0, 1, 0
pattern = Pattern(
    Command::Erase,
    Command::Read,
    Command::Write,
    Command::Read,
);

// A variable that yields random values in the set {0, 1, 2}
sample = Sample(Command);

// A variable that yields random values in the range [0, 7] inclusive
range = [0, 7];

// A variable that yields weighted random values `0` 40% of the time, `1` 50%
// of the time, and `2` 10% of the time.
weighted = {
    40: Command::Read,
    50: Command::Write,
    10: Command::Erase,
};
```

## Feature Status

* [ ] Rv features
  * [x] `next()`
  * [x] `prev()`
  * [x] `done()`
  * [ ] `reset()`
  * [x] `Display`

* [x] Parse from string
  * [x] Parse from file
  * [x] Parsing error reporting
  * [x] Overriding existing variable definitions

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
  * Consructs
    * [x] Variables
    * [x] Enums
      * [x] Implicit values E.g. `enum Enum { Value, }`
      * [x] Explicit values E.g. `enum Enum { Value = 0, }`
      * [x] Use of enum members E.g. `enum Enum { Value = 0, } a =
        Enum::Value` expands to `a = 0`
      * [x] Use of enum types E.g. `enum Enum { Value0, Value1, } a =
        Sample(Enum)` expands to `a = Sample(0, 1)`
    * [ ] Structs
  * Types
    * Meta Types
      * [ ] Next - Returns the next value of a variable. Syntax: `<identifier>`
      * [ ] Copy - Returns a copy of a variable. Syntax: `<identifier>.copy`
      * [ ] Prev - Returns the last value of a variable. Syntax:
            `<identifier>.prev`
      * [ ] Done - Forces the sub expression to indicate done on every next.
            Syntax: `<expr>.done`
      * [ ] Once - Forces the sub expression to be evaluated once. Syntax:
            `<expr>.once`
      * [ ] Expand - Returns all evaluations of the expression until done.
            Syntax: `<expr>.expand`
    * Random Types
      * [x] Range - Returns a random value in the range [<lower>, <upper>]
            inclusive.  Syntax: `[<lower>, <upper>]`
      * [x] Sample - Randomly selects then returns a sub-expression.  Syntax:
            `Sample(<expr>, ...)`
      * [ ] SampleNoRepeat - Randomly selects then returns a sub-expression.
            Will not return same sub-expression until all sub-expressions have
            been returned.  Syntax: `SampleNoRepeat(<expr>, ...)`
      * [x] WeightedSample - Randomly selects then returns a sub-expression
            according to weight.  Syntax: `{<weight>: <expr>, ...}`
    * Misc Types
      * [x] Pattern - Returns sub-expressions in order.  Syntax:
            `Pattern(<expr>, ...)`
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
  * [x] Require/Include/Import/Etc
    * [ ] Require is idempotent
    * [ ] Search path - Key value pair E.g. 'key0=/a/b/c:key1=/d/e/f'.
      * [ ] Key relative paths E.g. `::key0::path::file => '/a/b/c/path/file.rvs'`
      * [ ] Precendence path E.g. `path::file` => ['/a/b/c/path/file.rvs', '/d/e/f/path/file.rvs']
    * [ ] Source relative path E.g. a `require fileb` in `filea` becomes `$(dirname filea)/fileb.rvs`
    * [ ] Simplified naming E.g. `path::file` instead of `'path/file.rvs'`
  * [ ] Filename in errors
  * [x] Line numbers in errors

### Extra

* Optimizations
  * [ ] Convert `HashMap<String, Box<Rv>>` to `HashMap<&str, Box<Rv>>`
  * [x] Replace `RangeInclusive` with
    [`rand::distributions::Range::new_inclusive()`](https://github.com/rust-lang-nursery/rand/issues/188)
* [ ] Separate into multiple crates
  * [ ] Rvs Library
  * [ ] DSL (Grammar/Parser, AST)
  * [ ] Interactive binary
  * [ ] C-API
* [ ] Implement the `Iterator` trait
