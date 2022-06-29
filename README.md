# Bevy Pong

A bevy bevy pong game.

[Demo](https://tong-k-k.github.io/my-pong/index.html)

to build web:
```
cargo run --target wasm32-unknown-unknown
```

to release web:
```
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/
```

Windows:
```
wasm-bindgen .\target\wasm32-unknown-unknown\release\my-pong.wasm --out-dir .\out\ --target web
```