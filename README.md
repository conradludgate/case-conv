# case-conv

Fast and correct case conversion.

Provides fast cases for ascii, but falls back to a unicode correct version if it discovers non ascii chars.

## Benchmark results

> Tests performed with a copy of the Macbeth play text. One with ascii only, the other with
4 wide UTF-8 chars directly at the end of the text.

> Tests performed on a Ryzen 5 3600X on Linux

```text
lowercase/ascii         time:   [8.6785 us 8.6962 us 8.7166 us] *
lowercase/unicode       time:   [8.9324 us 8.9473 us 8.9647 us] *

lowercase/ascii_std     time:   [520.37 us 521.49 us 522.65 us]
lowercase/unicode_std   time:   [519.91 us 520.64 us 521.47 us]
```

```text
uppercase/ascii         time:   [8.7429 us 8.7534 us 8.7635 us] *
uppercase/unicode       time:   [8.7853 us 8.7972 us 8.8102 us] *

uppercase/ascii_std     time:   [439.54 us 440.59 us 441.86 us]
uppercase/unicode_std   time:   [441.31 us 442.10 us 442.94 us]
```

> Tests performed on an Apple M1 Macos Monterey

```text
lowercase/ascii         time:   [4.4029 us 4.4085 us 4.4152 us] *
lowercase/unicode       time:   [4.4530 us 4.4559 us 4.4597 us] *

lowercase/ascii_std     time:   [279.34 us 279.42 us 279.50 us]
lowercase/unicode_std   time:   [283.05 us 283.86 us 284.66 us]
```

```text
uppercase/ascii         time:   [4.3839 us 4.3929 us 4.4033 us] *
uppercase/unicode       time:   [4.4758 us 4.4844 us 4.4921 us] *

uppercase/ascii_std     time:   [246.35 us 247.50 us 248.85 us]
uppercase/unicode_std   time:   [249.75 us 250.72 us 251.78 us]
```

## is_ascii

During the development of this lib, I discovered a way to optimise the `is_ascii` method more.
It uses similar tricks that the std impl does, but doesn't concern itself with awkward edge cases and
instead unrolls some of the tricks up to 16 times (in the end, doing checks on 128 bytes at a time)

> Tests performed with a copy of the Macbeth play text

> Tests performed on a Ryzen 5 3600X on Linux

```text
is_ascii/case_conv      time:   [3.7527 us 3.7584 us 3.7646 us] *
is_ascii/std_lib        time:   [6.8951 us 6.9151 us 6.9367 us]
```

> Tests performed on an Apple M1 Macos Monterey

```text
is_ascii/case_conv      time:   [3.1531 us 3.1571 us 3.1624 us] *
is_ascii/std_lib        time:   [4.3892 us 4.3938 us 4.4004 us]
```
