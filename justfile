default:
    cargo run --release

check:
    cargo check
        
bench:
    cargo build --release
    hyperfine ./target/release/ray_tracing

open:
    kitten icat './out/img.ppm'
