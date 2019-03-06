# Summarise Generation Log Data

This rough-and-ready script reads a log containing generation number, a space, and a score.
It outputs the mean (and standard error) of the score for each generation.

For example:

```
$ head -n 10  generations-sorted.log
9 235.00000
9 235.00000
9 235.00000
9 235.00000
9 235.00000
18 235.00000
18 235.00000
18 235.00000
18 235.00000
18 235.00000
```

The program will produce:

```
$ head -n 20 generations-sorted.log | target/debug/sumgen
9 235 0
```

Note that:

1. It only outputs a value if the mean/standard error changes.

2. It is very rough and is expected to panic at the end of file:

    thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/libcore/option.rs:345:21
    note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

3. It expects exactly 5 samples of each generation (as shown in the example input). This is a `const` in the main file.


