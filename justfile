default:
    cargo run > './out/img.ppm'

bench:
    cargo build --release
    hyperfine './target/release/ray_tracing > ./out/img.ppm' --warmup 10 --runs 30
