
build-dev:
	RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals'; wasm-pack build --dev --target nodejs -d pkg/node
build-web:
	RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals' \
  	cargo build --target wasm32-unknown-unknown -Z build-std=std,panic_abort
	yarn webpack
build-prod:
	wasm-pack build --target nodejs 
