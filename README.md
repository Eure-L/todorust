# Todo webapp
## Setup
### Rust Installation

```shell
sudo apt install rustc
cargo install cargo-watch
```

### Tailwind Install 
```shell
sudo apt install npm
npm install -D tailwindcss@3.1.8
npx tailwindcss init
```

### Development Run configurations

Runs Rust Axum Server with automatic reloading upon changes in wathched directories.
```shell
RUST_LOG=DEBUG cargo watch -x run -w templates -w src -w static
```
```shell
npm run dev
```

### Production Run configurations


```shell
RUST_LOG=TRACE cargo run
```
```shell
npm run prod
```




