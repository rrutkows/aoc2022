https://adventofcode.com/2022

Assumming `DD` is the day number, paste your input into `dDD/input.rs` or `dDD/input.txt` and run:
```
cargo run --release --bin dDD
```

Starting from d14 benchmarks are available:
```
cargo bench -- dDD
```

I have no generic solution for d22. Your cube must have the following layout
```
 ##
 #
##
#
```
where `#` is a 50x50 fragment
