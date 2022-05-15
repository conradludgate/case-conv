# case-conv

Fast and correct case conversion.

Provides fast cases for ascii, but falls back to a unicode correct version if it discovers non ascii chars.

## Benchmark results

> Tests performed with a copy of the Macbeth play text. One with ascii only, the other with
4 wide UTF-8 chars directly at the end of the text.

> Tests performed on a Ryzen 5 3600X on Linux

```text
lowercase/ascii         time:   [29.912 us 29.957 us 30.009 us] *
lowercase/unicode       time:   [29.827 us 29.876 us 29.927 us] *

lowercase/ascii_std     time:   [520.37 us 521.49 us 522.65 us]
lowercase/unicode_std   time:   [519.91 us 520.64 us 521.47 us]
```

```text
uppercase/ascii         time:   [29.796 us 29.830 us 29.869 us] *
uppercase/unicode       time:   [30.025 us 30.092 us 30.164 us] *

uppercase/ascii_std     time:   [439.54 us 440.59 us 441.86 us]
uppercase/unicode_std   time:   [441.31 us 442.10 us 442.94 us]
```

> Tests performed on an Apple M1 Macos Monterey

```text
lowercase/ascii         time:   [18.687 us 18.696 us 18.706 us] *
lowercase/unicode       time:   [18.740 us 18.754 us 18.769 us] *

lowercase/ascii_std     time:   [279.34 us 279.42 us 279.50 us]
lowercase/unicode_std   time:   [283.05 us 283.86 us 284.66 us]
```

```text
uppercase/ascii         time:   [18.692 us 18.701 us 18.713 us] *
uppercase/unicode       time:   [18.875 us 19.006 us 19.210 us] *

uppercase/ascii_std     time:   [246.35 us 247.50 us 248.85 us]
uppercase/unicode_std   time:   [249.75 us 250.72 us 251.78 us]
```
