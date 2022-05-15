# case-conv

Fast and correct case conversion.

Provides fast cases for ascii, but falls back to a unicode correct version if it discovers non ascii chars.

## Benchmark results

> Tests performed with a copy of the Macbeth play text. One with ascii only, the other with
4 wide UTF-8 chars directly at the end of the text.

> Tests performed on a Ryzen 5 3600X on Linux

```text
lowercase/ascii         time:   [8.1783 us 8.3009 us 8.4188 us] *
lowercase/unicode       time:   [8.3850 us 8.4944 us 8.5908 us] *

lowercase/ascii_std     time:   [522.00 us 522.97 us 524.00 us]
lowercase/unicode_std   time:   [520.60 us 521.19 us 521.83 us]
```

```text
uppercase/ascii         time:   [5.3919 us 5.4112 us 5.4305 us] *
uppercase/unicode       time:   [5.4151 us 5.4259 us 5.4371 us] *

uppercase/ascii_std     time:   [446.56 us 447.98 us 449.36 us]
uppercase/unicode_std   time:   [466.68 us 467.22 us 467.85 us]
```
