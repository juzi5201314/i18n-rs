# i18n-rs

A simple compile time i18n implementation in Rust.

> *This is a personal project.
If you need a stable and powerful i18n library, 
you may need [fluent](https://github.com/projectfluent/fluent-rs).*

> If you think this crate is not easy to use, I found another similar crate: [https://github.com/terry90/internationalization-rs](https://github.com/terry90/internationalization-rs)

### Examples
Look [i18n-example](./examples/i18n-example)

### Strict 
Under normal circumstances, if you try to get a non-existent field or a non-existent locale, the field itself will be returned.

But strict mode will check your input in `lang!` and `i18n!` to make sure that you are using the existing locale and fields that exist in all locales.

If there is an error, it will be `panic!`.

Don't worry, all of this is checked at compile time, 
so strict checking will hardly affect runtime performance,
and there will be not panic at runtime.

> note: Because it needs to be checked at compile time, 
string literals must be used in strict mode

Fortunately, we can freely choose whether to use strict mode.

### Benchmark
```
strict contrast/no strict
                        time:   [29.048 ns 29.387 ns 29.736 ns]
                        change: [-15.897% -13.053% -10.253%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
  
strict contrast/strict  time:   [29.108 ns 29.431 ns 29.776 ns]
                        change: [-2.6412% -0.8426% +1.0984%] (p = 0.38 > 0.05)
                        No change in performance detected.
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

change_lang             time:   [148.38 ns 159.76 ns 178.01 ns]
                        change: [+0.4039% +4.5240% +10.326%] (p = 0.05 > 0.05)
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe
```

---
To be added. . .
