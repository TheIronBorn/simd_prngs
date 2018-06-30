A crate researching various SIMD PRNG speeds.
You need nightly Rust and SIMD capable hardware to use this crate.
To use it, run:
```console
$ RUSTFLAGS='-C target-cpu=native' cargo bench
```
