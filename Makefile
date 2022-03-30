
build-dev:
	wasm-pack build --dev --target nodejs

build-web:
	wasm-pack build --dev --target web
build-prod:
	wasm-pack build --target nodejs
