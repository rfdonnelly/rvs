# Rvs Language Guide

## Defining a Variable

Variables are defined in the form of `variable-name = expression;`.

A variable can be used as static configuration or a dynamic sequence of values.

Let's start with a simple variable that indicates whether something is enabled
or not.  We'll start off with it disabled.

```
enable = 0;
```

We could later decide we want the enable to be random with an equal chance of
being enabled or disabled using the Range type.

```
enable = [0, 1];
```

Maybe we want a higher chance of being enabled vs disabled.  We could make
enabled twice as likely as disabled using the Weighted type.

```
enable = {
    2: 1,
    1: 0,
};
```

## Overriding a Variable

Variables can be defined and later re-defined or overridden.  For example, we
could have verbosity turned off by default but later override it to turn it on.

```
verbosity = 0;
...
verbosity = 1;
```

## Variables as Sequences of Values

Variables yield a sequence of values.  A simple example of this is the Pattern
type.

```
pattern = Pattern(0, 1, 2);
```

The `pattern` variable yields the sequence `0`, `1`, `2` then repeats
indefinitely.

Randomness can be added by using a random expression type.  One example of this
is the Range type.

```
range = [0, 2];
```

Each iteration of the `range` variable will yield a value between `0` and `2`
inclusive.

Variables can be composed of multiple expressions to yield a complex sequence
of values.

```
complex = Pattern([0, 1], [6, 7]);
```

The first iteration of the `complex` variable yields a random value between `0`
and `1`.  The second iteration yields a random value between `6` and `7`.  The
third iteration again yields a random value between `0` and `1`.

## Expression Doneness

Expressions have a concept of doneness.  For expressions that may yield
sub-expressions, doneness is used to determine selection of the next
sub-expression.

When a sub-expression is selected, it will be yielded until it signals done.
After the current sub-expression signals done, the next sub-expression will be
selected and yielded.

The expression `Pattern(Pattern(0, 1), 2)` yields the sequence `0, 1, 2, 0, 1,
2, ...` rather than `0, 2, 1, 2, ...`.  This is because the Pattern
sub-expression is selected and yielded until it signals done.  The next
sub-expression (`2`) is only selected and yielded after the first
sub-expresison (`Pattern(0, 1)`) signals done.

Each expression type handles doneness different.  See the language reference
for details.

## Enumerations

Enumerations can be used to give numeric values symbolic names.

Each iteration of the `command` variable has an equal chance of being a `Write`
(`0`), a `Read` (`1`), or a `Erase` (`2`).

```
enum Command {
    Write = 0,
    Read = 1,
    Erase = 2,
}

command = Sample(Command);
```

If we wanted the `Write` command to have a higher probability of occuring we
could use the Weighted type to give the `Write` more weight than the `Read` or
the `Erase`.

```
enum Command {
    Write = 0,
    Read = 1,
    Erase = 2,
}

command = {
    2: Command::Write,
    1: Command::Read,
    1: Command::Erase,
};
```

## Imports

Other `.rvs` files may be imported using the `import` statement.

The following statement imports the file `some/path/to/file.rvs`.

```
import some::path::to::file;
```

The import path places a few restrictions on the names of paths and files.

* Path names may only include alpha-numeric characters and the `_` character
* Filenames must end with `.rvs`
