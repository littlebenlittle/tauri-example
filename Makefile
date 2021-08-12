build:
	wasm-pack build --target web --out-name app

package:
	rsync pkg/app.js public/
	rsync pkg/app_bg.wasm public/
	rsync -r pkg/snippets public/

serve:
	npx tauri dev
