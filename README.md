# case-conv

Fast and correct case conversion.

Provides fast cases for ascii, but falls back to a unicode correct version if it discovers non ascii chars.

## Benchmark results

> Tests performed with a copy of the Macbeth play text. One with ascii only, the other with
4 wide UTF-8 chars directly at the end of the text.

> Tests performed on a Ryzen 5 3600X on Linux

```text
lowercase/ascii         time:   [29.912 us 29.957 us 30.009 us]
lowercase/unicode       time:   [29.827 us 29.876 us 29.927 us]

lowercase/ascii_std     time:   [520.37 us 521.49 us 522.65 us]
lowercase/unicode_std   time:   [519.91 us 520.64 us 521.47 us]
```

```
uppercase/ascii         time:   [29.796 us 29.830 us 29.869 us]
uppercase/unicode       time:   [30.025 us 30.092 us 30.164 us]

uppercase/ascii_std     time:   [439.54 us 440.59 us 441.86 us]
uppercase/unicode_std   time:   [441.31 us 442.10 us 442.94 us]
```

> Tests performed on an Apple M1 Macos Monterey

```
lowercase/ascii         time:   [2.4684 us 2.4740 us 2.4851 us] *
lowercase/unicode       time:   [2.5334 us 2.5357 us 2.5388 us] *

lowercase/ascii_std     time:   [279.69 us 280.16 us 280.83 us]
lowercase/unicode_std   time:   [279.70 us 280.13 us 280.60 us]
```

```
uppercase/ascii         time:   [2.5087 us 2.5117 us 2.5147 us] *
uppercase/unicode       time:   [2.5746 us 2.5754 us 2.5764 us] *

uppercase/ascii_std     time:   [247.11 us 247.81 us 248.51 us]
uppercase/unicode_std   time:   [245.50 us 245.93 us 246.46 us]
```
