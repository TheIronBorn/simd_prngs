RUSTFLAGS='-C target-feature=mmx -C codegen-units=1 -C lto=thin' cargo bench > benches/benchmarks/mmx &&
RUSTFLAGS='-C target-cpu=nehalem -C codegen-units=1 -C lto=thin' cargo bench > benches/benchmarks/nehalem &&
RUSTFLAGS='-C target-cpu=native -C codegen-units=1 -C lto=thin' cargo bench > benches/benchmarks/native_sandybridge
