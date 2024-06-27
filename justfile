default:
    cargo run --release

bench:
    cargo build --release
    hyperfine ./target/release/ray_tracing

open:
    kitten icat './out/img.ppm'
