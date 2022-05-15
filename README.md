# case-conv

Fast and correct case conversion.

Provides fast cases for ascii, but falls back to a unicode correct version if it discovers non ascii chars.

There are two functions provided for each conversion variant.
* `to_Xcase()` provides good average speeds whether there is ascii or unicode in the string. This works by assuming ascii, but supports a seamless fallback into the unicode correct version after.
* `to_Xcase2()` provides better than average speeds on ascii only strings, and better than std speeds on unicode containing strings. This works by checking the entire string is ascii upfront and choosing a different path accordingly.

## Benchmark results

Tests performed with a copy of the Macbeth play text. One with ascii only, the other with
4 wide UTF-8 chars directly at the end of the text.

> Tests performed on a Ryzen 5 3600X on Linux

```
lowercase/ascii_std     time:   [522.94 us 523.39 us 523.89 us]
lowercase/unicode_std   time:   [550.58 us 551.49 us 552.45 us]

lowercase/ascii         time:   [109.34 us 109.52 us 109.73 us] *
lowercase/unicode       time:   [108.03 us 108.17 us 108.32 us] *

lowercase/ascii2        time:   [11.597 us 11.624 us 11.652 us] **
lowercase/unicode2      time:   [475.29 us 476.56 us 477.96 us] **
```

```
uppercase/ascii_std     time:   [444.06 us 444.60 us 445.21 us]
uppercase/unicode_std   time:   [466.65 us 467.25 us 467.87 us]

uppercase/ascii         time:   [108.77 us 109.19 us 109.62 us] *
uppercase/unicode       time:   [109.79 us 109.87 us 109.97 us] *

uppercase/ascii2        time:   [11.494 us 11.513 us 11.534 us] **
uppercase/unicode2      time:   [363.06 us 363.65 us 364.32 us] **
```
