# case-conv

Fast and correct case conversion.

Provides fast cases for ascii, but falls back to a unicode correct version if it discovers non ascii chars.

## Benchmark results

> Tests performed with a copy of the Macbeth play text. One with ascii only, the other with
4 wide UTF-8 chars directly at the end of the text.

> Tests performed on a Ryzen 5 3600X on Linux

```text
lowercase/ascii         time:   [5.4363 us 5.4486 us 5.4629 us] *
lowercase/unicode       time:   [5.5051 us 5.5135 us 5.5229 us] *

lowercase/ascii_std     time:   [522.00 us 522.97 us 524.00 us]
lowercase/unicode_std   time:   [520.60 us 521.19 us 521.83 us]
```

```text
uppercase/ascii         time:   [5.3779 us 5.3837 us 5.3903 us] *
uppercase/unicode       time:   [5.3809 us 5.3930 us 5.4073 us] *

uppercase/ascii_std     time:   [441.45 us 442.01 us 442.64 us]
uppercase/unicode_std   time:   [437.97 us 438.51 us 439.11 us]
```
