# i18n-rs

![Crates.io](https://img.shields.io/crates/d/simple-i18n?style=flat-square)
![Lines](https://img.shields.io/tokei/lines/github/juzi5201314/i18n-rs?style=flat-square)
![Crates.io](https://img.shields.io/crates/l/simple-i18n?style=flat-square)
![docs.rs](https://docs.rs/simple-i18n/badge.svg)

A simple compile time i18n implementation in Rust.

> *This is a personal project.
If you need a stable and powerful i18n library, 
you may need [fluent](https://github.com/projectfluent/fluent-rs).*

> If you think this crate is not easy to use, I found another similar crate: [https://github.com/terry90/internationalization-rs](https://github.com/terry90/internationalization-rs)

### Use
In crates.io, the name of this package is `simple-i18n`, because the name of `i18n-rs` is occupied by an empty crate. shit...

Add `simple-i18n = "0.1"` to Cargo.toml

### Examples
Look [i18n-example](./examples/i18n-example)
```shell
cd examples/i18n-example
LOCALE_PATH=locale cargo run --package i18n-example --bin i18n-example
```

### [Docs](https://docs.rs/simple-i18n)

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
