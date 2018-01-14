# Rvs Language Reference

The purpose of the Rvs language is to provide a lightweight method of defining
random variables.

## Variable Definitions

Variables are declared and defined using an assignment statement.  The last
definition of a variable takes precedence.  Variables are assigned an
expression.

### Examples

Define the variable `a`:

* Statement: `a = 5;`
* Result: `5 <done>, 5 <done>, ...`

Re-define the variable `b`:

* Statement: `b = 1; b = 2;`
* Result: `2 <done>, 2 <done>, ...`

### Grammar

```
variable = identifier "=" expr ";"
```

## Expressions

```
expr = atom | binary_expr
binary_expr = atom operator atom
operator = ("|" | "^" | "&" | "<<" | ">>" | "+" | "-" | "*" | "/" | "%")
atom = "(" expr ")"
     | unary_expr
     | number
     | type
     | r_identifier
unary_expr = "~" atom
           | "-" atom
```

### Values

Values are the most basic Rvs expression.  Values are 32 bit unsigned integers.
The value expression yields the underlying 32 bit unsigned integer.

Syntax:

* `<decimal_value>`
* `0x<hexidecimal_value>`

#### Doneness

The Value type indicates done after every iteration.

#### Examples

A decimal value:

* Expression: `5`
* Result: `5 <done>, 5 <done>, ...`

A hexidecimal value:

* Expression: `0xa`
* Result: `10 <done>, 10 <done>, ...`

A decimal value with separator:

* Expression: `1_000`
* Result: `1000 <done>, 1000 <done>, ...`

A hexidecimal value with separator:

* Expression: `0x1_0000`
* Result: `65536 <done>, 65536 <done>, ...`

### Operators

#### Binary Operators

| Precendence | Operators   |
| ----------- | ----------- |
| 1 (highest) | `*` `/` `%` |
| 2           | `+` `-`     |
| 3           | `<<` `>>`   |
| 4           | `&`         |
| 5           | `^`         |
| 6 (lowest)  | <code>&#124;</code> |

#### Unary Operators

* `-` - Two's complement
* `~` - Bitwise invert

## Types

### Range

The Range type yields a random value between limit `a` and limit `b` inclusive.

Syntax: `[a, b]`

Both `a` and `b` are expressions.  They are evaluated once on transform.

#### Doneness

The Range type indicates done after every iteration.

#### Examples

A range from low to high.

* Expression: `[0, 3]`
* Possible result: `2 <done>, 1 <done>, 2 <done>, 0 <done>, ...`

A range from high to low.

* Expression: `[3, 0]`
* Equivalent expression: `[0, 3]`

A range with equal limits.

* Expression: `[5, 5]`
* Equivalent expression: `5`

#### Grammar

```
range = "[" expr "," expr "]"
```

### Sequence

The Sequence type yields an linear sequence of values.

Syntax:

* `Sequence(<last>)`
* `Sequence(<first>, <last>, [<increment>])`

Arguments:

* `last` - The ending value.
* `first` - The starting value.  Defaults to `0`.
* `increment` - The value to increment by.  Defaults to `1`.

All arguments are expressions.  The arguments are evaluated at the beginning of
each cycle.

#### Doneness

The Sequence type indicates done when either of the following are true:

* The yielded value is equal to `last`
* The yielded value + the `increment` passes `last`

#### Examples

Sequence with a `last` only.

* Expression: `Sequence(3)`
* Result: `0, 1, 2, 3 <done>, 0, 1, ...`

Sequence with `first` and `last`.

* Expression: `Sequence(1, 4)`
* Result: `1, 2, 3, 4 <done>, 1, 2, ...`

Sequence with `first`, `last`, and `increment`.

* Expression: `Sequence(1, 7, 2)`
* Result: `1, 3, 5, 7 <done>, 1, 3, ...`

Sequence with a negative `increment`.

* Expression: `Sequence(3, 0, -1)`
* Result: `3, 2, 1, 0 <done>, 3, 2, ...`

Sequence with complex sub-expressions.

* Expression: `Sequence(Pattern(2, 4), Pattern(4, 16), Pattern(2, 4))`
* Result: `2, 4 <done>, 4, 8, 12, 16 <done>, 2, 4 <done>, 4, ...`

Sequence where `increment` passes `last`:

* Expression: `Sequence(0, 3, 2)`
* Result: `0, 2 <done>, 0, 2 <done>, ...`

Sequence where negative `increment` passes `last`:

* Expression: `Sequence(13, 10, -2)`
* Result: `13, 11 <done>, 13, 11 <done>, ...`

#### Grammar

```
sequence = "Sequence" "(" sequence_args optional_comma ")"
sequence_args = expr ("," expr){0, 2}
```

### Pattern

The Pattern type sequentially yields each sub-expression and waits for
the current sub-expression to signal done before yielding the next
sub-expression.

Syntax: `Pattern(expr_0, ..., expr_n)`

#### Doneness

The Pattern type indicates done after the last sub-expression indicates done.

#### Examples

Pattern with sub-expression that always indicate done.

* Expression: `Pattern(0, 1, 2, 3)`
* Result: `0, 1, 2, 3 <done>, 0, 1, ...`

Pattern with sub-expression that yield multiple values before indicating done.

* Expression: `Pattern(3, 2, 1, Sequence(4))`
* Result: `3, 2, 1, 0, 1, 2, 3 <done>, 3, 2, 1, 0, ...`

#### Grammar

```
pattern = "Pattern" "(" pattern_args optional_comma ")"
pattern_args = expr ("," expr)*
```

### Weighted

The Weighted type yields sub-expressions in a weighted random fashion waiting
for the current sub-expression to indicate done before selecting the next
sub-expression.  The probability that a given sub-expression will be selected
is the weight of the sub-expression divided by the combined weight of all
sub-expressions.

Syntax: `{weight_0: expr_0, ..., weight_n: expr_n}`

#### Doneness

The Weighted type indicates done when the current sub-expression indicates done.

#### Examples

A weighted expression that yields `0` 25% of the time and yields `1` 75% of the
time.

* Expression: `{1: 0, 3: 1}`
* Possible result: `1 <done>, 0 <done>, 1 <done>, 1 <done>, ...`

A weighted expression with a sub-expression that yields multiple values before
indicating done.

* Expression: `{2: Pattern(4, 5, 6), 1: 0}`
* Possible result: `4, 5, 6 <done>, 0 <done>, 4, 5, 6 <done>, 4, ...`

A weighted expression with equal weights.

* Expression: `{50: 0, 50: 1}`
* Equivalent expression: `Sample(0, 1)`

A weighted expression with a single sub-expression.

* Expression: `{100: 1}`
* Equivalent expression: `1`

#### Grammar

```
weighted = "{" weighted_pairs optional_comma "}"
weighted_pairs = weighted_pair ("," weighted_pair)*
weighted_pair = dec_number ":" expr
```

### Sample

The Sample type selects a sub-expression at random and continues to yield the
selected sub-expression until the sub-expression indicates done.  This type
implements random sampling with replacement.

Syntax: `Sample(expr_0, ..., expr_n)`

#### Doneness

The Sample type indicates done when the selected sub-expression indicates done.

#### Examples

Sample two Values:

* Expression: `Sample(0, 1)`
* Possible results
  * `0 <done>, 0 <done>, ...`
  * `0 <done>, 1 <done>, ...`
  * `1 <done>, 0 <done>, ...`
  * `1 <done>, 1 <done>, ...`

Sample a sub-expression with multiple values per cycle:

* Expression: `Sample(Pattern(0, 1), Pattern(2, 3))`
* Possible results:
  * `0, 1 <done>, 0, 1 <done>, ...`
  * `0, 1 <done>, 2, 3 <done>, ...`
  * `2, 3 <done>, 0, 1 <done>, ...`
  * `2, 3 <done>, 2, 3 <done>, ...`

Sample multiple values:

* Expression: `Sample(0, 1, 2)`
* Equivalent expression: `{1: 0, 1: 1, 1: 2}`

Sample an enum:

* Enum: `enum State { Off, On }`
* Expression: `Sample(State)`
* Equivalent expressions:
  * `Sample(State::Off, State::On)`
  * `Sample(0, 1)`

#### Grammar

```
sample = "Sample" "(" sample_args optional_comma ")"
sample_args = sample_arg ("," sample_arg)*
sample_arg = expand | expr
```

### Unique

The Unique type selects a sub-expression at random and continues to yield the
selected sub-expression until the sub-expression indicates done.  The selected
sub-expression is removed from the set of possible sub-expressions to choose
from until all sub-expressions have been yielded exactly once.  This type
implements random sampling without replacement.

#### Doneness

The Unique type indicates done when the last selected sub-expression indicates
done.

#### Examples

Uniquely sample two values:

* Expression: `Unique(0, 1)`
* Possible results
  * `0, 1 <done>, 1, 0 <done>, ...`
  * `0, 1 <done>, 0, 1 <done>, ...`
  * `1, 0 <done>, 1, 0 <done>, ...`
  * `1, 0 <done>, 0, 1 <done>, ...`

#### Grammar

```
unique = "Unique" "(" sample_args optional_comma ")"
```

### Expand

The Expand type is a special type that is only valid as an argument to the
Sample and Unique types.  It allows the use of an expression to populate the
Sample and Unique types.  It repeatedly evaluates the sub-expression until done
and adds the yielded values to the containing Sample or Unique type.

Syntax:

* `Expand(expr)` - Evaluates `expr` until it indicates done
* `Expand(expr, count_expr)` - Evaluates `expr` `count_expr` times

#### Doneness

Doneness does not apply to the Expand special type.

#### Examples

* Expression: `Sample(Expand(Sequence(3)))`
* Equivalent expression: `Sample(0, 1, 2, 3)`

#### Grammar

```
expand = "Expand" "(" expand_args optional_comma ")"
expand_args = expr ("," expr){0, 1}
```

### Done

The Done type is a modifier type.  It yields the underlying expression but
always indicates done.

#### Doneness

Always indicates done after the first evaluation.

#### Examples

* Expression: `Done(Patten(0, 1, 2, 3))`
* Result: `0 <done>, 1 <done>, 2 <done>, 3 <done>, 0 <done, ...`

* Expression: `Pattern(Done(Pattern(0, 1, 2)), 5)`
* Result: `0, 5 <done>, 1, 5 <done>, 2, 5 <done>, 0, 5 <done>, ...`

#### Grammar

```
done = "Done" "(" expr ")"
```

### Once

The Once type is a modifier type.  It evaluates the underlying expression once
then always yields the result.

#### Doneness

Always indicates done after the first evaluation.

#### Grammar

```
once = "Once" "(" expr ")"
```

## Variables in Expressions

Variables may be used in expessions.  Variables may be used directly or by
calling their `prev` or `copy` methods.

Using a variable directly (without method call) will result in the variables
state being advanced.  Using the `copy` method creates a copy of the variable's
expression and does not modify the variable's state.

The `prev` method yields the variables most recently yielded value.  It does
not modify the variable's state.

### Examples



```
r_identifier = identifier variable_method_call?

variable_method_call = "." variable_method
variable_method = "prev" | "copy"
```

TODO

## Enums

### Examples

An enum with implicit member values.

```
enum Direction {
    Up,     // 0
    Down,   // 1
    Left,   // 2
    Right,  // 3
}
```

An enum with explicit member values.

```
enum State {
    On = 1,     // 1
    Off = 0,    // 0
}
```

An enum with a mix of implicit and explicit member values.

```
enum Access {
    Read,       // 0
    Write = 2,  // 2
    Erase,      // 3
}
```

### Grammar

```
enum = "enum" type_name "{" enum_members optional_comma "}"
enum_members = enum_member ("," enum_member)*
enum_member = type_name explicit_value?
explicit_value = "=" number
```

## Appendix: Grammar

### Grammar Syntax

* `*` - match 0 or more of the preceding
* `+` - match 1 or more of the preceding
* `{n, m}` - match n to m of the preceding
* `?` - preceding is optional
* `a | b` - match a else match b

### Common Definitions

```
identifier = [a-zA-Z_] [a-zA-Z0-9_:]*
type_name = [A-Z] [a-zA-Z0-9]*
optional_comma = ","?
```

### Literals

```
number = hex_number | dec_number
hex_number = "0" [xX] hex_digit (hex_digit | "_")*
hex_digit = [0-9a-fA-F]
dec_number = dec_number (dec_number | "_")*
```

### Types

```
type = range | sequence | pattern | weighted | sample | unique | done | once
```

### Top Level Definitions

```
items = item+
item = import | enum | variable
```

### Imports

```
import = "import" import_path ";"
import_path = [_:0-9a-zA-Z]+
```
