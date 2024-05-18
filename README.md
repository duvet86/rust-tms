## Setup

```sh
cargo install cargo-watch systemfd
```

## Running

```sh
systemfd --no-pid -s http::3000 -- cargo watch -x run
```