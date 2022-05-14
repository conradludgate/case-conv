# case-conv

Fast and correct case conversion.

Provides a fast case for ascii, but falls back to a unicode correct version if it discovers non ascii chars.

## Benchmark results

> Tests performed on a Ryzen 5 3600X on Linux

> `*` means it's from this crate. Lower number is better

```
lowercase/ascii         time:   [947.44 ns 956.89 ns 971.81 ns] *
lowercase/unicode       time:   [1.5051 us 1.5081 us 1.5116 us] *
lowercase/ascii_std     time:   [2.1723 us 2.1797 us 2.1887 us]
lowercase/unicode_std   time:   [2.3111 us 2.3155 us 2.3205 us]

uppercase/ascii         time:   [938.50 ns 944.61 ns 952.93 ns] *
uppercase/unicode       time:   [1.3386 us 1.3436 us 1.3496 us] *
uppercase/ascii_std     time:   [1.8429 us 1.8555 us 1.8687 us]
uppercase/unicode_std   time:   [1.9726 us 1.9761 us 1.9800 us]
```
