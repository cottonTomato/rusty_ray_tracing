default:
    cargo run > './out/img.ppm'

bench:
    cargo build --release
    hyperfine './target/release/ray_tracing > ./out/img.ppm'

open:
    kitten icat './out/img.ppm'
