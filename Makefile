build:
	cd frontend; wasm-pack build --target web

rollup:
	npx rollup frontend/pkg/frontend.js --format iife --file public/bundle.js

serve:
	npx tauri dev
